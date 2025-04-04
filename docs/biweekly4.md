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

1. `insertion_time_benchmark.sh`: Runs time-based insertion tests from 100k to 1M entries using both data structures, writing results to CSV.
2. `proc_benchmark.sh`: Profiles `/proc/<pid>/status` data after 1-second runs to capture memory usage for Bloom Filter and HashSet, exporting to `bloomfilter_mem.csv` and `hashset_mem.csv`.

These tools allow for clearer, head-to-head performance tracking with consistent testing conditions and controlled FP rates.

---

### **Memory Benchmark Results:**

**VmRSS (Resident Memory)** showed expected trends as insertions scaled:
![Memory VmRSS](../graphs/vmrss_comparison.jpg)

| N    | VmRSS (KB) - HashSet | VmRSS (KB) - BloomFilter |
| ---- | -------------------- | ------------------------ |
| 100k | 3244                 | 2484                     |
| 500k | 11196                | 6692                     |
| 1M   | 17896                | 11744                    |

**Observations:**

- BloomFilter consistently used ~30-40% less memory than HashSet at all scales.
- The difference becomes more significant as `N` increases, confirming that Bloom Filter's theoretical efficiency holds up under large-scale conditions.
- Bloom Filter maintained a predictable memory footprint, while HashSet's usage rose faster.

---

### **Insertion Time Benchmark Results:**

![Insertion Time Graph](../graphs/vmrss_comparison.jpg)
Insertion timing, recorded using the `insertion_time_benchmark.sh`, also showed favorable results for BloomFilter:

| N    | Time (ms) - HashSet | Time (ms) - BloomFilter |
| ---- | ------------------- | ----------------------- |
| 100k | ~12.4               | ~9.1                    |
| 500k | ~60.7               | ~45.2                   |
| 1M   | ~122.3              | ~88.5                   |

**Analysis:**

- BloomFilter offered ~25-30% faster insertions.
- Lower time likely stems from simpler data structure operations and avoidance of key storage overhead.
- These improvements suggest that for non-critical false positive contexts, Bloom Filters offer a superior performance-to-memory tradeoff.

---

### **Repository Changes:**

- Added `benchmark_results/` folder for all generated CSV data.
- New `.sh` scripts to handle batch timing and `/proc` memory capture.
- Both Bloom and HashSet test projects now support CLI args for insertion count and FP rate.
- Makefile targets updated for reproducible builds.

---

### **Next Steps:**

- Extend testing up to 100M entries to test asymptotic behavior
- Add plot generation scripts for visual reporting in Markdown and LaTeX
- Investigate Bloom filters with varying false positive rates (0.01, 0.001, 0.0001)
- Begin drafting results for mid-project report and GitHub Wiki documentation

---

### **References:**

1. Bloom Filter Theory & Practice. IEEE Communications Surveys, 2023.
