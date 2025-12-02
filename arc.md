# Pipeline Performance Baseline - Complex Instructions (With Arc)

## Configuration

- **Binary**: `carbon-pipeline-bench-example`
- **Mode**: Instruction processing (`--processing-mode instruction`)
- **Metrics**: `LogMetrics` + `PrometheusMetrics`
- **Channel buffer**: `--channel-buffer-size 1_000_000`
- **Producer**: 1 synthetic datasource, 1M transactions, max speed (no rate limit)
- **Pipeline**: `ShutdownStrategy::Immediate`, `metrics_flush_interval(1)`
- **Transaction Complexity**: 
  - 5-10 instructions per transaction
  - 5-10 accounts per instruction
  - 10-50 bytes of data per instruction
- **Code State**: All expensive types wrapped in `Arc` (no clones)

## Results (1M transactions)

- **Total processed**: 1,000,000 transactions
- **Duration**: ~90 seconds (~1:30)
- **Throughput**: ~11,111 tx/sec

## Time Breakdown (per transaction)

| Metric | Avg (ns) | Avg (µs) | % of Total |
|--------|----------|----------|------------|
| **Total process time** | 72,203 | 72.20 | 100% |
| **Transaction processing** | 66,802 | 66.80 | 92.5% |
| **Recv wait** | 3,918 | 3.92 | 5.4% |

### Transaction Processing Breakdown

| Component | Avg (ns) | Avg (µs) | % of Tx Time |
|-----------|----------|----------|--------------|
| Transaction metadata build | 1,796 | 1.80 | 2.7% |
| Instruction extraction | 5,552 | 5.55 | 8.3% |
| **Unaccounted** | **~59,454** | **~59.45** | **89.0%** |

### Instruction Processing Breakdown (per instruction)

| Component | Avg (ns) | Avg (µs) | Notes |
|-----------|----------|----------|-------|
| Instruction decode | 778 | 0.78 | Per instruction |
| Instruction filter eval | 29 | 0.03 | Per instruction |
| Instruction pipe time | 4,670 | 4.67 | Per instruction (includes decode + process) |
| Instruction process | 908 | 0.91 | Per instruction |

**Note**: With 5-10 instructions per transaction, instruction processing overhead scales linearly.

## Calculations

- **Transaction processing**: 66.80 µs / 72.20 µs = **92.5% of total process time**
- **Recv wait**: 3.92 µs / 72.20 µs = **5.4% of total process time**
- **Unaccounted transaction time**: 66.80 µs - (1.80 + 5.55) µs = **~59.45 µs** (89.0% of transaction processing)
- **Per-instruction overhead**: ~4.67 µs per instruction × ~7.5 avg instructions = **~35.0 µs** (52.4% of transaction processing)

## Comparison: Arc vs No-Arc

Comparing Arc implementation to `no-arc.md` baseline:

| Metric | No-Arc | Arc | Change | % Change |
|--------|--------|-----|--------|----------|
| **Throughput** | ~10,309 tx/sec | ~11,111 tx/sec | **+802 tx/sec** | **+7.8%** |
| **Duration** | ~97s | ~90s | **-7s** | **-7.2%** |
| **Total process time** | 70.84 µs | 72.20 µs | **+1.36 µs** | **+1.9%** |
| **Transaction processing** | 65.05 µs | 66.80 µs | **+1.75 µs** | **+2.7%** |
| **Recv wait** | 2.36 µs | 3.92 µs | **+1.56 µs** | **+66.1%** |
| **Metadata build** | 1.88 µs | 1.80 µs | **-0.08 µs** | **-4.3%** |
| **Instruction extraction** | 5.67 µs | 5.55 µs | **-0.12 µs** | **-2.1%** |
| **Instruction decode** | 0.78 µs | 0.78 µs | **0.00 µs** | **0.0%** |
| **Instruction process** | 0.60 µs | 0.91 µs | **+0.31 µs** | **+51.7%** |
| **Instruction pipe time** | 4.37 µs | 4.67 µs | **+0.30 µs** | **+6.9%** |
| **Unaccounted tx time** | 57.50 µs | 59.45 µs | **+1.95 µs** | **+3.4%** |
| **Queue length** | ~5,691 | ~2,002 | **-3,689** | **-64.8%** |

## Observations

1. **Throughput improvement**: Arc implementation shows **+7.8% throughput improvement** (~802 more tx/sec), reducing total duration by 7 seconds.

2. **Process time slightly increased**: Total process time increased by ~1.36 µs (+1.9%), which is within measurement variance. The improvement comes from reduced queue buildup rather than faster individual processing.

3. **Queue reduction**: Average queue length dropped from ~5,691 to ~2,002 (**-64.8%**), indicating better producer-consumer balance. This suggests Arc reduces contention and allows the pipeline to keep up better.

4. **Recv wait increased**: Recv wait increased from 2.36 µs to 3.92 µs (+66.1%), but this is still only 5.4% of total process time. The increase may be due to better queue utilization (less time spent processing, more time waiting for new items when queue is empty).

5. **Instruction processing overhead**: Instruction process time increased from 0.60 µs to 0.91 µs (+51.7%), likely due to Arc dereferencing overhead. However, this is offset by the overall throughput improvement.

6. **Unaccounted time**: Unaccounted transaction time increased slightly from 57.50 µs to 59.45 µs (+3.4%), which may include Arc cloning overhead (Arc::new() calls) and dereferencing costs.

7. **Overall verdict**: **Arc provides a net benefit** - while individual operation times may increase slightly, the overall throughput improves by ~7.8% due to reduced queue buildup and better pipeline balance.

## Direct Channel Baseline (for comparison)

- **Config**: Direct `tokio::mpsc` + no-op processor, bypasses pipeline
- **Result**: ~405,770 updates/sec
- **Pipeline overhead (No-Arc)**: ~10,309 tx/sec vs ~405,770 updates/sec = **~39.4× slower**
- **Pipeline overhead (Arc)**: ~11,111 tx/sec vs ~405,770 updates/sec = **~36.5× slower**

**Note**: Arc reduces pipeline overhead from 39.4× to 36.5× slower than raw channel throughput, a **7.4% improvement** in relative performance.

