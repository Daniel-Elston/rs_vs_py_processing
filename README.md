# Rust vs. Python: Performance Comparison

### Summary
This project was designed as a hands-on learning exercise to explore the Rust programming language through a performance focused lens. It serves as my first structured Rust project.

The objective is to compare **function-level** and **end-to-end runtime performance** between Rust and Python. The pipeline involves generating large synthetic datasets, processing them with Polars, timing the operations, and saving the results to disk.


#### -- Project Status: [Complete]
- Benchmark results available in: `results/*.txt`

---

#### Technologies
- **Rust** `1.85.1` (rustc)
- **Cargo** (build and dependency manager)
- **Python** `3.12.7`
- **Polars**

#### Learning Objective
- Ownership & Borrowing
- Scope & Data Flow
- Traits, Generics, & Type Inference
- Threading & Parallelism
- Thread Tuning & Join Handles
- Error Propagation & Handling
- Formatting & File I/O
- Benchmarking & Runtime Analysis

#### Performance Design

Each language runs:
- 10 iterations (runs)
- Each iteration generates **1 million rows**
- Timings include:
  - **Data creation**
  - **Data processing**
  - **Total program runtime**

Rust uses a thread-per-run strategy with Polars parallelism tuned via `POLARS_MAX_THREADS`.

---

### Contacts

For questions, feedback, or collaboration, feel free to reach out.

Email: delstonds@outlook.com