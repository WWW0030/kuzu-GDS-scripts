# kuzu-GDS-scripts
This repository contains scripts designed to benchmark Graph Data Science (GDS) algorithms using the Kuzu graph database management system. These tools facilitate performance evaluation and correctness testing of GDS algorithms within Kuzu.

# Repository Structure and Usage
## WCC-Correctness benchmarking
`run_benchmark.sh` and `run_correctness.py` are used for testing correctness of algorithm. 
Three files needs to be available `node_csv`, `edge_csv`, `correctness_file`.
### Required files:
- `run_benchmark.sh` and `run_correctness.py` should be under the same directory
- `node_csv` and `edge_csv` and their path are required variables to change within `run_benchmark.sh`.
    - `node_csv` and `edge_csv` should follow the format of generic datasets provided in: https://ldbcouncil.org/benchmarks/graphalytics/ specfically:
    - `node_csv` is a csv of one column (id)
    - `edge_csv` is a csv of three columns with space delimiter (`source id`, `destination id`, `weight`)
- `correctness_file` and its path is required variable to change within `run_benchmark.sh`
    - `correctness_file` should be two columns with space delimiter (`id`, `component_id`)
    - (For future correctness check of other algorithms, this can change)
### Running Benchmark
- Build instance of kuzu
- Replace file paths in `run_benchmark.sh`
- run `./run_benchmark.sh`
- The terminal will then print whether the result matches the expected

## Petgraph Benchmarking
### Required files:
- `main.rs` and `cargo.toml`
- Make new rust project and replace `main.rs` and `cargo.toml` with that in the repo
- Replace file path in `main.rs`
- Run rust project
