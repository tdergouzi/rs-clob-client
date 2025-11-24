mod common;

use rs_clob_client::types::Event;
use common::create_test_client;

#[tokio::test]
async fn test_get_events() {
    let client = create_test_client();

    // Get first page of sampling markets
    let events: Vec<Event> = client
        .get_events(Some(2))
        .await
        .expect("Failed to fetch events");

    assert!(
        events.len() > 0,
        "Should have at least one event"
    );

    println!("=== Events ===");
    println!("Number of events: {}", events.len());
    println!("\nFirst 3 events:");

    for (i, event) in events.iter().take(3).enumerate() {
        println!("\n{}. Event:", i + 1);
        println!("  ID: {}", event.id);
        println!("  Title: {}", event.title);
        println!("  Category: {}", event.category);
        println!("  Volume: {}", event.volume);
        println!("  Liquidity: {}", event.liquidity);
        println!("  Active: {}", event.active);
        println!("  Closed: {}", event.closed);
    }
}
