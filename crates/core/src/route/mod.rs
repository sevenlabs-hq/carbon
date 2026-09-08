//! Routes, processor inputs, and context.

mod account;
mod account_closure;
mod block;
mod instruction;
mod transaction;

pub use {
    account::AccountProcessorInput, instruction::InstructionProcessorInput,
    transaction::TransactionProcessorInput,
};

pub(crate) use {
    account::{AccountRoute, DynAccountRoute},
    account_closure::{AccountClosureRoute, DynAccountClosureRoute},
    block::{BlockRoute, DynBlockRoute},
    instruction::{DynInstructionRoute, InstructionRoute},
    transaction::{DynTransactionRoute, TransactionRoute},
};

use crate::{
    filter::{Filter, Filters},
    id::Id,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorPolicy {
    Exit,
    Continue,
}

pub struct RouteOptions<T> {
    filters: Filters<T>,
    processor_error_policy: ErrorPolicy,
}

impl<T> Default for RouteOptions<T> {
    fn default() -> Self {
        Self {
            filters: Filters::default(),
            processor_error_policy: ErrorPolicy::Exit,
        }
    }
}

impl<T: Sync + 'static> RouteOptions<T> {
    pub fn filter(mut self, filter: impl Filter<T> + 'static) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn processor_error_policy(mut self, policy: ErrorPolicy) -> Self {
        self.processor_error_policy = policy;
        self
    }
}

pub struct DecodedRouteOptions<T> {
    filters: Filters<T>,
    decode_error_policy: ErrorPolicy,
    processor_error_policy: ErrorPolicy,
}

impl<T> Default for DecodedRouteOptions<T> {
    fn default() -> Self {
        Self {
            filters: Filters::default(),
            decode_error_policy: ErrorPolicy::Continue,
            processor_error_policy: ErrorPolicy::Exit,
        }
    }
}

impl<T: Sync + 'static> DecodedRouteOptions<T> {
    pub fn filter(mut self, filter: impl Filter<T> + 'static) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn decode_error_policy(mut self, policy: ErrorPolicy) -> Self {
        self.decode_error_policy = policy;
        self
    }

    pub fn processor_error_policy(mut self, policy: ErrorPolicy) -> Self {
        self.processor_error_policy = policy;
        self
    }
}

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
