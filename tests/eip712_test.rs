use alloy_signer_local::PrivateKeySigner;
use rs_clob_client::signing::build_clob_eip712_signature;

#[tokio::test]
async fn test_build_clob_eip712_signature() {
    let pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(); // Test case from Typescript SDK
    let wallet: PrivateKeySigner = pk.parse().expect("Invalid private key");
    println!("Wallet address: {}", wallet.address());

    let chain_id: u64 = 80002;
    let timestamp: u64 = 10000000;
    let nonce: u64 = 23;

    let signature = build_clob_eip712_signature(&wallet, chain_id, timestamp, nonce)
        .await
        .expect("Failed to build EIP-712 signature");

    assert_eq!(signature, "0xf62319a987514da40e57e2f4d7529f7bac38f0355bd88bb5adbb3768d80de6c1682518e0af677d5260366425f4361e7b70c25ae232aff0ab2331e2b164a1aedc1b");
    // println!("Signature: {}", signature);
}
