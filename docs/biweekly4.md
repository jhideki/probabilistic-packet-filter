**Project Bi-Weekly Update: Enhanced Memory & Time Benchmarking with HashSet vs BloomFilter**

**Student:** Jonathan Ami  
**Date:** April 4, 2025

---

### **Planned Activities:**

- Create new benchmarking scripts to isolate and profile memory and time usage
- Chart results from `proc_benchmark.sh` and `insertion_time_benchmark.sh`
- Modify packet filter code to simulate client server architecture with a working packet filter

---

### **Progress Update:**

#### **New Testing Methods:**

Two new benchmarking tools were introduced:

1. `insertion_time_benchmark.sh`: Runs time-based insertion tests from 100k to 5M entries using both data structures, writing results to CSV.
2. `proc_benchmark.sh`: Profiles `/proc/<pid>/status` data after 1-second runs to capture memory usage for Bloom Filter and HashSet, exporting to `bloomfilter_mem.csv` and `hashset_mem.csv`.

---

### **Memory Benchmark Results:**

**VmRSS (Resident Memory)** showed expected trends as insertions scaled:
![Memory VmRSS](assets/vmrss_comparison.jpg)

| N    | VmRSS (KB) - HashSet | VmRSS (KB) - BloomFilter |
| ---- | -------------------- | ------------------------ |
| 100k | 3244                 | 2740                     |
| 1M   | 20428                | 3680                     |
| 2M   | 38976                | 4932                     |

**Observations:**

- At 100k insertions, memory usage is relatively close.
- At 1M, the HashSet uses over 5.5x the memory of the Bloom Filter.
- By 2M insertions, that difference increases to nearly 8x, confirming the Bloom Filter’s substantial memory efficiency.

---

### **Insertion Time Benchmark Results:**

![Insertion Time Graph](assets/insertion_time_comparison.jpg)
Insertion timing, recorded using the `insertion_time_benchmark.sh`, also showed favorable results for BloomFilter:

| N    | Time (ms) - HashSet | Time (ms) - BloomFilter |
| ---- | ------------------- | ----------------------- |
| 100k | ~1.47               | ~2.69                   |
| 5M   | ~81.10              | ~182.17                 |

**Analysis:**

- BloomFilter is approximetly 2x slower than the hashset
- Likely cuased by multiple hashfunctions and bit manipultaion overhead

---

### **Repository Changes:**

- Added `benchmark_results/` folder for all generated CSV data.
- New `.sh` scripts to handle batch timing and `/proc` memory capture.
- Two new rust projects created to benchmark performance
- Both Bloom and HashSet test projects now support CLI args for insertion count and FP rate.
- Updated the pkt filter code to simulate client server architecure

---

### **Next Steps:**

- Full writen report + demo of packet filter


