// Order helpers - market price calculation

use crate::errors::{ClobError, ClobResult};
use crate::types::{OrderSummary, OrderType};

/// Calculates the execution price for a market buy order
///
/// Walks through the ask side of the orderbook and calculates
/// the worst price needed to fill the order.
///
/// # Arguments
///
/// * `asks` - Ask side of orderbook (sorted by price ascending)
/// * `amount` - Amount in USDC to buy
/// * `order_type` - FOK (must fill completely) or FAK (partial ok)
pub fn calculate_buy_market_price(
    asks: &[OrderSummary],
    amount: f64,
    order_type: OrderType,
) -> ClobResult<f64> {
    if asks.is_empty() {
        return Err(ClobError::NoMatch);
    }

    let mut remaining = amount;
    let mut worst_price = 0.0;

    for ask in asks {
        let price: f64 = ask.price.parse()
            .map_err(|_| ClobError::Other("Invalid price in orderbook".to_string()))?;
        let size: f64 = ask.size.parse()
            .map_err(|_| ClobError::Other("Invalid size in orderbook".to_string()))?;

        let order_value = price * size;

        if remaining <= order_value {
            // This level can fill the remaining amount
            worst_price = price;
            remaining = 0.0;
            break;
        }

        // Take the entire level
        worst_price = price;
        remaining -= order_value;
    }

    // Check if we could fill the order
    if remaining > 0.0 {
        match order_type {
            OrderType::Fok => {
                // FOK requires complete fill
                return Err(ClobError::NoMatch);
            }
            OrderType::Fak => {
                // FAK accepts partial fill, return the worst price we reached
                if worst_price == 0.0 {
                    return Err(ClobError::NoMatch);
                }
            }
            _ => {}
        }
    }

    // Add small buffer to ensure execution (0.5%)
    let buffered_price = worst_price * 1.005;

    // Cap at 0.99 (maximum valid price)
    Ok(buffered_price.min(0.99))
}

/// Calculates the execution price for a market sell order
///
/// Walks through the bid side of the orderbook and calculates
/// the worst price needed to fill the order.
///
/// # Arguments
///
/// * `bids` - Bid side of orderbook (sorted by price descending)
/// * `amount` - Amount in tokens to sell
/// * `order_type` - FOK (must fill completely) or FAK (partial ok)
pub fn calculate_sell_market_price(
    bids: &[OrderSummary],
    amount: f64,
    order_type: OrderType,
) -> ClobResult<f64> {
    if bids.is_empty() {
        return Err(ClobError::NoMatch);
    }

    let mut remaining = amount;
    let mut worst_price = 0.0;

    for bid in bids {
        let price: f64 = bid.price.parse()
            .map_err(|_| ClobError::Other("Invalid price in orderbook".to_string()))?;
        let size: f64 = bid.size.parse()
            .map_err(|_| ClobError::Other("Invalid size in orderbook".to_string()))?;

        if remaining <= size {
            // This level can fill the remaining amount
            worst_price = price;
            remaining = 0.0;
            break;
        }

        // Take the entire level
        worst_price = price;
        remaining -= size;
    }

    // Check if we could fill the order
    if remaining > 0.0 {
        match order_type {
            OrderType::Fok => {
                // FOK requires complete fill
                return Err(ClobError::NoMatch);
            }
            OrderType::Fak => {
                // FAK accepts partial fill
                if worst_price == 0.0 {
                    return Err(ClobError::NoMatch);
                }
            }
            _ => {}
        }
    }

    // Subtract small buffer to ensure execution (0.5%)
    let buffered_price = worst_price * 0.995;

    // Floor at 0.01 (minimum valid price)
    Ok(buffered_price.max(0.01))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_buy_market_price() {
        let asks = vec![
            OrderSummary {
                price: "0.50".to_string(),
                size: "100".to_string(),
            },
            OrderSummary {
                price: "0.51".to_string(),
                size: "100".to_string(),
            },
        ];

        // Buy 25 USDC worth (50 tokens at 0.50)
        let price = calculate_buy_market_price(&asks, 25.0, OrderType::Fok).unwrap();
        assert!(price > 0.50 && price <= 0.52); // 0.50 * 1.005 buffer
    }

    #[test]
    fn test_calculate_sell_market_price() {
        let bids = vec![
            OrderSummary {
                price: "0.50".to_string(),
                size: "100".to_string(),
            },
            OrderSummary {
                price: "0.49".to_string(),
                size: "100".to_string(),
            },
        ];

        // Sell 50 tokens
        let price = calculate_sell_market_price(&bids, 50.0, OrderType::Fok).unwrap();
        assert!(price < 0.50 && price >= 0.48); // 0.50 * 0.995 buffer
    }

    #[test]
    fn test_fok_fails_on_insufficient_liquidity() {
        let asks = vec![
            OrderSummary {
                price: "0.50".to_string(),
                size: "10".to_string(), // Only 5 USDC worth
            },
        ];

        // Try to buy 100 USDC worth - should fail with FOK
        let result = calculate_buy_market_price(&asks, 100.0, OrderType::Fok);
        assert!(result.is_err());
    }

    #[test]
    fn test_fak_accepts_partial_fill() {
        let asks = vec![
            OrderSummary {
                price: "0.50".to_string(),
                size: "10".to_string(), // Only 5 USDC worth
            },
        ];

        // Try to buy 100 USDC worth with FAK - should succeed with partial
        let result = calculate_buy_market_price(&asks, 100.0, OrderType::Fak);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_orderbook() {
        let asks: Vec<OrderSummary> = vec![];
        let result = calculate_buy_market_price(&asks, 10.0, OrderType::Fok);
        assert!(matches!(result, Err(ClobError::NoMatch)));
    }
}

