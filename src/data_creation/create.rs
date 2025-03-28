use polars::prelude::*;
use rand::Rng;
use polars::datatypes::PlSmallStr;
use std::fs::File;
use std::error::Error;
use rayon::prelude::*;
use polars::prelude::ParquetWriter;
use polars::prelude::ParquetCompression;

use crate::config::NUM_ROWS;


fn generate_data() -> (Vec<i32>, Vec<f64>, Vec<String>) {
    let ids: Vec<i32> = (1..=NUM_ROWS as i32).collect();
    let values: Vec<f64> = (0..NUM_ROWS)
        .into_par_iter() // init parallel iterator
        .map_init(
            || rand::thread_rng(), // init new rng, thread local (||), seed automatically from system, safe concurrent
            |rng, _| rng.gen_range(0.0..100.0) // |args,args| to anon fn (closure), 1 f per item
        )
        .collect(); // run iterator pipe and collect results into a Vec
    let categories: Vec<String> = (0..NUM_ROWS)
        .into_par_iter()
        .map_init(
            || rand::thread_rng(),
            |rng, _| match rng.gen_range(0..4) {
                0 => "A".to_string(),
                1 => "B".to_string(),
                2 => "C".to_string(),
                _ => "D".to_string(),
            }
        )
        .collect();
    (ids, values, categories)
}

fn create_frame(
    ids: Vec<i32>,
    values: Vec<f64>,
    categories: Vec<String>
) -> Result<DataFrame, PolarsError> {
    let df = DataFrame::new(vec![
        Series::new(PlSmallStr::from("id"), ids).into(),
        Series::new(PlSmallStr::from("values"), values).into(),
        Series::new(PlSmallStr::from("category"), categories).into(),
    ])?;
    Result::Ok(df)
}

// fn save_csv(
//     df: &mut DataFrame,
//     path: &str
// ) -> Result<(), Box<dyn Error>> {
//     let file = File::create(path)?;
//     CsvWriter::new(file)
//         // .has_header(true)
//         .finish(df)?;
//     Ok(())
// }

fn save_parquet(df: &mut DataFrame, path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Snappy)
        .finish(df)?;
    Ok(())
}

pub fn run(output_path: &str) -> Result<(), Box<dyn Error>> {
    let (ids, values, categories) = generate_data();
    let mut df = create_frame(ids, values, categories)?;
    // save_csv(&mut df, output_path)?;
    save_parquet(&mut df, output_path)?;
    Ok(())
}
