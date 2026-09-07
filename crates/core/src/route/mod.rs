//! Route inputs and context.

mod account;
mod instruction;
mod transaction;

pub use {
    account::AccountProcessorInput,
    instruction::InstructionProcessorInput,
    transaction::{TransactionFilterInput, TransactionProcessorInput},
};

use crate::id::Id;

/// Identifies the pipeline, datasource, and route running a callback.
#[derive(Clone, Copy, Debug)]
pub struct RouteContext<'a> {
    pipeline_id: &'a Id,
    datasource_id: &'a Id,
    route_id: &'a Id,
}

impl<'a> RouteContext<'a> {
    pub const fn new(pipeline_id: &'a Id, datasource_id: &'a Id, route_id: &'a Id) -> Self {
        Self {
            pipeline_id,
            datasource_id,
            route_id,
        }
    }

    pub const fn pipeline_id(&self) -> &'a Id {
        self.pipeline_id
    }

    pub const fn datasource_id(&self) -> &'a Id {
        self.datasource_id
    }

    pub const fn route_id(&self) -> &'a Id {
        self.route_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_borrows_the_supplied_ids() {
        let pipeline_id = Id::new("indexer").unwrap();
        let datasource_id = Id::new("yellowstone").unwrap();
        let route_id = Id::new("transfers").unwrap();
        let context = RouteContext::new(&pipeline_id, &datasource_id, &route_id);

        assert!(std::ptr::eq(context.pipeline_id(), &pipeline_id));
        assert!(std::ptr::eq(context.datasource_id(), &datasource_id));
        assert!(std::ptr::eq(context.route_id(), &route_id));
    }
}
