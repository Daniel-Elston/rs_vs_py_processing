use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use crate::config::NUM_RUNS;


pub fn duration_to_millis(duration: Duration) -> f64 {
    // param: T assigns ownership
    duration.as_secs_f64() * 1000.0
}

pub fn average_duration(durations: Vec<Duration>) -> Duration {
    // Using as here to match types. For example, (5:int / 2:int = 2.5:float) is ❌
    // Must cast as (5:float/2:float = 2.5:float) is ✅
    let total: u128 = durations.iter().map(|d| d.as_millis()).sum();
    let average = total as f64 / durations.len() as f64;
    Duration::from_millis(average as u64)
}

pub fn display_results(avg_creation: Duration, avg_processing: Duration, total_runtime: Duration) -> String {
    let result_string = format!(
        "\n=== Average Function Times over {} Runs ===\n\
        Average creation time: {:.3} ms\n\
        Average processing time: {:.3} ms\n\
        Total Rust main() time: {:.3} ms\n",
        NUM_RUNS,
        duration_to_millis(avg_creation),
        duration_to_millis(avg_processing),
        duration_to_millis(total_runtime)
    );
    println!("{result_string}");
    result_string
}


// fn save_results(results: &str, filename: &str) -> Result<(), Box<dyn Error>> {
//     // param: &T is a reference to the value
//     // &param: &T, is a copy of the value (like .clone())
//     let mut file = File::create(filename)?;
//     file.write_all(results.as_bytes())?;
//     Ok(())
// }
pub fn save_results<R: AsRef<str>, F: AsRef<str>>(results: R, filename: F) -> Result<(), Box<dyn Error>> {
    // Idiomatic version of above commented out fn ^^^. Accepts String, &str or &String
    let mut file = File::create(filename.as_ref())?;
    file.write_all(results.as_ref().as_bytes())?;
    Ok(())
}