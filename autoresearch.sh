#!/usr/bin/env bash
set -euo pipefail

# Cairn benchmark harness: measures development-workflow performance and release binary size
# Primary metric: scan_time_ms (best of 3 warm runs)
# Secondary metrics: lint_time_ms, health_time_ms, next_time_ms, hook_time_ms, binary_size_kb

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Build release binary
cargo build --release 2>/dev/null

BINARY="./target/release/cairn"

# Helper: benchmark a command, returning best (lowest) ms of 3 runs
_i="Benchmarking cairn commands"
benchmark_cmd() {
    local cmd="$1"
    local best_ms=999999
    # Warm-up run
    eval "$cmd" >/dev/null 2>&1 || true
    # Measure 3 runs
    for i in 1 2 3; do
        start_ms=$(perl -MTime::HiRes=time -e 'printf "%.0f", time * 1000')
        eval "$cmd" >/dev/null 2>&1
        end_ms=$(perl -MTime::HiRes=time -e 'printf "%.0f", time * 1000')
        dur_ms=$((end_ms - start_ms))
        if [ "$dur_ms" -lt "$best_ms" ]; then
            best_ms=$dur_ms
        fi
    done
    echo "$best_ms"
}

scan_ms=$(benchmark_cmd '"$BINARY" scan --json')
lint_ms=$(benchmark_cmd '"$BINARY" lint --json')
health_ms=$(benchmark_cmd '"$BINARY" health --json')
next_ms=$(benchmark_cmd '"$BINARY" next')
hook_ms=$(benchmark_cmd '"$BINARY" hook all --json')

# Measure binary size
binary_size_kb=$(du -k "$BINARY" | cut -f1)

echo "METRIC scan_time_ms=$scan_ms"
echo "METRIC lint_time_ms=$lint_ms"
echo "METRIC health_time_ms=$health_ms"
echo "METRIC next_time_ms=$next_ms"
echo "METRIC hook_time_ms=$hook_ms"
echo "METRIC binary_size_kb=$binary_size_kb"

exit 0
