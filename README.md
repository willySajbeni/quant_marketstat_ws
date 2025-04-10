# 🦀📊 MarketStat — High-Performance Financial Stats Engine in Rust

**MarketStat** (a.k.a. `quant_marketstat_ws`) is a blazing-fast, quantitative financial analytics CLI engine, crafted in Rust by **Willy Sajbeni**.

Ideal for traders, quants, and data scientists who need low-latency, high-integrity insights extracted from price/volume data.

## Quick Install & Run (Requires [Rust](https://www.rust-lang.org/tools/install))

```bash
cargo install quant_marketstat_ws
quant_marketstat_ws
```
---

## Features

- ✔️ Arithmetic Mean  
- ✔️ VWAP (Volume Weighted Average Price)  
- ✔️ VWAP Group (Bid/Ask average)  
- ✔️ Variance (Sample & Population)  
- ✔️ Standard Deviation (STD)  
- ✔️ VWAP Variance & VWAP STD  
- ✔️ Group VWAP Variance & STD
- ✔️ Profit & Loss Calculation (P&L Summary)  
- 📈 Global Summary with human-readable interpretations  
- 📂 CSV Data Import (Option 11)  
- 🦀 Built in pure Rust — fast, safe, and lightweight

---

## CLI Menu Preview

```
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
10 - Profit & Loss Calculation (P&L Summary)
11 - Full Market Stats Report (Global Summary)
12 - Load data from CSV (Bid, Ask, Volume)
```

---

## CSV Format Example (for Option 11)

Your CSV file must follow this format:

```csv
Bid,Ask,Volume
10.5,11.0,1000
10.6,11.1,1500
10.4,10.9,1200
```

**Instructions:**

- Save your file as `data.csv`
- Place it in the **same folder where you run** `cargo run`
- When prompted, just type:  
  ```
  data.csv
  ```
- You can type `x` to cancel if you change your mind
- The system loops until a valid file is found


## License

MIT License — built and maintained by [**Willy Sajbeni**](https://github.com/willySajbeni)

---

## Why Rust over Python?

Because speed, safety, and clarity matter.  
This crab 🦀 just decapitated the snake 🐍 — **with both claws!**

---

## Coming Soon

- 📡 Yahoo Finance real-time fetch (optional)
- 📊 Graphs and interactive charts via TUI
- 📦 Binary executables (.exe, .app, ELF) for all platforms

---

## Author

Developed with love and mathematical rigor by **Willy Sajbeni**  
Follow on [LinkedIn](https://linkedin.com/in/willysajbeni) | [GitHub](https://github.com/willySajbeni)

> *Quantitative AI & Algorithmic Research | Linux, Rust, Python & Mathematical Modeling for Quant Trading*
