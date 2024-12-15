use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use nalgebra::DMatrix;
fn parse_dataset(file_path: &str) -> Vec<(f64, f64, f64, f64)> {
    let mut dataset = Vec::new();
    let mut skipped_rows = 0;

    if !Path::new(file_path).exists() {
        eprintln!("Error: File '{}' does not exist. Check the file path.", file_path);
        return dataset;
    }

    println!("Reading dataset from '{}'", file_path);
    if let Ok(lines) = read_lines(file_path) {
        for (i, line) in lines.enumerate() {
            if let Ok(record) = line {
                if i == 0 {
                    continue;
                }
                let fields: Vec<&str> = record.split(',').map(|field| field.trim()).collect();
                if fields.len() < 12 {
                    skipped_rows += 1;
                    continue;
                }

                let year_raw = fields[5];
                let engine_raw = fields[7];
                let kilometers_raw = fields[8];
                let price_raw = fields[11];

                if is_numeric(year_raw) && is_numeric(engine_raw) && is_numeric(kilometers_raw) && is_numeric(price_raw) {
                    let year = year_raw.parse::<f64>().unwrap();
                    let engine = engine_raw.parse::<f64>().unwrap();
                    let kilometers = kilometers_raw.parse::<f64>().unwrap();
                    let price = price_raw.parse::<f64>().unwrap();

                    dataset.push((kilometers, year, engine, price));
                } else {
                    skipped_rows += 1;
                }
            } else {
                skipped_rows += 1;
            }
        }
    } else {
        eprintln!("Error: Could not open the file '{}'. Check permissions.", file_path);
    }

    if !dataset.is_empty() {
        println!("Successfully parsed {} rows of valid data.", dataset.len());
    } else {
        println!("No valid rows were parsed.");
    }
    println!("Skipped {} invalid rows.", skipped_rows);

    dataset
}
fn is_numeric(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn mean(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}
fn standard_deviation(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let avg = mean(data);
    let variance = data.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}
fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }
    let mean_x = mean(x);
    let mean_y = mean(y);
    let numerator: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| (xi - mean_x)*(yi - mean_y)).sum();
    let denominator_x = x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f64>().sqrt();
    let denominator_y = y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<f64>().sqrt();
    if denominator_x == 0.0 || denominator_y == 0.0 {
        0.0
    } else {
        numerator / (denominator_x * denominator_y)
    }
}
