// MEAN ---------------------------------------------------------------------
/// Calculates the arithmetic mean (average) of a given list of values.
///
/// # Formula
/// The formula for the mean of a set of values is:
///
/// $$
/// \text{Mean} = \frac{1}{n} \sum_{i=1}^{n} x_i
/// $$
///
/// # Arguments
/// * `values` - A slice of `f64` values representing the dataset.
///
/// # Returns
/// * `f64` - The arithmetic mean.
///
/// # Example
/// ```
/// let data = vec![10.0, 20.0, 30.0];
/// let avg = stats::mean(&data);
/// assert_eq!(avg, 20.0);
/// ```
pub fn mean(values: &[f64]) -> f64 {
    let sum: f64 = values.iter().sum();
    let count = values.len();

    if count == 0 {
        return 0.0;
    }

    sum / count as f64
}
//-------------------------------------------------------------------------------


//VWAP Simple -------------------------------------------------------------------
/// Calculates the Volume Weighted Average Price (VWAP) and gives a market signal.
///
/// # Formula
/// The VWAP is calculated as:
/// $$
/// VWAP = \frac{\sum (P_i \times V_i)}{\sum V_i}
/// $$
///
/// # Market Signal Logic
/// - If `VWAP > last_price` → Market is weak (trading below average)
/// - If `VWAP < last_price` → Market is strong (trading above average)
///
/// # Arguments
/// * `prices` - A slice of f64 representing traded prices.
/// * `volumes` - A slice of f64 representing the volume at each price.
///
/// # Returns
/// * `(f64, &'static str)` - Tuple containing VWAP and signal string.
///
/// # Example
/// ```
/// let prices = vec![10.0, 11.0, 12.0];
/// let volumes = vec![100.0, 200.0, 150.0];
/// let (vwap, signal) = stats::vwap(&prices, &volumes);
/// ```
pub fn vwap(prices: &[f64], volumes: &[f64]) -> (f64, &'static str) {
    if prices.len() != volumes.len() || prices.is_empty() {
        return (0.0, "Invalid input: prices and volumes must be the same length and not empty.");
    }

    let total_value: f64 = prices.iter()
        .zip(volumes.iter())
        .map(|(p, v)| p * v)
        .sum();

    let total_volume: f64 = volumes.iter().sum();

    if total_volume == 0.0 {
        return (0.0, "Total volume is zero. Cannot calculate VWAP.");
    }

    let vwap = total_value / total_volume;
    let last_price = *prices.last().unwrap();

    let signal = if vwap > last_price {
        "VWAP is above the current price → weak market (trading below the day's average value)"
    } else if vwap < last_price {
        "VWAP is below the current price → strong market (trading above the day's average value)"
    } else {
        "VWAP is equal to the current price → neutral market"
    };

    (vwap, signal)
}
//-------------------------------------------------------------------------------------------


//VWAP Group---------------------------------------------------------------------------------
/// Calculates VWAP based on the average between bid and ask prices and returns a market signal.
///
/// # Formula
/// For each price point:  
/// $$
/// Preço\_Médio_i = \frac{Compra_i + Venda_i}{2}
/// $$  
/// Then apply:
/// $$
/// VWAP = \frac{\sum (Preço\_Médio_i \times Volume_i)}{\sum Volume_i}
/// $$
///
/// # Signal
/// - VWAP > Último Preço de Venda → Mercado Fraco  
/// - VWAP < Último Preço de Venda → Mercado Forte
///
/// # Returns
/// * `(f64, &'static str)` - VWAP calculado e sinal textual.
///
/// # Panics
/// Se os slices tiverem tamanhos diferentes.
pub fn vwap_group(
    precos_compra: &[f64],
    precos_venda: &[f64],
    volumes: &[f64],
) -> (f64, &'static str) {
    if precos_compra.len() != precos_venda.len() || precos_compra.len() != volumes.len() {
        return (
            0.0,
            "Invalid input: all input slices must have the same length and not be empty.",
        );
    }

    let precos_medios: Vec<f64> = precos_compra
        .iter()
        .zip(precos_venda.iter())
        .map(|(b, a)| (b + a) / 2.0)
        .collect();

    let total_valor: f64 = precos_medios
        .iter()
        .zip(volumes.iter())
        .map(|(pm, v)| pm * v)
        .sum();

    let total_volume: f64 = volumes.iter().sum();

    if total_volume == 0.0 {
        return (0.0, "Total volume is zero. Cannot calculate VWAP Group.");
    }

    let vwap = total_valor / total_volume;
    let ultimo_preco_venda = *precos_venda.last().unwrap();

    let sinal = if vwap > ultimo_preco_venda {
        "VWAP is above the current price → weak market (trading below the day's average value)"
    } else if vwap < ultimo_preco_venda {
        "VWAP is below the current price → strong market (trading above the day's average value)"
    } else {
        "VWAP is equal to the current price → neutral market"
    };

    (vwap, sinal)
}
//---------------------------------------------------------------------------------------------------------


/// Calculates the variance of a dataset and returns an interpretation string.
/// 
/// # Arguments
/// * `data` - A slice of f64 values representing the dataset.
/// * `is_population` - A boolean indicating whether to use population variance or sample variance.
///
/// # Returns
/// A tuple `(variance_value, explanation)` where:
/// - `variance_value`: f64, the calculated variance.
/// - `explanation`: &'static str, interpretation based on the variance level.
pub fn variance(data: &[f64], is_population: bool) -> (f64, &'static str) {
    if data.is_empty() {
        return (0.0, "No data provided.");
    }

    let mean = mean(data);
    let squared_diffs: f64 = data
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum();

    let denominator = if is_population {
        data.len() as f64
    } else {
        (data.len().saturating_sub(1)) as f64
    };

    let variance = squared_diffs / denominator;

    // Interpretation of the variance value
    let explanation = if variance < 0.5 {
        "Low variance → stable market with low volatility"
    } else if variance < 2.0 {
        "Moderate variance → normal market fluctuations"
    } else {
        "High variance → volatile market with larger price swings"
    };

    (variance, explanation)
}
//-------------------------------------------------------------------------------------------------------

//STD for the Variance-----------------------------------------------------------------------------------
/// Calculates the standard deviation (population or sample) of a list of f64 values.
///
/// # Arguments
/// - `data`: slice of f64 values
/// - `is_population`: true for population standard deviation, false for sample
///
/// # Returns
/// - Standard deviation as `f64`
///
/// $$ \text{STD} = \sqrt{\text{Variance}} $$
pub fn std(data: &[f64], is_population: bool) -> (f64, &'static str) {
    let (variance_value, explanation) = variance(data, is_population);
    let std_dev = variance_value.sqrt();
    (std_dev, explanation)
}
//--------------------------------------------------------------------------------------------------------

//Variance VWAP-------------------------------------------------------------------------------------------
/// Calculates the variance of prices weighted by volumes around the VWAP.
/// 
/// # Arguments
/// - `prices`: slice of prices (f64)
/// - `volumes`: slice of volumes (f64)
/// - `is_population`: true if population variance, false for sample
/// 
/// # Returns
/// - Weighted variance around the VWAP
/// 
/// $$ \text{Variance} = \frac{\sum v_i(p_i - \text{VWAP})^2}{\sum v_i \text{ or } \sum v_i - 1} $$
/// Calculates the variance of prices weighted by volumes around the VWAP, with explanation.
/// 
/// # Returns
/// - (variance_value, explanation)
pub fn variance_vwap(prices: &[f64], volumes: &[f64], is_population: bool) -> (f64, &'static str) {
    let (vwap_value, _) = vwap(prices, volumes);
    let weighted_squared_diffs: f64 = prices.iter()
        .zip(volumes.iter())
        .map(|(p, v)| v * (p - vwap_value).powi(2))
        .sum();

    let total_volume: f64 = volumes.iter().sum();
    let variance = if is_population {
        weighted_squared_diffs / total_volume
    } else {
        let adjusted_total = total_volume - 1.0;
        if adjusted_total <= 0.0 {
            0.0
        } else {
            weighted_squared_diffs / adjusted_total
        }
    };

    // Explanation
    let explanation = if variance < 0.5 {
        "Low VWAP variance → stable volume-weighted market with small deviations from VWAP"
    } else if variance < 2.0 {
        "Moderate VWAP variance → typical market volume-weighted fluctuation"
    } else {
        "High VWAP variance → volatile market with large volume-weighted deviations from VWAP"
    };

    (variance, explanation)
}
//---------------------------------------------------------------------------------------------------------

//STD for the Variance VWAP--------------------------------------------------------------------------------
/// Calculates the standard deviation around the VWAP (weighted by volumes), with explanation.
/// 
/// # Arguments
/// - `prices`: slice of prices
/// - `volumes`: slice of volumes
/// - `is_population`: true for population STD, false for sample
/// 
/// # Returns
/// - Weighted standard deviation around VWAP
pub fn std_vwap(prices: &[f64], volumes: &[f64], is_population: bool) -> (f64, &'static str) {
    let (var, _) = variance_vwap(prices, volumes, is_population);
    let std = var.sqrt();

    // Explanation
    let explanation = if std < 0.7 {
        "Low VWAP STD → low dispersion of prices around VWAP"
    } else if std < 1.5 {
        "Moderate VWAP STD → normal dispersion of prices around VWAP"
    } else {
        "High VWAP STD → significant dispersion from VWAP, indicating higher risk"
    };

    (std, explanation)
}
//-----------------------------------------------------------------------------------------------------------

//Variance VWAP GRoup ---------------------------------------------------------------------------------------
/// Calculates the VWAP Group variance (bid + ask / 2), weighted by volume.
///
/// # Arguments
/// - `bids`: bid prices (buy side)
/// - `asks`: ask prices (sell side)
/// - `volumes`: traded volumes
/// - `is_population`: true for population variance, false for sample
///
/// # Returns
/// - (variance_value, interpretation)
pub fn variance_vwap_group(
    bids: &[f64],
    asks: &[f64],
    volumes: &[f64],
    is_population: bool,
) -> (f64, &'static str) {
    let mid_prices: Vec<f64> = bids.iter()
        .zip(asks.iter())
        .map(|(b, a)| (b + a) / 2.0)
        .collect();

    let (vwap_group, _) = vwap_group(bids, asks, volumes);

    let total_volume: f64 = volumes.iter().sum();
    let weighted_diffs: f64 = mid_prices.iter()
        .zip(volumes.iter())
        .map(|(p, v)| v * (p - vwap_group).powi(2))
        .sum();

    let variance = if is_population {
        weighted_diffs / total_volume
    } else {
        let adjusted = total_volume - 1.0;
        if adjusted <= 0.0 { 0.0 } else { weighted_diffs / adjusted }
    };

    let interpretation = if variance < 0.5 {
        "Low variance: stable market"
    } else if variance < 2.0 {
        "Medium variance: moderate activity"
    } else {
        "High variance: volatile market"
    };

    (variance, interpretation)
}
//-------------------------------------------------------------------------------------------------

// STD for thr Variance VWAP Group-----------------------------------------------------------------
/// Calculates standard deviation of VWAP Group using its variance.
///
/// # Arguments
/// - same as variance_vwap_group
///
/// # Returns
/// - (std_value, interpretation)
pub fn std_vwap_group(
    bids: &[f64],
    asks: &[f64],
    volumes: &[f64],
    is_population: bool,
) -> (f64, &'static str) {
    let (variance, _) = variance_vwap_group(bids, asks, volumes, is_population);
    let std = variance.sqrt();

    let interpretation = if std < 0.7 {
        "Low STD: low volatility"
    } else if std < 1.5 {
        "Moderate STD: watch the market"
    } else {
        "High STD: market unstable"
    };

    (std, interpretation)
}
//----------------------------------------------------------------------------------------------------


//Funcao Global with all fn---------------------------------------------------------------------------
/// Global market statistics summary based on bids, asks, and volumes.
/// Computes and interprets all relevant stats in one call.
pub fn global_stats_summary(bids: &[f64], asks: &[f64], volumes: &[f64]) {
    println!("\n📊 === Global Market Stats Summary ===\n");

    // Mean
    let mean_bid = mean(bids);
    let mean_ask = mean(asks);
    println!("Mean Bid: {:.4}", mean_bid);
    println!("Mean Ask: {:.4}", mean_ask);

    // VWAP
    let (vwap_bid, sig_bid) = vwap(bids, volumes);
    let (vwap_ask, sig_ask) = vwap(asks, volumes);
    println!("\nVWAP Bid: {:.4} => {}", vwap_bid, sig_bid);
    println!("VWAP Ask: {:.4} => {}", vwap_ask, sig_ask);

    // VWAP Group
    let (vwap_group, sig_group) = vwap_group(bids, asks, volumes);
    println!("VWAP Group: {:.4} => {}", vwap_group, sig_group);

    // Variance & STD - raw
    let (var_bid, note_var_bid) = variance(bids, true);
    let (var_ask, note_var_ask) = variance(asks, true);
    println!("\nVariance Bid: {:.4} => {}", var_bid, note_var_bid);
    println!("Variance Ask: {:.4} => {}", var_ask, note_var_ask);

    let (std_bid, note_std_bid) = std(bids, true);
    let (std_ask, note_std_ask) = std(asks, true);
    println!("STD Bid: {:.4} => {}", std_bid, note_std_bid);
    println!("STD Ask: {:.4} => {}", std_ask, note_std_ask);

    // VWAP Variance & STD
    let (var_vwap_bid, note_var_vwap_bid) = variance_vwap(bids, volumes, true);
    let (var_vwap_ask, note_var_vwap_ask) = variance_vwap(asks, volumes, true);
    println!("\nVWAP Variance Bid: {:.4} => {}", var_vwap_bid, note_var_vwap_bid);
    println!("VWAP Variance Ask: {:.4} => {}", var_vwap_ask, note_var_vwap_ask);

    let (std_vwap_bid, note_std_vwap_bid) = std_vwap(bids, volumes, true);
    let (std_vwap_ask, note_std_vwap_ask) = std_vwap(asks, volumes, true);
    println!("VWAP STD Bid: {:.4} => {}", std_vwap_bid, note_std_vwap_bid);
    println!("VWAP STD Ask: {:.4} => {}", std_vwap_ask, note_std_vwap_ask);

    // Variance & STD - VWAP Group
    let (var_vwap_group, note_var_group) = variance_vwap_group(bids, asks, volumes, true);
    let (std_vwap_group, note_std_group) = std_vwap_group(bids, asks, volumes, true);
    println!("\nVWAP Group Variance: {:.4} => {}", var_vwap_group, note_var_group);
    println!("VWAP Group STD: {:.4} => {}", std_vwap_group, note_std_group);

}


