use {crate::error::CarbonResult, std::future::Future};

pub trait Processor<T>
where
    T: Sync,
{
    fn process(&mut self, data: &T) -> impl Future<Output = CarbonResult<()>> + Send;
}
