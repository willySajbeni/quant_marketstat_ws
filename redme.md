# 🦀📊 MarketStat — High-Performance Financial Stats Engine in Rust

**MarketStat** (a.k.a. `quant_marketstat_ws`) is a blazing-fast, quantitative financial analytics CLI engine, crafted in Rust by **Willy Sajbeni**.

Ideal for traders, quants, and data scientists who want low-latency, high-integrity insights from price/volume data.

---

## 🚀 Features

- Arithmetic Mean
- VWAP (Volume Weighted Average Price)
- VWAP Group (Bid/Ask average)
- Variance (Sample & Population)
- Standard Deviation (STD)
- VWAP Variance / STD
- Group VWAP Variance / STD
- 📈 Full Global Summary with human-readable insights
- 📂 CSV Import (Option 11)

---

## 🧪 CLI Menu Preview

Choose a function: 
1 - Mean 
2 - VWAP 
3 - VWAP Group 
4 - Variance 
5 - STD 
6 - VWAP Variance 
7 - VWAP STD 
8 - VWAP Group Variance 
9 - VWAP Group STD 
10 - Full Market Stats Report (Global Summary) 
11 - Load data from CSV (Bid, Ask, Volume)

---

## 🧾 CSV Format Example (Option 11)

Your CSV should look like this:
Bid,Ask,Volume
10.5,11.0,1000
10.6,11.1,1500
...


- Keep the file (e.g. `data.csv`) in the **same directory** where you run `cargo run`
- If the path is invalid, the program prompts again (or enter `x` to exit)

---

## 🛠️ Installation & Run - Requires Rust installed.

```bash
git clone https://github.com/willysajbeni/marketstat.git
cd marketstat
cargo run



# 📊 MarketStat: Rust-based Financial Statistical Analysis

**MarketStat** is a high-performance financial statistics engine written in Rust. It allows traders, data scientists and finance enthusiasts to compute key market indicators such as:

- Arithmetic Mean
- VWAP (Volume Weighted Average Price)
- VWAP Group (Bid/Ask)
- Variance (Sample & Population)
- Standard Deviation (STD)
- VWAP Variance / STD
- Group VWAP Variance / STD
- 📈 Plus: Full Global Summary with insights

---

## Features

- Identify strong/weak markets from VWAP signals
- Detect market volatility using STD/Variance
- Works with real-time data (soon via Yahoo Finance)
- Easily extensible, fast and accurate
- Outputs include human-readable interpretations

---

## You will be prompted with a CLI menu like this:

Choose a function:
1 - Mean
2 - VWAP
3 - VWAP Group
4 - Variance
5 - STD
6 - VWAP Variance
7 - VWAP STD
8 - VWAP Group Variance
9 - VWAP Group STD
10 - Full Market Stats Report (Global Summary)

## Just enter comma-separated values like:
Enter your prices separated by commas:
120.4, 121.3, 122.5

## CSV Format
Bid,Ask,Volume
10.5,11.0,1000
10.6,11.1,1500
...



## IInstalação

git clone https://github.com/willysajbeni/marketstat.git
cd marketstat
cargo run

# Necessário: Rust instalado
