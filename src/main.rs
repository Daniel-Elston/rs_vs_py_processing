mod data_creation;
mod data_processing;

use std::time::Instant;
use std::error::Error;

fn create_data() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let _ = data_creation::create::run();
    let duration = start.elapsed();

    println!("Creation - Execution time: {:?}", duration);
    Ok(())
}

fn process_data() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let _ = data_processing::process::run();
    let duration = start.elapsed();

    println!("Processing - Execution time: {:?}", duration);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    create_data()?;
    process_data()?;
    Ok(())
}
