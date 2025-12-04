mod common;

use common::create_test_client;
use rs_clob_client::types::{Event, EventParams};

#[tokio::test]
async fn test_get_events() {
    let client = create_test_client();

    let params = EventParams {
        limit: Some(5),
        offset: None,
        tag_id: None,
        closed: Some(false),
        order: Some("id".to_string()),
        ascending: Some(true),
    };

    // Get first page of sampling markets
    let events: Vec<Event> = client
        .get_events(params)
        .await
        .expect("Failed to fetch events");

    assert!(events.len() > 0, "Should have at least one event");

    println!("=== Events ===");
    println!("Number of events: {}", events.len());
    println!("\nFirst 3 events:");

    for (i, event) in events.iter().take(3).enumerate() {
        println!("\n{}. Event:", i + 1);
        println!("{}", serde_json::to_string_pretty(event).unwrap());
    }
}

#[tokio::test]
async fn test_get_event_by_id() {
    let client = create_test_client();

    // Get first page of sampling markets
    let event: Event = client
        .get_events_by_id("23656")
        .await
        .expect("Failed to fetch events");

    assert!(!event.id.is_empty(), "Event ID should not be empty");

    println!("=== Event ===");
    println!("{}", serde_json::to_string_pretty(&event).unwrap());
}

#[tokio::test]
async fn test_get_event_by_slug() {
    let client = create_test_client();

    // Get first page of sampling markets
    let event: Event = client
        .get_event_by_slug("ethereum-above-on-december-4")
        .await
        .expect("Failed to fetch events");

    assert!(!event.id.is_empty(), "Event ID should not be empty");

    println!("=== Event ===");
    println!("{}", serde_json::to_string_pretty(&event).unwrap());
}