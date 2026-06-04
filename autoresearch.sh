#!/usr/bin/env bash
set -euo pipefail

# Cairn benchmark harness: measures scan performance and release binary size
# Primary metric: scan_time_ms (best of 3 warm runs)
# Secondary metric: binary_size_kb

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Build release binary
cargo build --release 2>/dev/null

BINARY="./target/release/cairn"

# Warm-up run to warm filesystem caches
"$BINARY" scan --json >/dev/null 2>&1 || true

# Measure 3 runs, keep best (lowest) time in ms
best_ms=999999
for i in 1 2 3; do
    start_ms=$(perl -MTime::HiRes=time -e 'printf "%.0f", time * 1000')
    "$BINARY" scan --json >/dev/null 2>&1
    end_ms=$(perl -MTime::HiRes=time -e 'printf "%.0f", time * 1000')
    dur_ms=$((end_ms - start_ms))
    if [ "$dur_ms" -lt "$best_ms" ]; then
        best_ms=$dur_ms
    fi
done

# Measure binary size
binary_size_kb=$(du -k "$BINARY" | cut -f1)

echo "METRIC scan_time_ms=$best_ms"
echo "METRIC binary_size_kb=$binary_size_kb"

exit 0
