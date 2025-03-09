**Project Bi-Weekly Update: Bloom Filter Performance in DDoS mitigation**

**Student:** Jonathan Ami
**Date:** March 7, 2025

---

### **Planned Activities:**

- Simulate a DDoS attack to evaluate performance under high load.
- Benchmark Bloom Filtervs Hashset Memory Usage

### **Progress Update:**

#### **Packet Filter false positive rate**

In this update, I made improvements to the DDos attack benchmarking. The simulation code was refactored to separate Bloom Filter testing and HashSet testing
for filtering out attack packets. The following changes were made:

- The DDoS module was moved from a single function to a struct. The struct implementation contains separate functions for both Bloom Filter implementations and HashMap.
- Timestamps were updated to properly record HashSet vs Bloom Filter performance

Bloom filter false positive output:

```bash
========================================
Running ddos test with Bloom Filter implementation
Running DDoS filtering benchmark...
=== DDoS Packet Filtering Performance ===
Total Packets Checked: 1000000
Elapsed Time: 0.06 sec
Throughput: 16320842.97 packets/sec
False Positives: 211 (0.02%)
False Negatives: 0 (0.00%)
========================================
```

The Bloom Filter was set to a false positive rate of 0.1 and outputed a false positive
rate of 0.02%. The Bloom Filter implementation took 0.06 sec. HashSets will never output a false positive thus the Bloom Filter implementation provides
lower accuracy than the HashSet implementation.

#### **DDoS Attack Memory Usage:**

Heaptrack was used to monitor memory usage. main.rs was updated to accept command-line arguments to conditionally run either the Bloom Filter or HashSet implementation of
the packet filter based off the --bloom flag.
Example usage:

```bash
cargo run --release -- --test ddos --bloom
```

This way I was able to test the Bloom Filter and HashSet implementations of the packet filter separately as heaptrack monitors the memory usage of the entire program.
Heaptrack usage

```bash
heaptrack cargo run --release -- --test ddos
```

The following is the memory usage statistics for both the HashSet and Bloom Filter implementations the packet filter.
HashSet implementation:

```bash
heaptrack stats:
	allocations:          	246053
	leaked allocations:   	3321
	temporary allocations:	79753
```

Bloom filter implementation:

```bash
heaptrack stats:
	allocations:          	246056
	leaked allocations:   	3323
	temporary allocations:	79753
```

As shown above the Bloom Filter has more allocations than the HashSet. This contradicts results in Bloom's paper on space/time tradeoffs as the Bloom Filter is expected to
have overall fewer memory allocations than the HashSet. Further investigation is required to determine why this contradiction is occurring.

### **Changes in the Repository:**

- **Refactored probabilistic filter implementation** to use an existing rust implementation of the Bloom Filter.
- **Refactored DDoS module to separate performance testing and false positive testing**

### **Next Steps:**

- Investigate why the HashSet is performing better than the Bloom Filter
- Potentially implement both HashSet and Bloom filter in C for more controlled benchmarking
- Investigate performance in other applications of packet filters
- Explore performance of Bloom Filters when modifying false positive rate
