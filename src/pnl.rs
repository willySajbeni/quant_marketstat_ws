/// Module for calculating generic Profit and Loss (P&L)
/// for any type of traded asset: FX, stocks, futures, etc.
/// Debug ->	Permite que você use {:?} no println! pra imprimir a struct
/// Clone ->	Permite criar uma cópia exata da struct (let copy = original.clone();)
#[derive(Debug, Clone)]
pub struct AssetPosition {
    /// Unique identifier for the asset (e.g. "BTC/USD", "AAPL", "EUR/BRL")
    pub asset_id: String,

    /// Price at which the asset was bought (entry price)
    pub buy_price: f64,

    /// Price at which the asset was sold (exit price or fixed forward price)
    pub sell_price: f64,

    /// Agreed or traded volume (in units, lots, contracts, etc.)
    pub contracted_volume: f64,

    /// Realized volume (actual volume traded or delivered). Optional.
    pub realized_volume: Option<f64>,

    /// Market price used for valuing deviations (e.g. spot price, mark-to-market)
    pub market_price: f64,

    /// Extra costs: commissions, fees, slippage, taxes. Optional.
    pub additional_costs: f64,
}

/// Struct to hold the result of a P&L calculation
#[derive(Debug, Clone)]
pub struct PnLResult {
    pub revenue: f64,
    pub cost: f64,
    pub exposure: f64,
    pub pnl: f64,
}

/// Calculates Profit & Loss (P&L) for a given asset position, in a generic way.
///
/// # Formula:
/// ```text
/// Revenue  = sell_price × contracted_volume(amount)
/// Cost     = buy_price × contracted_volume(amount)
/// Exposure = (realized_volume(amount) - contracted_volume(amount)) × market_price
/// P&L      = Revenue - Cost - Exposure - Additional Costs
/// ```
///
/// If `realized_volume` is not provided, exposure is assumed to be zero.
///
/// # Example:
/// ```
/// let position = AssetPosition {
///     asset_id: "EUR/USD".to_string(),
///     buy_price: 1.08,
///     sell_price: 1.11,
///     contracted_volume: 1_000_000.0,
///     realized_volume: Some(1_020_000.0),
///     market_price: 1.10,
///     additional_costs: 1500.0,
/// };
///
/// let result = calculate_pnl(&position);
/// println!("{:?}", result);
/// ```
pub fn calculate_pnl(position: &AssetPosition) -> PnLResult {
    let revenue = position.sell_price * position.contracted_volume;
    let cost = position.buy_price * position.contracted_volume;

    let exposure = match position.realized_volume {
        Some(realized) => (realized - position.contracted_volume) * position.market_price,
        None => 0.0,
    };

    let pnl = revenue - cost - exposure - position.additional_costs;

    PnLResult {
        revenue,
        cost,
        exposure,
        pnl,
    }
}
