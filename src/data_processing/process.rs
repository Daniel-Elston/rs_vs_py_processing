use polars::prelude::*;
use std::error::Error;


fn load_frame(path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;
    Ok(df)
}

fn group_by_category(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let grouped = df.clone()
        .lazy()
        .group_by(["category"])
        .agg([col("values").mean().alias("mean_values")])
        .collect()?;
    Ok(grouped)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let df = load_frame("data/data.csv")?;
    let grouped = group_by_category(&df)?;
    println!("Grouped means:\n{}", grouped);
    Ok(())
}
