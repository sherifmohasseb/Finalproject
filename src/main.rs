use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./Egypt-Used-Car-Price.csv";

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(record) = line {
                println!("{}", record);
            }
        }
    } else {
        eprintln!("Error: Could not open the file '{}'.", file_path);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
