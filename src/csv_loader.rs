use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads a CSV file with format: bid,ask,volume
/// Returns vectors of (bids, asks, volumes)
pub fn read_csv_to_vectors(path: &str) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), String> {
    let file = File::open(path).map_err(|_| "Failed to open file.")?;
    let reader = BufReader::new(file);

    let mut bids = Vec::new();
    let mut asks = Vec::new();
    let mut volumes = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|_| format!("Failed to read line {}", i + 1))?;
        let parts: Vec<&str> = line.trim().split(',').collect();

        if parts.len() != 3 {
            return Err(format!("Line {} is not properly formatted (needs 3 values)", i + 1));
        }

        let bid = parts[0].trim().parse::<f64>().map_err(|_| format!("Invalid bid at line {}", i + 1))?;
        let ask = parts[1].trim().parse::<f64>().map_err(|_| format!("Invalid ask at line {}", i + 1))?;
        let volume = parts[2].trim().parse::<f64>().map_err(|_| format!("Invalid volume at line {}", i + 1))?;

        bids.push(bid);
        asks.push(ask);
        volumes.push(volume);
    }

    Ok((bids, asks, volumes))
}
