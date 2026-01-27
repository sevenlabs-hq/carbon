use {crate::error::CarbonResult, async_trait::async_trait, std::sync::Arc};

#[async_trait]
pub trait Metrics: Send + Sync {
    async fn initialize(&self) -> CarbonResult<()>;
    async fn flush(&self) -> CarbonResult<()>;
    async fn shutdown(&self) -> CarbonResult<()>;

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()>;

    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()>;

    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()>;
}

#[derive(Default)]
pub struct MetricsCollection {
    pub metrics: Vec<Arc<dyn Metrics>>,
}

impl MetricsCollection {
    pub fn new(metrics: Vec<Arc<dyn Metrics>>) -> Self {
        Self { metrics }
    }

    pub async fn initialize_metrics(&self) -> CarbonResult<()> {
        for metric in &self.metrics {
            metric.initialize().await?;
        }
        Ok(())
    }

    pub async fn shutdown_metrics(&self) -> CarbonResult<()> {
        for metric in &self.metrics {
            metric.shutdown().await?;
        }
        Ok(())
    }

    pub async fn flush_metrics(&self) -> CarbonResult<()> {
        for metric in &self.metrics {
            metric.flush().await?;
        }
        Ok(())
    }

    pub async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()> {
        for metric in &self.metrics {
            metric.update_gauge(name, value).await?;
        }
        Ok(())
    }

    pub async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()> {
        for metric in &self.metrics {
            metric.increment_counter(name, value).await?;
        }
        Ok(())
    }

    pub async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()> {
        for metric in &self.metrics {
            metric.record_histogram(name, value).await?;
        }
        Ok(())
    }
}
