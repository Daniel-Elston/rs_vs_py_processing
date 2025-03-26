use polars::prelude::*;
use rand::Rng;
use polars::datatypes::PlSmallStr;
use std::fs::File;
use std::error::Error;


fn generate_data(num_rows: usize) -> (Vec<i32>, Vec<f64>, Vec<String>) {
    let mut rng = rand::thread_rng();
    let ids: Vec<i32> = (1..=num_rows as i32).collect();
    let values: Vec<f64> = (0..num_rows).map(|_| rng.gen_range(0.0..100.0)).collect();
    let categories: Vec<String> = (0..num_rows)
        .map(|_| match rng.gen_range(0..4) {
            0 => "A".to_string(),
            1 => "B".to_string(),
            2 => "C".to_string(),
            _ => "D".to_string(),
        })
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

fn save_frame(
    df: &mut DataFrame,
    path: &str
) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    CsvWriter::new(file)
        // .has_header(true)
        .finish(df)?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let num_rows = 100;
    let (ids, values, categories) = generate_data(num_rows);
    let mut df = create_frame(ids, values, categories)?;
    save_frame(&mut df, "data/data.csv")?;
    Ok(())
}
