mod data_creation;
mod data_processing;

use std::error::Error;
use std::time::Instant;
use core::time::Duration;

fn create_data() -> Duration {
    let start = Instant::now();
    let _ = data_creation::create::run();
    start.elapsed()
}

fn process_data() -> Duration {
    let start = Instant::now();
    let _ = data_processing::process::run();
    start.elapsed()
}

fn duration_to_millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn average_duration(durations: &[Duration]) -> Duration {
    let total = durations.iter().sum::<Duration>();
    total / durations.len() as u32
}

fn display_results(creation_times: &[Duration], processing_times: &[Duration]) {
    let avg_creation = average_duration(creation_times);
    let avg_processing = average_duration(processing_times);

    println!("\n=== Average Execution Times over {} Runs ===", creation_times.len());
    println!("Average creation time: {:.3} ms", duration_to_millis(avg_creation));
    println!("Average processing time: {:.3} ms", duration_to_millis(avg_processing));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut creation_times = Vec::new();
    let mut processing_times = Vec::new();

    for i in 0..10 {
        println!("\n--- Run {} ---", i + 1);

        let creation_duration = create_data();
        creation_times.push(creation_duration);

        let processing_duration = process_data();
        processing_times.push(processing_duration);
    }

    display_results(&creation_times, &processing_times);
    Ok(())
}