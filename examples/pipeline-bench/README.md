## Pipeline bench example

This example sets up a synthetic high-throughput pipeline using `carbon-core` to
help measure bottlenecks before any real decoder or application logic runs.

It uses:

- a synthetic datasource that pushes account or transaction updates as fast as possible (or at a
  configured rate),
- trivial synthetic decoders and processors that only receive updates (no-op),
- existing metrics backends (`LogMetrics` and optionally `PrometheusMetrics`).

### Running

From the repository root:

```bash
cargo run -p carbon-pipeline-bench-example -- \
  --processing-mode instruction \
  --updates-per-producer 3000000 \
  --channel-buffer-size 1000000 \
  --use-prometheus
```

Key CLI flags:

- `--processing-mode`: choose between "account" or "instruction" processing (default: "instruction").
- `--updates-per-producer`: optional cap on how many updates the synthetic
  datasource sends. If omitted, it runs until you terminate the process.
- `--updates-per-second`: optional number of synthetic updates per second; if
  set, the datasource sleeps between sends to approximate this rate.
- `--channel-buffer-size`: size of the `tokio::mpsc` channel buffer used by the
  pipeline.
- `--use-prometheus`: enable the Prometheus metrics backend in addition to
  `LogMetrics`.
- `--prometheus-port`: port for the Prometheus exporter (default: 9100).

When built with the `channel-experiments` feature, an additional flag is
available:

- `--direct-channel-benchmark`: run a direct `tokio::mpsc` + no-op processor
  benchmark that bypasses the `carbon-core` pipeline entirely, useful as a
  baseline for channel overhead without any pipeline bookkeeping.

This example is intended to reflect the current `carbon-core` behavior and is
meant for measurement only; it does not change the behavior of the core
pipeline.


