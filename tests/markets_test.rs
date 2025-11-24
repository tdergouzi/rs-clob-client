use rs_clob_client::types::markets::Market;
use std::fs;

#[test]
fn test_deserialize_market() {
    let json_data = fs::read_to_string("data/market.json").expect("Failed to read market.json");
    let markets: Vec<Market> = serde_json::from_str(&json_data).expect("Failed to deserialize market data");
    
    assert!(!markets.is_empty());
    
    let market = &markets[0];
    assert_eq!(market.id, "690560");
    assert_eq!(market.question.as_ref().unwrap(), "Will Trump release the Epstein files by December 31?");
    assert_eq!(market.condition_id.as_ref().unwrap(), "0x4048fed324ac27f378ce44da1b12f0d338c8340ef82962989f38eea05409baab");
    assert_eq!(market.slug.as_ref().unwrap(), "will-trump-release-the-epstein-files-by-december-31");
    assert_eq!(market.active, Some(true));
    assert_eq!(market.closed, Some(false));
    assert!(market.volume_num.is_some());
    assert!(market.liquidity_num.is_some());
    
    // Check nested events
    assert!(market.events.is_some());
    let events = market.events.as_ref().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, "84898");
    
    // Check volume fields (using approximate comparison for f64)
    assert!((market.volume_num.unwrap() - 534148.650211).abs() < 0.001);
    assert!((market.volume24hr.unwrap() - 27342.475169).abs() < 0.001);
    assert!((market.volume1wk.unwrap() - 532940.737918).abs() < 0.001);
    assert!((market.volume1mo.unwrap() - 532940.737918).abs() < 0.001);
    assert!((market.volume1yr.unwrap() - 532940.737918).abs() < 0.001);
    assert!((market.volume24hr_clob.unwrap() - 27342.475169).abs() < 0.001);
    assert!((market.volume1wk_clob.unwrap() - 532940.737918).abs() < 0.001);
    assert!((market.volume1mo_clob.unwrap() - 532940.737918).abs() < 0.001);
    assert!((market.volume1yr_clob.unwrap() - 532940.737918).abs() < 0.001);
    assert!((market.volume_clob.unwrap() - 534148.650211).abs() < 0.001);
    
    println!("Market deserialization test passed!");
    println!("Market: {} - {}", market.id, market.question.as_ref().unwrap());
    println!("Volume (total): {:?}", market.volume_num);
    println!("Volume (24h): {:?}", market.volume24hr);
    println!("Volume (1wk): {:?}", market.volume1wk);
    println!("Liquidity: {:?}", market.liquidity_num);
}
