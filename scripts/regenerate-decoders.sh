#!/bin/bash
set -e

# Regenerate all decoders using carbon-cli in parallel
# RPC URL can be provided as first argument, defaults to mainnet-beta

RPC_URL="${1:-https://rpc.solami.dev/sol?api_key=6FUhgbE92ZBW9Wjk}"
PARALLEL_JOBS="${2:-8}"  # Number of parallel jobs
IDL_CACHE_DIR=".idl-cache"  # Local IDL cache directory

echo "🔄 Regenerating all decoders using RPC: $RPC_URL"
echo "⚡ Running $PARALLEL_JOBS jobs in parallel"
echo "📁 IDL cache directory: $IDL_CACHE_DIR"
echo ""

# Create IDL cache directory if it doesn't exist
mkdir -p "$IDL_CACHE_DIR"

# Discover decoder_name:program_id pairs directly from each decoder's lib.rs.
# This keeps the regeneration list in sync with the source tree and avoids
# duplicating program IDs in bash.
DECODERS=()
while IFS= read -r decoder_entry; do
    DECODERS+=("$decoder_entry")
done < <(python3 - <<'PY'
from pathlib import Path
import re
for lib in sorted(Path('decoders').glob('*-decoder/src/lib.rs')):
    text = lib.read_text()
    match = re.search(r'from_str_const\("([^"]+)"\)', text)
    if not match:
        continue
    decoder_name = lib.parent.parent.name
    print(f'{decoder_name}:{match.group(1)}')
PY
)

TOTAL=${#DECODERS[@]}

echo "Total decoders to regenerate: $TOTAL"
echo ""

# Build the CLI first
echo "📦 Building carbon-cli..."
cd packages/cli
pnpm build > /dev/null 2>&1
cd ../..
echo ""

# Create temp directory for tracking
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Function to fetch and cache IDL
fetch_and_cache_idl() {
    local program_id=$1
    local cache_file="$IDL_CACHE_DIR/${program_id}.json"
    
    if [ -f "$cache_file" ]; then
        echo "$cache_file"
        return 0
    fi
    
    # Fetch IDL using the helper script
    node scripts/fetch-idl.mjs "$program_id" "$RPC_URL" "$cache_file" 2>/dev/null
}

# Function to regenerate a single decoder
regenerate_decoder() {
    local entry=$1
    local index=$2
    local total=$3
    
    local decoder_name program_id
    IFS=':' read -r decoder_name program_id <<< "$entry"
    
    # Extract name without -decoder suffix for --name option
    local decoder_base_name="${decoder_name%-decoder}"
    
    local log_file="$TEMP_DIR/${decoder_name}.log"
    local status_file="$TEMP_DIR/${decoder_name}.status"
    local idl_cache_file="$IDL_CACHE_DIR/${program_id}.json"
    
    echo "[$index/$total] 🔨 Regenerating $decoder_name..."
    
    # Check if IDL is cached, otherwise fetch it. If no IDL can be found,
    # fall back to generating ClickHouse source directly from the pre-generated
    # Rust decoder modules.
    generate_from_source=false
    if [ ! -f "$idl_cache_file" ]; then
        echo "[$index/$total] 💾 Fetching and caching IDL for $program_id..."
        idl_file=$(fetch_and_cache_idl "$program_id")
        if [ $? -ne 0 ] || [ -z "$idl_file" ] || [ ! -f "$idl_file" ]; then
            generate_from_source=true
        fi
    else
        echo "[$index/$total] ✓ Using cached IDL for $program_id"
    fi

    if [ "$generate_from_source" = true ]; then
        echo "[$index/$total] ⚙️ No IDL found; generating ClickHouse source directly..."
        if python3 scripts/generate_clickhouse_from_source.py --overwrite "decoders/$decoder_name" > "$log_file" 2>&1; then
            echo "SUCCESS" > "$status_file"
            echo "✅ [$index/$total] Successfully generated ClickHouse files for $decoder_name"
        else
            echo "FAILED" > "$status_file"
            echo "❌ [$index/$total] Failed to generate ClickHouse files for $decoder_name"
        fi
        return 0
    fi
    
    # Run carbon-cli parse with cached IDL file
    if node packages/cli/dist/cli.js parse \
        --idl "$idl_cache_file" \
        --out-dir "decoders/$decoder_name" \
        --name "$decoder_base_name" \
        --program-id "$program_id" \
        --as-crate \
        --package-version 1.0.0 \
        --standalone = false \
        --standard anchor \
        --with-postgres true \
        --with-clickhouse true \
        --with-graphql true \
        --with-base58 true \
        --postgres-mode typed \
        --no-clean false > "$log_file" 2>&1; then
        echo "SUCCESS" > "$status_file"
        echo "✅ [$index/$total] Successfully regenerated $decoder_name"
    else
        echo "FAILED" > "$status_file"
        echo "❌ [$index/$total] Failed to regenerate $decoder_name"
    fi
}

export -f fetch_and_cache_idl
export -f regenerate_decoder
export RPC_URL
export TEMP_DIR
export IDL_CACHE_DIR

# Run decoders in parallel using xargs
printf '%s\n' "${DECODERS[@]}" | nl -v 1 -w 1 -s '|' | \
    xargs -P "$PARALLEL_JOBS" -I {} bash -c '
        IFS="|" read -r index entry <<< "{}"
        regenerate_decoder "$entry" "$index" "'"$TOTAL"'"
    '

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Regeneration complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Count successes and failures
SUCCESS=0
FAILED=0
declare -a FAILED_DECODERS

for entry in "${DECODERS[@]}"; do
    IFS=':' read -r decoder_name program_id <<< "$entry"
    status_file="$TEMP_DIR/${decoder_name}.status"
    
    if [ -f "$status_file" ]; then
        status=$(cat "$status_file")
        if [ "$status" = "SUCCESS" ]; then
            SUCCESS=$((SUCCESS + 1))
        else
            FAILED=$((FAILED + 1))
            FAILED_DECODERS+=("$decoder_name")
        fi
    else
        FAILED=$((FAILED + 1))
        FAILED_DECODERS+=("$decoder_name")
    fi
done

echo "✅ Success: $SUCCESS/$TOTAL"
echo "❌ Failed:  $FAILED/$TOTAL"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failed decoders:"
    for failed in "${FAILED_DECODERS[@]}"; do
        echo "  - $failed"
    done
    exit 1
fi
