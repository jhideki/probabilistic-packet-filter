#!/bin/bash

# === CONFIG ===
HASHSET_EXEC="hashset-test/target/release/hashset-test"
BLOOM_EXEC="bloomfilter-test/target/release/bloomfilter-test"


START_N=100000
MAX_N=${1:-1000000}   # Accept MAX_N as first CLI arg, default to 1,000,000
STEP=100000
FP_RATE=0.01

OUTPUT_DIR="benchmark_results"
rm -fr "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# === BUILD PROJECTS ===
echo "Building both projects in release mode..."
cargo build --release --manifest-path hashset-test/Cargo.toml
cargo build --release --manifest-path bloomfilter-test/Cargo.toml

# === FUNCTION TO GET MEMORY ===
get_memory_usage() {
    local pid=$1
    local status_file="/proc/$pid/status"
    local vmrss=$(grep -i VmRSS "$status_file" | awk '{print $2, $3}')
    local vmsize=$(grep -i VmSize "$status_file" | awk '{print $2, $3}')
    echo "$vmrss, $vmsize"
}

# === RUN BENCHMARKS ===
for ((n=START_N; n<=MAX_N; n+=STEP)); do
    echo "Running HashSet with N=$n"
    $HASHSET_EXEC $n &
    sleep 1  # Wait for allocation

    pid=$(pidof -s hashset-test)
    echo "$pid"
    mem_info=$(get_memory_usage $pid)
    echo "$n, $mem_info" >> "$OUTPUT_DIR/hashset_mem.csv"
    kill $pid
    wait $pid 2>/dev/null

    echo "Running BloomFilter with N=$n, FP=$FP_RATE"
    $BLOOM_EXEC $n $FP_RATE &
    sleep 1

    pid=$(pidof -s bloomfilter-test)
    mem_info=$(get_memory_usage $pid)
    echo "$n, $mem_info" >> "$OUTPUT_DIR/bloomfilter_mem.csv"
    kill $pid
    wait $pid 2>/dev/null
done

# === HEADERS ===
sed -i '1iN, VmRSS, VmSize' "$OUTPUT_DIR/hashset_mem.csv"
sed -i '1iN, VmRSS, VmSize' "$OUTPUT_DIR/bloomfilter_mem.csv"

echo "✅ Memory profiling complete. Check $OUTPUT_DIR/*.csv for results."
