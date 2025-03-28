use polars::prelude::*;
use std::error::Error;
use std::fs::File;


fn load_frame(path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let file = File::open(path)?; // '?' error propagation, ie. try open file, if fail, early return the error
    let df = ParquetReader::new(file).finish()?; // '?' only for returned Result<...,...>
    Ok(df) // Everything worked, no error, return df
}

fn group_by_category(df: DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let grouped = df // .lazy() takes ownership, therefore we must clone, if we want to use df later
        .lazy() // build plan
        .group_by(["category"]) // add to plan
        .agg([col("values").mean().alias("mean_values")])  // add to plan
        .collect()?;  // execute plan
    Ok(grouped)
}

pub fn run(input_path: &str) -> Result<(), Box<dyn Error>> {
    let df = load_frame(input_path)?;
    let _grouped = group_by_category(df)?;
    // println!("Grouped means:\n{}", grouped);
    Ok(()) // Everything worked, no error, no value to return
}
