use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

use crate::stats;

pub fn analyze_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut bids = Vec::new();
    let mut asks = Vec::new();
    let mut volumes = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let bid: f64 = record.get(0).unwrap_or("0").trim().parse()?;
        let ask: f64 = record.get(1).unwrap_or("0").trim().parse()?;
        let vol: f64 = record.get(2).unwrap_or("0").trim().parse()?;

        bids.push(bid);
        asks.push(ask);
        volumes.push(vol);
    }

    println!("\n✅ Successfully loaded data from CSV. Running global stats analysis...");
    stats::global_stats_summary(&bids, &asks, &volumes);

    Ok(())
}
