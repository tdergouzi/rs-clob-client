mod common;
use common::create_test_client;
use rs_clob_client::types::{Tag, TagParams};

#[tokio::test]
async fn test_get_tags() {
    let client = create_test_client();

    let params = TagParams {
        limit: Some(2),
        offset: None,
        order: None,
        ascending: None,
    };

    // Get first page of sampling markets
    let tags: Vec<Tag> = client.get_tags(params).await.expect("Failed to fetch tags");

    assert!(tags.len() > 0, "Should have at least one tag");

    println!("=== Tags ===");
    println!("Number of tags: {}", tags.len());
    println!("\nFirst 3 tags:");

    for (i, tag) in tags.iter().take(3).enumerate() {
        println!("\n{}. Tag:", i + 1);
        println!("{}", serde_json::to_string_pretty(tag).unwrap());
    }
}

#[tokio::test]
async fn test_get_tag_by_slug() {
    let client = create_test_client();

    let tag: Tag = client
        .get_tag_by_slug("elections")
        .await
        .expect("Failed to fetch tag");

    assert!(!tag.id.is_empty(), "Tag ID should not be empty");

    println!("=== Tag ===");
    println!("{}", serde_json::to_string_pretty(&tag).unwrap());
}

#[tokio::test]
async fn test_get_popular_tags() {
    let client = create_test_client();

    let tags: Vec<Tag> = client
        .get_popular_tags()
        .await
        .expect("Failed to fetch popular tags");

    for (i, tag) in tags.iter().take(5).enumerate() {
        println!("\n{}. Tag:", i + 1);
        println!("{}", serde_json::to_string_pretty(tag).unwrap());
    }
}
