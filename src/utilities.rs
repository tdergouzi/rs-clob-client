use crate::types::{OrderBookSummary, TickSize};
use sha2::{Digest, Sha256};

/// Validates if a price is valid for the given tick size
pub fn price_valid(price: f64, tick_size: TickSize) -> bool {
    let tick = tick_size.as_f64();
    let min = tick;
    let max = 1.0 - tick;
    
    price >= min && price <= max
}

/// Checks if one tick size is smaller than another
pub fn is_tick_size_smaller(tick_size: TickSize, min_tick_size: TickSize) -> bool {
    tick_size.as_f64() < min_tick_size.as_f64()
}

/// Generates a hash for the orderbook summary
pub fn generate_orderbook_summary_hash(orderbook: &OrderBookSummary) -> String {
    let mut hasher = Sha256::new();
    
    // Hash market and asset_id
    hasher.update(orderbook.market.as_bytes());
    hasher.update(orderbook.asset_id.as_bytes());
    
    // Hash bids
    for bid in &orderbook.bids {
        hasher.update(bid.price.as_bytes());
        hasher.update(bid.size.as_bytes());
    }
    
    // Hash asks
    for ask in &orderbook.asks {
        hasher.update(ask.price.as_bytes());
        hasher.update(ask.size.as_bytes());
    }
    
    let result = hasher.finalize();
    hex::encode(result)
}

/// Parse a string tick size to TickSize enum
pub fn parse_tick_size(tick_size: &str) -> Option<TickSize> {
    match tick_size {
        "0.1" => Some(TickSize::ZeroPointOne),
        "0.01" => Some(TickSize::ZeroPointZeroOne),
        "0.001" => Some(TickSize::ZeroPointZeroZeroOne),
        "0.0001" => Some(TickSize::ZeroPointZeroZeroZeroOne),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_valid() {
        assert!(price_valid(0.5, TickSize::ZeroPointZeroOne));
        assert!(price_valid(0.01, TickSize::ZeroPointZeroOne));
        assert!(price_valid(0.99, TickSize::ZeroPointZeroOne));
        
        assert!(!price_valid(0.005, TickSize::ZeroPointZeroOne));
        assert!(!price_valid(1.0, TickSize::ZeroPointZeroOne));
        assert!(!price_valid(0.0, TickSize::ZeroPointZeroOne));
    }

    #[test]
    fn test_is_tick_size_smaller() {
        assert!(is_tick_size_smaller(
            TickSize::ZeroPointZeroOne,
            TickSize::ZeroPointOne
        ));
        assert!(!is_tick_size_smaller(
            TickSize::ZeroPointOne,
            TickSize::ZeroPointZeroOne
        ));
    }

    #[test]
    fn test_parse_tick_size() {
        assert_eq!(parse_tick_size("0.1"), Some(TickSize::ZeroPointOne));
        assert_eq!(parse_tick_size("0.01"), Some(TickSize::ZeroPointZeroOne));
        assert_eq!(parse_tick_size("invalid"), None);
    }
}

