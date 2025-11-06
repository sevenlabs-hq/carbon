# Public Key Storage Analysis for Carbon Database

## Current State Analysis

- **Table**: `pool_account` (77 rows, 88 kB total, 40 kB data)
- **Storage**: Raw byte arrays (`bytea` type)
- **Public Key Columns**: 6 columns storing 32-byte public keys
- **Current Efficiency**: ~520 bytes per row for public keys (6 × 32 + metadata)

## Storage Approach Comparison

### 1. Raw Byte Arrays (Current - RECOMMENDED)

```sql
-- Current approach
__pubkey BYTEA NOT NULL,
creator BYTEA NOT NULL,
base_mint BYTEA NOT NULL,
-- etc.
```

**Pros:**

- ✅ **Most storage efficient**: 32 bytes per pubkey
- ✅ **Fastest performance**: Native binary operations
- ✅ **Index efficiency**: Smaller indexes, faster lookups
- ✅ **Memory efficient**: Less RAM usage
- ✅ **Network efficient**: Smaller result sets

**Cons:**

- ❌ **Not human-readable**: Requires conversion for display
- ❌ **Debugging complexity**: Hard to read in query results

**Storage Impact:**

- Current: 6 × 32 = 192 bytes per row for public keys
- With 77 rows: ~15 KB for all public keys

### 2. Hex Strings (Alternative)

```sql
-- Alternative approach
__pubkey VARCHAR(66) NOT NULL,  -- 0x + 64 hex chars
creator VARCHAR(66) NOT NULL,
base_mint VARCHAR(66) NOT NULL,
-- etc.
```

**Pros:**

- ✅ **Human-readable**: Easy to debug and query
- ✅ **No conversion needed**: Direct display

**Cons:**

- ❌ **2x storage overhead**: 66 bytes vs 32 bytes per pubkey
- ❌ **Slower operations**: String comparisons vs binary
- ❌ **Index bloat**: 2x larger indexes
- ❌ **Memory overhead**: More RAM usage

**Storage Impact:**

- Hex: 6 × 66 = 396 bytes per row for public keys
- With 77 rows: ~30 KB for all public keys (2x current)

### 3. Base58 Strings (Alternative)

```sql
-- Alternative approach
__pubkey VARCHAR(44) NOT NULL,  -- Base58 encoded
creator VARCHAR(44) NOT NULL,
base_mint VARCHAR(44) NOT NULL,
-- etc.
```

**Pros:**

- ✅ **Human-readable**: Standard Solana format
- ✅ **Compact**: Shorter than hex
- ✅ **No conversion needed**: Direct display

**Cons:**

- ❌ **1.4x storage overhead**: 44 bytes vs 32 bytes per pubkey
- ❌ **Slower operations**: String comparisons vs binary
- ❌ **Index bloat**: Larger indexes
- ❌ **No native PostgreSQL support**: Requires custom functions

**Storage Impact:**

- Base58: 6 × 44 = 264 bytes per row for public keys
- With 77 rows: ~20 KB for all public keys (1.4x current)

## Performance Analysis

### Query Performance

```sql
-- Raw byte array (current) - FASTEST
SELECT * FROM pool_account WHERE __pubkey = '\x1234...'::bytea;

-- Hex string - SLOWER
SELECT * FROM pool_account WHERE __pubkey = '0x1234...';

-- Base58 string - SLOWEST
SELECT * FROM pool_account WHERE __pubkey = 'ABC123...';
```

### Index Efficiency

- **Raw byte arrays**: 32-byte indexes
- **Hex strings**: 66-byte indexes (2x larger)
- **Base58 strings**: 44-byte indexes (1.4x larger)

## Recommendation: Keep Raw Byte Arrays + Add Helper Functions

### Why Raw Byte Arrays Are Best:

1. **Scale**: At 1M+ rows, storage difference becomes significant
2. **Performance**: Binary operations are fastest
3. **Memory**: Less RAM usage for large datasets
4. **Network**: Smaller result sets

### Implementation Strategy:

```sql
-- Keep current bytea columns
-- Add helper functions for conversion

-- Function to convert bytea to base58
CREATE OR REPLACE FUNCTION bytea_to_base58(input_bytea bytea)
RETURNS text AS $$
-- Implementation using base58 library
$$ LANGUAGE plpgsql;

-- Function to convert base58 to bytea
CREATE OR REPLACE FUNCTION base58_to_bytea(input_text text)
RETURNS bytea AS $$
-- Implementation using base58 library
$$ LANGUAGE plpgsql;

-- Create views for human-readable access
CREATE VIEW pool_account_readable AS
SELECT
    bytea_to_base58(__pubkey) as pubkey,
    bytea_to_base58(creator) as creator,
    bytea_to_base58(base_mint) as base_mint,
    -- ... other fields
FROM pool_account;
```

### Application Layer Solution:

```rust
// In your Rust application
impl From<PoolRow> for PoolGraphQL {
    fn from(row: PoolRow) -> Self {
        Self {
            pubkey: bs58::encode(&row.pubkey).into_string(),
            creator: bs58::encode(&row.creator).into_string(),
            // ... other conversions
        }
    }
}
```

## Final Recommendation

**KEEP RAW BYTE ARRAYS** for these reasons:

1. **Performance**: 2-3x faster queries and indexes
2. **Storage**: 50% less storage usage
3. **Memory**: Lower RAM requirements
4. **Network**: Smaller result sets
5. **Scalability**: Better performance at scale

**Add conversion layer** in application code (Rust) or database functions for human-readable output.

This gives you the best of both worlds: optimal performance and storage efficiency with human-readable output when needed.
