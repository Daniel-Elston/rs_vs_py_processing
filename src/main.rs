mod config;
mod utils;
mod data_creation;
mod data_processing;

use std::error::Error;
use std::time::Instant;
use core::time::Duration;
use std::thread;
use std::thread::JoinHandle;

use utils::pipe_utils::*;

use crate::config::NUM_RUNS;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // std::env::set_var("POLARS_MAX_THREADS", num_cpus::get().to_string()); // get thread count
    std::env::set_var("POLARS_MAX_THREADS", "16"); // get thread count

    let mut handles = Vec::with_capacity(NUM_RUNS); // Pre-allocate mem for NUM_RUNS upfront
    // vector store to hold threads ^
    
    for i in 0..NUM_RUNS {
        let handle: JoinHandle<(Duration, Duration)> = thread::spawn(move || { //<-- each thread runs the closure 'move' in parallel
            // Handle for ^ each thread. Spawn a thread for each ^ run.
            println!("\n--- Run {} ---", i + 1);
            let filename = format!("data/data_{}.parquet", i); // Create thread-specific filename

            let creation_duration = {
                let start = Instant::now();
                let _ = data_creation::create::run(&filename);
                start.elapsed()
            };
            
            let processing_duration = {
                let start = Instant::now();
                let _ = data_processing::process::run(&filename);
                start.elapsed()
            };
            
            (creation_duration, processing_duration)
        });
        handles.push(handle);
    }
    

    let mut creation_times = Vec::new(); // objects *_times in scope for fn main()
    let mut processing_times = Vec::new(); // ^
    
    for handle in handles {
        let (c, p) = handle.join().unwrap(); // Join(): Wait for thread to finish, return vals, Ok((c,p))
        // unwrap(): If success return vals, else panic ^
        creation_times.push(c);
        processing_times.push(p);
    }
    let avg_creation_time = average_duration(creation_times); // Ownership changes, past this line *_times out of scope
    let avg_processing_time = average_duration(processing_times); // ^ Objects now in scope for fn average_duration

    for i in 0..NUM_RUNS {
        let _ = std::fs::remove_file(format!("data/data_{}.parquet", i));
    }

    let total = start.elapsed();
    let results = display_results(avg_creation_time, avg_processing_time, total);

    save_results(&results, "results/rust_results.txt")?;
    
    Ok(())
}
