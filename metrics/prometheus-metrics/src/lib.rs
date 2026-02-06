use carbon_core::{
    error::CarbonResult,
    metrics::{MetricsExporter, MetricsRegistry, MetricsSnapshot},
};

#[cfg(feature = "http-server")]
#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    pub listen_addr: std::net::SocketAddr,
    pub enable_cors: bool,
    pub request_timeout_secs: u64,
}

#[cfg(feature = "http-server")]
impl PrometheusConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn listen_addr(mut self, addr: std::net::SocketAddr) -> Self {
        self.listen_addr = addr;
        self
    }

    pub fn enable_cors(mut self, enable: bool) -> Self {
        self.enable_cors = enable;
        self
    }

    pub fn request_timeout_secs(mut self, timeout: u64) -> Self {
        self.request_timeout_secs = timeout;
        self
    }
}

#[cfg(feature = "http-server")]
impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:9464".parse().unwrap(),
            enable_cors: true,
            request_timeout_secs: 30,
        }
    }
}

#[cfg(not(feature = "http-server"))]
#[derive(Debug, Clone)]
struct PrometheusConfig;

pub struct PrometheusMetrics {
    config: Option<PrometheusConfig>,
    auto_start_server: bool,
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl PrometheusMetrics {
    pub fn new() -> Self {
        Self {
            config: None,
            auto_start_server: false,
        }
    }

    #[cfg(feature = "http-server")]
    pub fn with_server(config: PrometheusConfig) -> Self {
        Self {
            config: Some(config),
            auto_start_server: true,
        }
    }

    #[cfg(feature = "http-server")]
    pub fn start_server(
        config: PrometheusConfig,
    ) -> tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        tokio::spawn(async move { run_metrics_server_with_config(config).await })
    }

    pub fn prometheus_text() -> String {
        let snapshot = MetricsRegistry::global().snapshot();
        Self::format_snapshot(&snapshot)
    }

    pub fn format_snapshot(snapshot: &MetricsSnapshot) -> String {
        format_prometheus_text(snapshot)
    }
}

impl MetricsExporter for PrometheusMetrics {
    fn export(&self, _snapshot: &MetricsSnapshot) -> CarbonResult<()> {
        Ok(())
    }

    #[cfg(feature = "http-server")]
    fn initialize(&self) -> CarbonResult<()> {
        if self.auto_start_server {
            if let Some(config) = &self.config {
                let config = config.clone();
                tokio::spawn(async move {
                    if let Err(e) = run_metrics_server_with_config(config).await {
                        log::error!("Prometheus metrics server error: {}", e);
                    }
                });
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "http-server"))]
    fn initialize(&self) -> CarbonResult<()> {
        Ok(())
    }
}

fn format_prometheus_text(snapshot: &MetricsSnapshot) -> String {
    let mut out = String::new();

    for (name, help, value) in &snapshot.counters {
        let base = sanitize_name(name);
        if !help.is_empty() {
            out.push_str(&format!("# HELP {base} {help}\n"));
        }
        out.push_str(&format!("# TYPE {base} counter\n"));
        out.push_str(&format!("{base} {value}\n"));
    }

    for (name, help, value) in &snapshot.gauges {
        let base = sanitize_name(name);
        if !help.is_empty() {
            out.push_str(&format!("# HELP {base} {help}\n"));
        }
        out.push_str(&format!("# TYPE {base} gauge\n"));
        out.push_str(&format!("{base} {value}\n"));
    }

    for (name, help, hist) in &snapshot.histograms {
        let base = sanitize_name(name);
        if !help.is_empty() {
            out.push_str(&format!("# HELP {base} {help}\n"));
        }
        out.push_str(&format!("# TYPE {base} histogram\n"));

        let mut cumulative: u64 = 0;
        for (i, &boundary) in hist.boundaries.iter().enumerate() {
            cumulative += hist.counts.get(i).copied().unwrap_or(0);
            out.push_str(&format!(
                "{base}_bucket{{le=\"{boundary}\"}} {cumulative}\n"
            ));
        }
        let total: u64 = hist.counts.iter().sum();
        cumulative = total;
        out.push_str(&format!("{base}_bucket{{le=\"+Inf\"}} {cumulative}\n"));
        out.push_str(&format!("{base}_count {total}\n"));
        out.push_str(&format!("{base}_sum {}\n", hist.sum));
    }

    out
}

fn sanitize_name(name: &str) -> String {
    name.replace('-', "_")
}

#[cfg(feature = "http-server")]
pub async fn run_metrics_server(
    listen_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_metrics_server_with_config(PrometheusConfig {
        listen_addr,
        ..Default::default()
    })
    .await
}

#[cfg(feature = "http-server")]
pub async fn run_default_metrics_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_metrics_server_with_config(PrometheusConfig::default()).await
}

#[cfg(feature = "http-server")]
pub async fn run_metrics_server_with_config(
    config: PrometheusConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use axum::{
        body::Body,
        http::{Response, StatusCode},
        routing::get,
        Router,
    };
    use tokio::net::TcpListener;
    use tower::ServiceBuilder;
    use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

    async fn metrics_handler() -> Result<Response<Body>, StatusCode> {
        let text = PrometheusMetrics::prometheus_text();

        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8; version=0.0.4")
            .header("cache-control", "no-cache, no-store, must-revalidate")
            .body(Body::from(text))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    let mut app =
        Router::new()
            .route("/metrics", get(metrics_handler))
            .layer(ServiceBuilder::new().layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                std::time::Duration::from_secs(config.request_timeout_secs),
            )));

    if config.enable_cors {
        app = app.layer(CorsLayer::permissive());
    }

    let listener = TcpListener::bind(config.listen_addr).await?;
    log::info!(
        "Prometheus metrics server starting on http://{}/metrics",
        config.listen_addr
    );

    let server = axum::serve(listener, app);

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                log::error!("Metrics server error: {e}");
            }
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("Received shutdown signal, stopping metrics server");
        }
    }

    Ok(())
}
