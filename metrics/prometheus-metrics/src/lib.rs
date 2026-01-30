use carbon_core::{
    error::CarbonResult,
    metrics::{MetricsExporter, MetricsRegistry, MetricsSnapshot},
};

pub struct PrometheusMetricsExporter;

impl Default for PrometheusMetricsExporter {
    fn default() -> Self {
        Self
    }
}

impl PrometheusMetricsExporter {
    pub fn new() -> Self {
        Self
    }

    /// Returns the current metrics snapshot formatted in Prometheus exposition text format.
    pub fn prometheus_text() -> String {
        let snapshot = MetricsRegistry::global().snapshot();
        format_prometheus_text(&snapshot)
    }
}

impl MetricsExporter for PrometheusMetricsExporter {
    fn export(&self, _snapshot: &MetricsSnapshot) -> CarbonResult<()> {
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
    }

    out
}

fn sanitize_name(name: &str) -> String {
    name.replace('-', "_")
}
