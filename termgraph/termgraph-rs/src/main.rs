use std::{env, error::Error, path::Path, process};

use csv::ReaderBuilder;
use termgraph_rust::{Candle, CandleStickGraph};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <csv_file>", args[0]);
        process::exit(2);
    }
    if let Ok(data) = read_csv(&args[1]) {
        let render = CandleStickGraph::new(&data, 30);
        render.draw();
    };
}

fn read_csv<P: AsRef<Path>>(path: &P) -> Result<Vec<Candle>, Box<dyn Error>> {
    let mut data = Vec::new();
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;

    for some in rdr.deserialize() {
        let candle: Candle = some?;
        data.push(candle);
    }
    Ok(data)
}
