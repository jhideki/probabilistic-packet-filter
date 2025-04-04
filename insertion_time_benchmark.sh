#!/bin/bash

# === CONFIG ===
HASHSET_EXEC="hashset-test/target/release/hashset-test"
BLOOM_EXEC="bloomfilter-test/target/release/bloomfilter-test"

START_N=100000
MAX_N=${1:-1000000}     # Default max inserts if not provided
STEP=100000
FP_RATE=0.01

OUTPUT_CSV="benchmark_results/insertion_times.csv"

# === INIT CSV ===
echo "N,Time (ms),Type" > "$OUTPUT_CSV"

# === RUN BENCHMARKS ===
for ((n=START_N; n<=MAX_N; n+=STEP)); do
    echo "Running HashSet with N=$n"
    hash_output=$(timeout 0.5s $HASHSET_EXEC $n 2>/dev/null)
    hash_time=$(echo "$hash_output" | grep "inserted" | grep -oP "[0-9]+\.[0-9]+(?=ms)")
    if [[ -n "$hash_time" ]]; then
        echo "$n,$hash_time,HashSet" >> "$OUTPUT_CSV"
    else
        echo "❌ Failed to parse HashSet output at N=$n"
    fi

    echo "Running BloomFilter with N=$n and FP=$FP_RATE"
    bloom_output=$(timeout 0.5s $BLOOM_EXEC $n $FP_RATE 2>/dev/null)
    bloom_time=$(echo "$bloom_output" | grep "inserted" | grep -oP "[0-9]+\.[0-9]+(?=ms)")
    if [[ -n "$bloom_time" ]]; then
        echo "$n,$bloom_time,BloomFilter" >> "$OUTPUT_CSV"
    else
        echo "❌ Failed to parse BloomFilter output at N=$n"
    fi
done

echo "✅ Done. Results saved to $OUTPUT_CSV"
