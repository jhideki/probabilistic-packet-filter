**Project Bi-Weekly Update: Implementing Bloom Filters & DDoS Attack Simulation**

**Student:** Jonathan Ami
**Date:** Feb 23, 2025

---

### **Planned Activities:**

- Implement a Bloom Filter for improved probabilistic packet filtering.
- Simulate a DDoS attack to evaluate performance under high load.
- Benchmark false positive rates and filtering efficiency.
- Optimize filtering parameters to minimize overhead.

### **Progress Update:**

#### **Implementation of Bloom Filter:**

In this update, we have integrated a **Bloom Filter** into our probabilistic filtering system to enhance accuracy and reduce false positives. Key modifications include:

- **Multi-hash function support** for improved distribution.
- **Optimized bit-array size selection** to balance memory use and false positive rates.
- **Support for dynamic insertions** without excessive memory expansion.

These changes were inspired by recent updates in the [Probabilistic Packet Filter Repository](https://github.com/jhideki/probabilistic-packet-filter/tree/main/), where similar filtering optimizations were applied for enhanced performance.

#### **DDoS Attack Simulation & Testing:**

A large-scale **DDoS attack simulation** was implemented to stress-test the filtering system:

- Attack packets and normal traffic were generated at high volume.
- Packets were screened using the Bloom Filter.
- Results were logged to measure accuracy, efficiency, and false positives.

### **Changes in the Repository:**

- **Refactored probabilistic filter implementation** to use Bloom filtering.
- **Added new hashing functions for improved accuracy.**
- **Integrated logging and benchmarking for in-depth performance analysis.**
- **Updated test cases to validate Bloom Filter efficiency.**

### **Next Steps:**

- Fine-tune filter parameters for **adaptive threshold adjustments**.
- Benchmark bloom filter against regular hashsets
- Extend real-world testing with **live network traffic datasets**.
- Explore parallelized filtering for **higher throughput scenarios**.
- Update GitHub Pages documentation with **latest results and insights**.

---

### **References:**

## [1] Probabilistic Filtering for Network Security, IEEE Transactions, 2023.
