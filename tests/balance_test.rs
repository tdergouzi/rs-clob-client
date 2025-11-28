mod common;

use common::{create_test_client_with_api_key};
use rs_clob_client::types::markets::BalanceAllowanceParams;
use rs_clob_client::types::primitives::AssetType;

#[tokio::test]
async fn test_get_balance_allowance() {
    let client = create_test_client_with_api_key(0);

    let result = client
        .get_balance_allowance(BalanceAllowanceParams {
            asset_type: AssetType::Collateral,
            token_id: None,
        })
        .await
        .expect("Failed to get balance allowance");

    println!(
        "=== Balance Allowance ===\n{}",
        serde_json::to_string_pretty(&result).unwrap()
    );
}
