use carbon_core::{
    error::CarbonResult,
    metrics::{MetricsExporter, MetricsRegistry, MetricsSnapshot},
};

pub struct PrometheusMetricsExporter;

impl PrometheusMetricsExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn prometheus_text() -> String {
        let snapshot = MetricsRegistry::global().snapshot();
        format_prometheus_text(&snapshot)
    }
}

impl Default for PrometheusMetricsExporter {
    fn default() -> Self {
        Self::new()
    }
}

fn format_prometheus_text(snapshot: &MetricsSnapshot) -> String {
    let mut output = String::new();

    for (name, help, value) in &snapshot.counters {
        if !help.is_empty() {
            output.push_str(&format!("# HELP {} {}\n", name, help));
        }
        output.push_str(&format!("# TYPE {} counter\n", name));
        output.push_str(&format!("{} {}\n", name, value));
    }

    for (name, help, value) in &snapshot.gauges {
        if !help.is_empty() {
            output.push_str(&format!("# HELP {} {}\n", name, help));
        }
        output.push_str(&format!("# TYPE {} gauge\n", name));
        output.push_str(&format!("{} {}\n", name, value));
    }

    for (name, help, hist) in &snapshot.histograms {
        if !help.is_empty() {
            output.push_str(&format!("# HELP {} {}\n", name, help));
        }
        output.push_str(&format!("# TYPE {} histogram\n", name));

        let mut cumulative_count = 0u64;
        for (i, boundary) in hist.boundaries.iter().enumerate() {
            cumulative_count += hist.counts.get(i).copied().unwrap_or(0);
            output.push_str(&format!(
                "{}_bucket{{le=\"{}\"}} {}\n",
                name, boundary, cumulative_count
            ));
        }

        let total_count: u64 = hist.counts.iter().sum();
        cumulative_count += hist.counts.get(hist.boundaries.len()).copied().unwrap_or(0);
        output.push_str(&format!(
            "{}_bucket{{le=\"+Inf\"}} {}\n",
            name, cumulative_count
        ));
        output.push_str(&format!("{}_count {}\n", name, total_count));

        let sum: f64 = hist
            .boundaries
            .iter()
            .enumerate()
            .map(|(i, boundary)| {
                let count = hist.counts.get(i).copied().unwrap_or(0);
                let midpoint = if i == 0 {
                    *boundary / 2.0
                } else {
                    (hist.boundaries[i - 1] + *boundary) / 2.0
                };
                count as f64 * midpoint
            })
            .sum();

        let overflow_count = hist.counts.get(hist.boundaries.len()).copied().unwrap_or(0);
        let overflow_sum = if let Some(last_boundary) = hist.boundaries.last() {
            overflow_count as f64 * *last_boundary * 1.5
        } else {
            0.0
        };

        output.push_str(&format!("{}_sum {:.0}\n", name, sum + overflow_sum));
    }

    output
}

impl MetricsExporter for PrometheusMetricsExporter {
    fn export(&self, _snapshot: &MetricsSnapshot) -> CarbonResult<()> {
        Ok(())
    }
}
