mod stats;
mod csv_loader;
mod analyze_csv;
use analyze_csv::analyze_csv;

use std::io;


/// Helper function to read and parse comma-separated input into Vec<f64>
fn read_and_parse_input(label: &str) -> Vec<f64> {
    println!("Enter {} separated by commas:", label);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect()
}

fn main() {
    println!("Quantitative Market Analyzer - by Willy Sajbeni\n");
    println!("Choose a function:");
    println!("1 - Mean");
    println!("2 - VWAP");
    println!("3 - VWAP Group (Compra + Venda / 2)");
    println!("4 - Variance (Population or Sample)");
    println!("5 - Standard Deviation (Population or Sample)");
    println!("6 - VWAP Variance (Population or Sample)");
    println!("7 - VWAP Standard Deviation (Population or Sample)");
    println!("8 - VWAP Group Variance (Compra, Venda, Volume)");
    println!("9 - VWAP Group STD (Compra, Venda, Volume)");
    println!("10 - Full Market Stats Report (Global Summary)");
    println!("11 - Load data from CSV file (Bid, Ask, Volume columns)");



    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            let prices = read_and_parse_input("prices");
            if prices.is_empty() {
                println!("Invalid price input.");
                return;
            }
            let mean = stats::mean(&prices);
            println!("Mean: {:.4}", mean);
        }

        "2" => {
            let prices = read_and_parse_input("prices");
            let volumes = read_and_parse_input("volumes");

            if prices.len() != volumes.len() {
                println!("Prices and volumes must have the same length.");
                return;
            }

            let (vwap_value, signal) = stats::vwap(&prices, &volumes);
            println!("VWAP: {:.4}", vwap_value);
            println!("Signal: {}", signal);
        }

        "3" => {
            let bids = read_and_parse_input("bid prices (compra)");
            let asks = read_and_parse_input("ask prices (venda)");
            let volumes = read_and_parse_input("volumes");

            if bids.len() != asks.len() || bids.len() != volumes.len() {
                println!("Compra, venda e volume devem ter o mesmo tamanho.");
                return;
            }

            let (vwap_group, sinal) = stats::vwap_group(&bids, &asks, &volumes);
            println!("VWAP Group: {:.4}", vwap_group);
            println!("Signal: {}", sinal);
        }

        "4" => {
            let data = read_and_parse_input("values");

            println!("Is this population data? (y/n):");
            let mut type_input = String::new();
            io::stdin().read_line(&mut type_input).unwrap();
            let is_population = matches!(type_input.trim().to_lowercase().as_str(), "y" | "yes");

            let (variance_value, explanation) = stats::variance(&data, is_population);
            let label = if is_population { "Population" } else { "Sample" };
            
            println!("{label} Variance: {:.4}", variance_value);
            println!("Market Interpretation: {}", explanation);
            
        }

        "5" => {
            let data = read_and_parse_input("values");

            println!("Is this population data? (y/n):");
            let mut type_input = String::new();
            io::stdin().read_line(&mut type_input).unwrap();
            let is_population = matches!(type_input.trim().to_lowercase().as_str(), "y" | "yes");

            let (std, note) = stats::std(&data, is_population);
            let label = if is_population { "Population" } else { "Sample" };

            println!("{label} Standard Deviation (STD): {:.4} => {}", std, note);
        }


        "6" => {
            let prices = read_and_parse_input("prices");
            let volumes = read_and_parse_input("volumes");

            if prices.len() != volumes.len() {
                println!("Prices and volumes must be of the same length.");
                return;
            }

            println!("Is this population data? (y/n):");
            let mut type_input = String::new();
            io::stdin().read_line(&mut type_input).unwrap();
            let is_population = matches!(type_input.trim().to_lowercase().as_str(), "y" | "yes");

            let (variance, note) = stats::variance_vwap(&prices, &volumes, is_population);
            let label = if is_population { "Population" } else { "Sample" };

            println!("{label} VWAP Variance: {:.4} => {}", variance, note);
        }

        "7" => {
            let prices = read_and_parse_input("prices");
            let volumes = read_and_parse_input("volumes");

            if prices.len() != volumes.len() {
                println!("Prices and volumes must be of the same length.");
                return;
            }

            println!("Is this population data? (y/n):");
            let mut type_input = String::new();
            io::stdin().read_line(&mut type_input).unwrap();
            let is_population = matches!(type_input.trim().to_lowercase().as_str(), "y" | "yes");

            let (std, note) = stats::std_vwap(&prices, &volumes, is_population);
            let label = if is_population { "Population" } else { "Sample" };

            println!("{label} VWAP Standard Deviation: {:.4} => {}", std, note);
        }


        "8" => {
            let bids = read_and_parse_input("bid prices");
            let asks = read_and_parse_input("ask prices");
            let volumes = read_and_parse_input("volumes");

            if bids.len() != asks.len() || asks.len() != volumes.len() {
                println!("Inputs must be the same length.");
                return;
            }

            println!("Is this population data? (y/n):");
            let mut pop_input = String::new();
            io::stdin().read_line(&mut pop_input).unwrap();
            let is_population = matches!(pop_input.trim().to_lowercase().as_str(), "y" | "yes");

            let (var, note) = stats::variance_vwap_group(&bids, &asks, &volumes, is_population);
            println!("VWAP Group Variance: {:.4} => {}", var, note);
        }

        "9" => {
            let bids = read_and_parse_input("bid prices");
            let asks = read_and_parse_input("ask prices");
            let volumes = read_and_parse_input("volumes");

            if bids.len() != asks.len() || asks.len() != volumes.len() {
                println!("Inputs must be the same length.");
                return;
            }

            println!("Is this population data? (y/n):");
            let mut pop_input = String::new();
            io::stdin().read_line(&mut pop_input).unwrap();
            let is_population = matches!(pop_input.trim().to_lowercase().as_str(), "y" | "yes");

            let (std, note) = stats::std_vwap_group(&bids, &asks, &volumes, is_population);
            println!("VWAP Group STD: {:.4} => {}", std, note);
        }

        "10" => {
            let bids = read_and_parse_input("bid prices (compra)");
            let asks = read_and_parse_input("ask prices (venda)");
            let volumes = read_and_parse_input("volumes");

            if bids.len() != asks.len() || asks.len() != volumes.len() {
                println!("All inputs must have the same length.");
                return;
            }

            stats::global_stats_summary(&bids, &asks, &volumes);
        }

        "11" => {
            println!("\nYou selected: Import CSV for VWAP Group Analysis.");
            println!("Your CSV should contain three columns:");
            println!(" 1. Bid prices (Compra)");
            println!(" 2. Ask prices (Venda)");
            println!(" 3. Volumes");
            println!("Each row should be in the format: bid,ask,volume");
            println!("Example: 10.2,10.5,1000");
            println!("Place your CSV file in the same directory as this program (where Cargo.toml is).");

            loop {
                println!("\nEnter CSV file path (e.g., data.csv), or type 'exit' to go back:");

                let mut path = String::new();
                std::io::stdin().read_line(&mut path).unwrap();
                let path = path.trim();

                if path.eq_ignore_ascii_case("exit") {
                    println!("Returning to main menu.");
                    break;
                }

                match analyze_csv(path) {
                    Ok(_) => break, // CSV processed successfully, exit loop
                    Err(err) => {
                        println!("Error: {}", err);
                        println!("Please try again.");
                    }
                }
            }
        }


        _ => println!("Invalid option."),
    }
}

