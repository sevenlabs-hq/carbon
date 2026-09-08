//! Block route execution.

use {
    super::{ErrorPolicy, RouteContext, RouteOptions},
    crate::{
        error::{CarbonResult, Error},
        processor::Processor,
        update::BlockUpdate,
    },
    std::{future::Future, pin::Pin},
};

pub(crate) struct BlockRoute<P> {
    processor: P,
    options: RouteOptions<BlockUpdate>,
}

impl<P> BlockRoute<P> {
    pub(crate) fn new(processor: P, options: RouteOptions<BlockUpdate>) -> Self {
        Self { processor, options }
    }
}

impl<P: Processor<BlockUpdate>> BlockRoute<P> {
    async fn run(&mut self, context: &RouteContext<'_>, update: &BlockUpdate) -> CarbonResult<()> {
        if !self
            .options
            .filters
            .filter(context, update)
            .await
            .map_err(Error::Filter)?
        {
            return Ok(());
        }

        let result = self.processor.process(context, update).await;
        if let Err(error) = &result {
            if self.options.processor_error_policy == ErrorPolicy::Exit {
                return result.map_err(Error::Processor);
            }
            log::error!(
                "block processing failed in pipeline {}, datasource {}, route {}: {error}",
                context.pipeline_id(),
                context.datasource_id(),
                context.route_id(),
            );
        }

        self.options
            .filters
            .commit(context, update, &result)
            .await
            .map_err(Error::FilterCommit)
    }
}

pub(crate) trait DynBlockRoute: Send {
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        update: &'a BlockUpdate,
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>>;
}

impl<P: Processor<BlockUpdate>> DynBlockRoute for BlockRoute<P> {
    fn run<'a>(
        &'a mut self,
        context: &'a RouteContext<'_>,
        update: &'a BlockUpdate,
    ) -> Pin<Box<dyn Future<Output = CarbonResult<()>> + Send + 'a>> {
        Box::pin(BlockRoute::run(self, context, update))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{error::BoxError, filter::Filter, id::Id, processor::ProcessorResult},
        std::{
            io,
            sync::{Arc, Mutex},
        },
    };

    #[derive(Clone)]
    struct Observer {
        calls: Arc<Mutex<Vec<&'static str>>>,
        filter_result: Result<bool, &'static str>,
        processor_result: Result<(), &'static str>,
        commit_result: Result<(), &'static str>,
    }

    impl Filter<BlockUpdate> for Observer {
        async fn filter(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &BlockUpdate,
        ) -> Result<bool, BoxError> {
            self.calls.lock().unwrap().push("filter");
            self.filter_result
                .map_err(|message| io::Error::other(message).into())
        }

        async fn commit(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &BlockUpdate,
            result: &ProcessorResult,
        ) -> Result<(), BoxError> {
            assert_eq!(
                result.as_ref().err().map(ToString::to_string),
                self.processor_result.err().map(str::to_owned),
            );
            self.calls.lock().unwrap().push("commit");
            self.commit_result
                .map_err(|message| io::Error::other(message).into())
        }
    }

    impl Processor<BlockUpdate> for Observer {
        async fn process(
            &mut self,
            _context: &RouteContext<'_>,
            _update: &BlockUpdate,
        ) -> ProcessorResult {
            self.calls.lock().unwrap().push("process");
            self.processor_result
                .map_err(|message| io::Error::other(message).into())
        }
    }

    #[tokio::test]
    async fn filters_and_processor_policy_control_commits() {
        let ids = ["pipeline", "source", "route"].map(|id| Id::new(id).unwrap());
        let context = RouteContext::new(&ids[0], &ids[1], &ids[2]);
        let update = BlockUpdate::new(1);

        for policy in [ErrorPolicy::Exit, ErrorPolicy::Continue] {
            for (filter_result, processor_result, commit_result) in [
                (Ok(true), Ok(()), Ok(())),
                (Ok(false), Ok(()), Ok(())),
                (Err("filter"), Ok(()), Ok(())),
                (Ok(true), Err("processor"), Ok(())),
                (Ok(true), Ok(()), Err("commit")),
                (Ok(true), Err("processor"), Err("commit")),
            ] {
                let calls = Arc::new(Mutex::new(Vec::new()));
                let observer = Observer {
                    calls: calls.clone(),
                    filter_result,
                    processor_result,
                    commit_result,
                };
                let options = RouteOptions::default()
                    .filter(observer.clone())
                    .filter(observer.clone())
                    .processor_error_policy(policy);
                let mut route = BlockRoute::new(observer, options);
                let result = route.run(&context, &update).await;

                if filter_result != Ok(true) {
                    assert_eq!(*calls.lock().unwrap(), ["filter"]);
                    if filter_result.is_err() {
                        assert!(matches!(result, Err(Error::Filter(_))));
                    } else {
                        result.unwrap();
                    }
                    continue;
                }
                if processor_result.is_err() && policy == ErrorPolicy::Exit {
                    assert_eq!(*calls.lock().unwrap(), ["filter", "filter", "process"]);
                    assert!(matches!(result, Err(Error::Processor(_))));
                    continue;
                }
                assert_eq!(
                    *calls.lock().unwrap(),
                    ["filter", "filter", "process", "commit", "commit"]
                );
                if commit_result.is_err() {
                    assert!(matches!(result, Err(Error::FilterCommit(_))));
                } else {
                    result.unwrap();
                }
            }
        }
    }
}
