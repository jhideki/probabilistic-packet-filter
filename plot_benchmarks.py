import pandas as pd
import matplotlib.pyplot as plt

def clean_memory_column(series):
    # Remove 'kB', strip whitespace, convert to numeric (gracefully handles bad values)
    return pd.to_numeric(series.astype(str).str.replace("kB", "", regex=False).str.strip(), errors='coerce')

# Load and clean memory CSVs
hashset_df = pd.read_csv("proc_benchmark_results/hashset_mem.csv")
bloom_df = pd.read_csv("proc_benchmark_results/bloomfilter_mem.csv")

# Strip whitespace from column headers
hashset_df.columns = hashset_df.columns.str.strip()
bloom_df.columns = bloom_df.columns.str.strip()

# Clean memory values
hashset_df["VmRSS"] = clean_memory_column(hashset_df["VmRSS"])
hashset_df["VmSize"] = clean_memory_column(hashset_df["VmSize"])
bloom_df["VmRSS"] = clean_memory_column(bloom_df["VmRSS"])
bloom_df["VmSize"] = clean_memory_column(bloom_df["VmSize"])

# Drop rows with NaNs (in case any parse fails)
hashset_df.dropna(inplace=True)
bloom_df.dropna(inplace=True)

# === Plot VmRSS Comparison ===
plt.figure(figsize=(10, 6))
plt.plot(hashset_df["N"], hashset_df["VmRSS"], label="HashSet VmRSS", marker='o')
plt.plot(bloom_df["N"], bloom_df["VmRSS"], label="BloomFilter VmRSS", marker='x')
plt.xlabel("Number of Inserts (N)")
plt.ylabel("VmRSS (kB)")
plt.title("VmRSS vs Number of Inserts")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("graphs/vmrss_comparison.jpg", dpi=300)
plt.close()

# === Plot VmSize Comparison ===
plt.figure(figsize=(10, 6))
plt.plot(hashset_df["N"], hashset_df["VmSize"], label="HashSet VmSize", marker='o')
plt.plot(bloom_df["N"], bloom_df["VmSize"], label="BloomFilter VmSize", marker='x')
plt.xlabel("Number of Inserts (N)")
plt.ylabel("VmSize (kB)")
plt.title("VmSize vs Number of Inserts")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("graphs/vmsize_comparison.jpg", dpi=300)
plt.close()

# === Plot Insertion Time ===
insertion_df = pd.read_csv("insertion_times.csv")
insertion_df.columns = insertion_df.columns.str.strip()

# Split into two separate DataFrames
hashset_times = insertion_df[insertion_df["Type"] == "HashSet"]
bloom_times = insertion_df[insertion_df["Type"] == "BloomFilter"]

plt.figure(figsize=(10, 6))
plt.plot(hashset_times["N"], hashset_times["Time (ms)"], label="HashSet Time", marker='o')
plt.plot(bloom_times["N"], bloom_times["Time (ms)"], label="BloomFilter Time", marker='x')
plt.xlabel("Number of Inserts (N)")
plt.ylabel("Insertion Time (ms)")
plt.title("Insertion Time vs Number of Inserts")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("graphs/insertion_time_comparison.jpg", dpi=300)
plt.close()

print("✅ All plots saved in the 'graphs/' directory.")
