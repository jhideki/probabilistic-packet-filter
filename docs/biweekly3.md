**Project Bi-Weekly Update: HashSet vs Bloom Filter Insertion Memory Benchmarking**

**Student:** Jonathan Ami  
**Date:** March 21, 2025

---

### **Planned Activities:**

- Refactor benchmarking code to isolate memory usage of data structures
- Remove lookup and filtering logic to benchmark pure insertion overhead
- Use heaptrack to compare Bloom Filter vs HashSet memory usage more accurately

---

### **Progress Update:**

#### **Refactor for Insertion-Only Testing**

The `Ddos` benchmarking module was significantly refactored to better isolate and analyze memory usage. The major improvements include:

- **Eliminated temporary packet storage**: Instead of storing packets in `Vec<Ipv4Addr>`, packets are now generated and inserted directly into the data structure.
- **Pure insertion benchmarks**: `is_malicious()` calls were removed. This ensures the benchmark only measures the memory and performance costs of insertion.
- **Simplified data pipeline**: Random IPs are inserted directly into the Bloom filter or HashSet without any lookup or retention, reducing confounding memory overhead.

These changes help ensure that any memory reported by `heaptrack` is strictly from the data structure itself.

---

### **Benchmark Results (heaptrack)**

Tests were run using the following commands:

**HashSet:**

```bash
heaptrack cargo run --release -- --test ddos-performance
```

**Bloom Filter:**

```bash
heaptrack cargo run --release -- --test ddos-performance --bloom
```

Here are the key results from `heaptrack` for both implementations:

#### HashSet implementation:

```bash
peak heap memory consumption:     11.2MB
peak RSS (including overhead):    24.0MB
total memory leaked:              416.9kB
calls to allocation functions:    251,395
temporary allocations:            81,548 (32.44%)
total runtime:                    00.114s
```

#### Bloom Filter implementation:

```bash
peak heap memory consumption:     11.2MB
peak RSS (including overhead):    23.9MB
total memory leaked:              417.0kB
calls to allocation functions:    251,398
temporary allocations:            81,548 (32.44%)
total runtime:                    00.114s
```

---

### **Discussion & Analysis**

Despite the expectation that a Bloom filter should use significantly less memory than a HashSet, both implementations showed identical peak memory usage (~11.2MB) and almost identical allocation behavior. The Bloom filter even made slightly more allocations than the HashSet.

This was surprising because:

- A Bloom filter with 1 million entries and 1% false positive rate theoretically only needs ~1.2MB.
- A HashSet must store all keys with higher per-key overhead.

Possible explanations include:

- The memory overhead is still dominated by random IP generation and internal hashing structures.
- The Bloom filter implementation may not be optimized and might use a `Vec<u8>` or boxed internal buffers.
- Small-scale testing (1M entries) is not enough to expose the Bloom filter’s advantages — a larger-scale test (10M–100M keys) may be required.

This contradiction highlights a key real-world insight: theoretical memory savings may be masked by surrounding allocations unless benchmarking is tightly controlled.

---

### **Changes in the Repository:**

- Rewrote `ddos.rs` to remove packet lookups and retain only direct insertion benchmarking
- Removed unnecessary `Vec`s and replaced them with inline IP generation
- Updated `main.rs` to pass benchmark parameters directly (e.g., blacklist size, FPR)

---

### **Next Steps:**

- Scale up to 10 million or 100 million insertions to better expose the Bloom filter's space advantages
- Swap out current Bloom filter implementation with a minimal or custom one to reduce overhead
- Investigate memory profiles for different Bloom filter false positive rates
- Try using `u32` instead of `Ipv4Addr` to reduce allocation overhead
- Consider re-implementing both data structures in C or Rust with `no_std` to benchmark raw memory use
- Performe tests with varying parameters (false positive rate, number of packets, etc...) then plotting results
