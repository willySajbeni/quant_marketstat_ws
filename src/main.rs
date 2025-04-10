mod stats; // <-- isso diz ao Rust: "existe um arquivo chamado stats.rs aqui"
mod csv_loader; // <-- isso diz ao Rust: "existe um arquivo chamado csv_loader.rs aqui"
mod analyze_csv; // <-- isso diz ao Rust: "existe um arquivo chamado analyze_csv.rs aqui"
mod pnl; // <-- isso diz ao Rust: "existe um arquivo chamado pnl.rs aqui"
use analyze_csv::analyze_csv;
use pnl::{AssetPosition, calculate_pnl};


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
    println!("2 - VWAP (Volume Weighted Average Price)");
    println!("3 - VWAP Group (Bid + Ask) / 2");
    println!("4 - Variance (Population or Sample)");
    println!("5 - Standard Deviation (Population or Sample)");
    println!("6 - VWAP Variance (Population or Sample)");
    println!("7 - VWAP Standard Deviation (Population or Sample)");
    println!("8 - VWAP Group Variance (Bid, Ask, Volume)");
    println!("9 - VWAP Group STD (Bid, Ask, Volume)");
    println!("10 - Profit & Loss Calculation (P&L Summary)");
    println!("11 - Full Market Stats Report (Global Summary)");
    println!("12 - Load data from CSV file (Bid, Ask, Volume columns)");
    



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
                println!("Prices and volumes must have the same length.");
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
            let buy_prices = read_and_parse_input("buy prices");
            let sell_prices = read_and_parse_input("sell prices");
            let volumes = read_and_parse_input("contracted volumes(amount)");
            let market_prices = read_and_parse_input("market prices(price at time of analysis)");
            let realized_volumes = read_and_parse_input("realized volumes(shares sold) (optional, press Enter to skip)");
            let costs = read_and_parse_input("additional costs(Brokerage, fees, emoluments, slippage, financing, taxes, etc.) (optional, press Enter to skip)");

            let len = buy_prices.len();

            if sell_prices.len() != len || volumes.len() != len || market_prices.len() != len {
                println!("All required inputs must have the same number of elements.");
                return;
            }

            for i in 0..len {
                let realized = realized_volumes.get(i).copied();
                let extra_costs = costs.get(i).copied().unwrap_or(0.0);

                let pos = AssetPosition {
                    asset_id: format!("Asset_{}", i + 1),
                    buy_price: buy_prices[i],
                    sell_price: sell_prices[i],
                    contracted_volume: volumes[i],
                    realized_volume: realized,
                    market_price: market_prices[i],
                    additional_costs: extra_costs,
                };

                let result = calculate_pnl(&pos);

                println!("\n--- P&L Result for {} ---", pos.asset_id);
                println!("Revenue       : {:.2}", result.revenue);
                println!("Cost          : {:.2}", result.cost);
                println!("Exposure      : {:.2}", result.exposure);
                println!("Total P&L     : {:.2}", result.pnl);
            }
        }


        "11" => {
            let bids = read_and_parse_input("bid prices (compra)");
            let asks = read_and_parse_input("ask prices (venda)");
            let volumes = read_and_parse_input("volumes");

            if bids.len() != asks.len() || asks.len() != volumes.len() {
                println!("All inputs must have the same length.");
                return;
            }

            stats::global_stats_summary(&bids, &asks, &volumes);
        }

        "12" => {
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

