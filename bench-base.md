# Pipeline Performance Baseline

## Configuration

- **Binary**: `carbon-pipeline-bench-example`
- **Mode**: Transaction processing (`--processing-mode transaction`)
- **Metrics**: `LogMetrics` + `PrometheusMetrics`
- **Channel buffer**: `--channel-buffer-size 1_000_000`
- **Producer**: 1 synthetic datasource, 3M transactions, max speed (no rate limit)
- **Pipeline**: `ShutdownStrategy::Immediate`, `metrics_flush_interval(1)`

## Results (3M transactions)

- **Total processed**: 3,000,000 transactions
- **Duration**: ~89 seconds
- **Throughput**: ~33,700 tx/sec

## Time Breakdown (per transaction)

| Metric | Avg (ns) | Avg (µs) | % of Total |
|--------|----------|----------|------------|
| **Total process time** | 10,246 | 10.25 | 100% |
| **Transaction processing** | 7,106 | 7.11 | 69.4% |
| **Metrics recording** | 3,890 | 3.89 | 38.0% |
| **Recv wait** | 2,482 | 2.48 | 24.2% |

### Transaction Processing Breakdown

| Component | Avg (ns) | Avg (µs) | % of Tx Time |
|-----------|----------|----------|--------------|
| Transaction metadata build | 722 | 0.72 | 10.1% |
| Instruction extraction | 195 | 0.19 | 2.7% |
| Transaction filter eval | 32 | 0.03 | 0.4% |
| Transaction pipe time | 368 | 0.37 | 5.2% |
| **Unaccounted** | **~5,800** | **~5.80** | **81.6%** |

## Calculations

- **Metrics overhead**: 3.89 µs / 10.25 µs = **38% of total process time**
- **Transaction processing**: 7.11 µs / 10.25 µs = **69% of total process time**
- **Unaccounted transaction time**: 7.11 µs - (0.72 + 0.19 + 0.03 + 0.37) µs = **~5.80 µs** (81.6% of transaction processing)

## Conclusions

1. **Metrics recording is a significant bottleneck**: ~38% of total process time is spent recording metrics. This includes histogram updates, counter increments, and gauge updates.

2. **Most transaction processing time is unaccounted**: Only ~18% of transaction processing time is explicitly measured (metadata build, instruction extraction, filter eval, pipe time). The remaining ~82% likely includes:
   - Metrics recording overhead within transaction processing path
   - Loop overhead for iterating instruction pipes (even when empty)
   - Arc cloning overhead for transaction metadata
   - Other pipeline bookkeeping

3. **Channel overhead is minimal**: Recv wait time (~2.48 µs) is relatively small compared to processing time (~10.25 µs), indicating the channel itself is not the primary bottleneck.

## Direct Channel Baseline (for comparison)

- **Config**: Direct `tokio::mpsc` + no-op processor, bypasses pipeline
- **Result**: ~405,000 updates/sec
- **Pipeline overhead**: ~33,700 tx/sec vs ~405,000 updates/sec = **~12× slower** than raw channel throughput
