**Project Bi-Weekly Update: Bloom Filter Performance in DDoS mitigation**

**Student:** Jonathan Ami
**Date:** March 7, 2025

---

### **Planned Activities:**

- Simulate a DDoS attack to evaluate performance under high load.
- Benchmark Bloom Filtervs Hashset Memory Usage

### **Progress Update:**

#### **Packet Filter false positive rate**

In this update, I made improvements to the DDos attack benchmarking. The simulation code was refactored to seperate bloom filter testing and hashset testing
for filtering out attack packets. The following changes were made:

- The Ddos module was moved from a single function to a struct. The struct implemention contains seperate functions for both bloom filter implementationns and hashmap.
- Timestamps were updated to properly record hashset vs bloom filter performance

Bloom filter false positive output:

```bash
========================================
Running ddos test with bloom filter implementation
Running DDoS filtering benchmark...
=== DDoS Packet Filtering Performance ===
Total Packets Checked: 1000000
Elapsed Time: 0.06 sec
Throughput: 16320842.97 packets/sec
False Positives: 211 (0.02%)
False Negatives: 0 (0.00%)
========================================
```

The bloom filter was set to a false positive rate of 0.1 and outputed a false positive
rate of 0.02%. The bloom filter immplementation took 0.06 sec. Hashets will never output a false positive thus the bloom filter implementaion provides
lower accuracy than the hashset implementation.

#### **DDoS Attack Memory Usage:**

Heaptrack was used to monitor memory usage. main.rs was updated to accept command-line arguments to conditionally run either the bloom filter or hashset implementation of
the packet filter based off the --bloom flag.
Example usage:

```bash
cargo run --release -- --test ddos --bloom
```

This way I was able to test the bloom filter and hashset implementations of the packet filter seperatly as heaptrack monitors the memeory usage of the entire program.
Heaptrack usage

```bash
heaptrack cargo run --release -- --test ddos
```

The folling is the memory usage statistics for both the hashset and bloom filter implementations the packet filter.
Hashet implementation:

```bash
heaptrack stats:
	allocations:          	246053
	leaked allocations:   	3321
	temporary allocations:	79753
```

Bloom filter implentation:

```bash
heaptrack stats:
	allocations:          	246056
	leaked allocations:   	3323
	temporary allocations:	79753
```

As shown above the bloom filter has more allocations than the hashset. This contradicts results in Bloom's paper on space/time tradeoffs as the bloom filter is expected to
have overall fewer memory allocations than the hashset. Further investigation is required to determine why this contradiction is occuring.

### **Changes in the Repository:**

- **Refactored probabilistic filter implementation** to use an existing rust implemntaion of the bloom filter.
- **Refactored Ddos module to sepearte perforamnce testing and false positive testing**

### **Next Steps:**

- Investigate why the hashset is performing better than the bloom filter
- Potentially implement both HashSet and Bloom filter in C for more controlled benchmarking
- Investigate performance in other applications of packet filters
- Explore performance of bloom filters when modifying false postive rate
