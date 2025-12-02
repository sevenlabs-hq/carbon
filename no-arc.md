# Pipeline Performance Baseline - Complex Instructions (No Arc)

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
- **Code State**: Same as `bench-base-clone-removal.md` (clone optimizations applied, but NO Arc changes)

## Results (1M transactions)

- **Total processed**: 1,000,000 transactions
- **Duration**: ~97 seconds (~1:37)
- **Throughput**: ~10,309 tx/sec

## Time Breakdown (per transaction)

| Metric | Avg (ns) | Avg (µs) | % of Total |
|--------|----------|----------|------------|
| **Total process time** | 70,836 | 70.84 | 100% |
| **Transaction processing** | 65,050 | 65.05 | 91.8% |
| **Recv wait** | 2,362 | 2.36 | 3.3% |

### Transaction Processing Breakdown

| Component | Avg (ns) | Avg (µs) | % of Tx Time |
|-----------|----------|----------|--------------|
| Transaction metadata build | 1,881 | 1.88 | 2.9% |
| Instruction extraction | 5,674 | 5.67 | 8.7% |
| **Unaccounted** | **~57,495** | **~57.50** | **88.4%** |

### Instruction Processing Breakdown (per instruction)

| Component | Avg (ns) | Avg (µs) | Notes |
|-----------|----------|----------|-------|
| Instruction decode | 779 | 0.78 | Per instruction |
| Instruction filter eval | 31 | 0.03 | Per instruction |
| Instruction pipe time | 4,372 | 4.37 | Per instruction (includes decode + process) |
| Instruction process | 601 | 0.60 | Per instruction |

**Note**: With 5-10 instructions per transaction, instruction processing overhead scales linearly.

## Calculations

- **Transaction processing**: 65.05 µs / 70.84 µs = **91.8% of total process time**
- **Recv wait**: 2.36 µs / 70.84 µs = **3.3% of total process time**
- **Unaccounted transaction time**: 65.05 µs - (1.88 + 5.67) µs = **~57.50 µs** (88.4% of transaction processing)
- **Per-instruction overhead**: ~4.37 µs per instruction × ~7.5 avg instructions = **~32.8 µs** (50.4% of transaction processing)

## Comparison to Simple Instructions Baseline

Comparing to `bench-base-clone-removal.md` (simple transactions with no instructions):

| Metric | Simple (no ixs) | Complex (5-10 ixs) | Change |
|--------|-----------------|-------------------|--------|
| **Throughput** | ~41,667 tx/sec | ~10,309 tx/sec | **-75%** (4× slower) |
| **Process time** | ~7.05 µs | ~70.84 µs | **+905%** (10× slower) |
| **Transaction processing** | ~3.91 µs | ~65.05 µs | **+1,563%** (16.6× slower) |
| **Metadata build** | ~0.43 µs | ~1.88 µs | **+337%** (4.4× slower) |
| **Instruction extraction** | ~0.21 µs | ~5.67 µs | **+2,600%** (27× slower) |

## Observations

1. **Instruction processing dominates**: With 5-10 instructions per transaction, processing time increases dramatically (~10× slower overall).

2. **Instruction extraction overhead**: The `transformers_extract_instructions_time_nanoseconds` metric shows ~5.67 µs per transaction, which is significant. This includes:
   - Parsing compiled instructions
   - Building instruction metadata
   - Creating nested instruction structures

3. **Per-instruction overhead**: Each instruction adds ~4.37 µs of pipe processing time (decode + filter + process). With ~7.5 instructions per transaction on average, this accounts for ~32.8 µs (50% of transaction processing time).

4. **Queue buildup**: Average queue length is ~5,691, indicating the producer is faster than the consumer when processing complex transactions.

5. **Most time still unaccounted**: Even with instruction metrics, ~57.5 µs (88.4%) of transaction processing time is unaccounted for. This likely includes:
   - Loop overhead for iterating through instructions
   - Arc cloning overhead (TransactionMetadata is Arc)
   - Nested instruction structure creation/cloning
   - Other pipeline bookkeeping

6. **Recv wait is low**: At ~2.36 µs (3.3% of total), recv wait is minimal, indicating the bottleneck is processing, not channel receive.

## Direct Channel Baseline (for comparison)

- **Config**: Direct `tokio::mpsc` + no-op processor, bypasses pipeline
- **Result**: ~405,770 updates/sec
- **Pipeline overhead**: ~10,309 tx/sec vs ~405,770 updates/sec = **~39.4× slower** than raw channel throughput

**Note**: This is significantly worse than the simple instruction case (~9.7× slower), indicating that instruction processing adds substantial overhead beyond just the channel.

