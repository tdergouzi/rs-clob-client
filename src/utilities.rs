use crate::types::{OrderBookSummary, TickSize};
use sha1::{Digest, Sha1};

/// Round to nearest value with specified decimal places.
/// Always applies rounding to avoid floating point precision issues.
pub fn round_normal(num: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (num * multiplier).round() / multiplier
}

/// Round down (floor) to specified decimal places.
/// Always applies rounding to avoid floating point precision issues.
pub fn round_down(num: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (num * multiplier).floor() / multiplier
}

/// Round up (ceil) to specified decimal places.
/// Always applies rounding to avoid floating point precision issues.
pub fn round_up(num: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (num * multiplier).ceil() / multiplier
}

pub fn decimal_places(num: f64) -> u32 {
    if num.fract() == 0.0 {
        return 0;
    }

    let s = format!("{}", num);
    if let Some(pos) = s.find('.') {
        (s.len() - pos - 1) as u32
    } else {
        0
    }
}

pub fn generate_orderbook_summary_hash(orderbook: &mut OrderBookSummary) -> String {
    orderbook.hash = String::new();
    let json = serde_json::to_string(orderbook).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(json.as_bytes());
    let result = hasher.finalize();
    let hash = hex::encode(result);
    orderbook.hash = hash.clone();
    hash
}

pub fn is_tick_size_smaller(a: TickSize, b: TickSize) -> bool {
    a.as_f64() < b.as_f64()
}

pub fn price_valid(price: f64, tick_size: TickSize) -> bool {
    let tick = tick_size.as_f64();
    price >= tick && price <= 1.0 - tick
}

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
    fn test_round_normal() {
        assert_eq!(round_normal(0.555, 2), 0.56);
        assert_eq!(round_normal(0.554, 2), 0.55);
        assert_eq!(round_normal(0.5, 2), 0.5);
    }

    #[test]
    fn test_round_down() {
        assert_eq!(round_down(0.559, 2), 0.55);
        assert_eq!(round_down(0.551, 2), 0.55);
        assert_eq!(round_down(0.5, 2), 0.5);
    }

    #[test]
    fn test_round_up() {
        assert_eq!(round_up(0.551, 2), 0.56);
        assert_eq!(round_up(0.559, 2), 0.56);
        assert_eq!(round_up(0.5, 2), 0.5);
    }

    #[test]
    fn test_decimal_places() {
        assert_eq!(decimal_places(0.5), 1);
        assert_eq!(decimal_places(0.55), 2);
        assert_eq!(decimal_places(0.555), 3);
        assert_eq!(decimal_places(5.0), 0);
        assert_eq!(decimal_places(0.0), 0);
    }

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
