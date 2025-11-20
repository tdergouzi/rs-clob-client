# rs-clob-client

🦀 Rust client for Polymarket's CLOB (Central Limit Order Book) API with full EIP-712 signing support.

This library provides a complete Rust port of the TypeScript `@polymarket/clob-client`, enabling you to interact with Polymarket's trading infrastructure for creating orders, managing API keys, fetching market data, and more.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs-clob-client = "0.1"
alloy-primitives = "0.8"
alloy-signer-local = "0.5"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Setup and Placing an Order

```rust
use rs_clob_client::{ClobClient, Chain, Side, OrderType, UserOrder};
use alloy_signer_local::PrivateKeySigner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a wallet from private key
    let private_key = "your_private_key_here";
    let wallet = private_key.parse::<PrivateKeySigner>()?;
    
    // Initialize the CLOB client
    let host = "https://clob.polymarket.com".to_string();
    let chain_id = Chain::Polygon; // 137
    
    // Create or derive API credentials
    let temp_client = ClobClient::new(
        host.clone(),
        chain_id,
        Some(wallet.clone()),
        None,
        None,
        None,
        None,
        false,
        None,
    );
    let creds = temp_client.create_or_derive_api_key().await?;
    
    // Create the main client with credentials
    let client = ClobClient::new(
        host,
        chain_id,
        Some(wallet),
        Some(creds),
        Some(1), // Signature type: 0 = EOA, 1 = Poly Proxy, 2 = EIP-1271
        None,    // Optional funder address
        None,    // Optional geo-block token
        false,   // Use server time
        None,    // Optional builder config
    );
    
    // Place a limit order
    let order = UserOrder {
        token_id: "your_token_id".to_string(),
        price: 0.52,
        size: 10.0,
        side: Side::Buy,
        fee_rate_bps: None,
        nonce: None,
        expiration: None,
    };
    
    let result = client.create_order(order, OrderType::Gtc).await?;
    println!("Order created: {:?}", result);
    
    Ok(())
}
```

### Market Data Access

```rust
use rs_clob_client::ClobClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Public endpoints don't require authentication
    let client = ClobClient::new(
        "https://clob.polymarket.com".to_string(),
        rs_clob_client::Chain::Polygon,
        None,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    
    // Get orderbook for a token
    let token_id = "your_token_id";
    let orderbook = client.get_orderbook(token_id).await?;
    println!("Orderbook: {:?}", orderbook);
    
    // Get current price
    let price = client.get_price(token_id, rs_clob_client::Side::Buy).await?;
    println!("Best buy price: {:?}", price);
    
    // Get spread
    let spread = client.get_spread(token_id).await?;
    println!("Spread: {:?}", spread);
    
    // Get recent trades
    let trades = client.get_trades(token_id).await?;
    println!("Recent trades: {:?}", trades);
    
    Ok(())
}
```

### Managing Orders

```rust
// Cancel a specific order
let order_id = "your_order_id";
client.cancel_order(order_id).await?;

// Cancel all orders for a market
let token_id = "your_token_id";
client.cancel_market_orders(
    rs_clob_client::CancelMarketOrdersParams {
        asset_id: token_id.to_string(),
    }
).await?;

// Cancel all orders
client.cancel_all().await?;

// Get your open orders
let open_orders = client.get_orders().await?;
println!("Open orders: {:?}", open_orders);
```

## Advanced Usage

### Market Orders

```rust
use rs_clob_client::{UserMarketOrder, Side};

// Execute a market buy
let market_order = UserMarketOrder {
    token_id: "your_token_id".to_string(),
    amount: 100.0, // Amount in USDC
    price: None,   // Optional price limit
    fee_rate_bps: None,
    nonce: None,
};

let result = client.post_order(market_order, Side::Buy).await?;
println!("Market order executed: {:?}", result);
```

### Builder API Integration

```rust
use rs_builder_signing_sdk::BuilderConfig;

// Create builder configuration
let builder_config = BuilderConfig::new(
    "your_builder_api_key".to_string(),
    "your_builder_secret".to_string(),
);

// Create client with builder support
let client = ClobClient::new(
    "https://clob.polymarket.com".to_string(),
    Chain::Polygon,
    Some(wallet),
    Some(creds),
    Some(0),
    None,
    None,
    false,
    Some(builder_config),
);

// Now you can use builder-specific endpoints
let builder_trades = client.get_builder_trades(None, None).await?;
```

## Security Considerations

- 🔒 **Private Keys**: Never hardcode private keys. Use environment variables or secure key management solutions
- 🔐 **EIP-712**: All orders are signed using EIP-712 standard to prevent replay and phishing attacks
- ✅ **API Credentials**: Store API keys securely. Use L2 authentication for production deployments
- 🛡️ **Domain Separation**: Orders are bound to specific chain IDs and contract addresses
- 🔑 **Key Derivation**: Use `create_or_derive_api_key()` instead of creating new keys repeatedly
- ⚠️ **Rate Limiting**: Respect API rate limits to avoid being blocked

## Configuration

### Chain IDs
- **Polygon Mainnet**: `Chain::Polygon` (137)
- **Amoy Testnet**: `Chain::Amoy` (80002)

### Signature Types
- **0 (EOA)**: Standard wallet signatures (MetaMask, Coinbase Wallet, etc.)
- **1 (Poly Proxy)**: Polymarket proxy contract (for email/Magic login users)
- **2 (EIP-1271)**: Smart contract wallets (Gnosis Safe, etc.)

### Order Types
- **GTC** (Good-Til-Cancelled): Order stays open until filled or cancelled
- **FOK** (Fill-Or-Kill): Order must be filled immediately or cancelled
- **GTD** (Good-Til-Date): Order expires at a specific time

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Build the library
cargo build --release
```

## Examples

See the TypeScript examples in the original repository for reference. Rust equivalents can be created following the patterns shown above.

## Notice

⚠️ **AI-Generated Code**: This library was generated with AI assistance as a port from the official TypeScript implementation. While it aims for feature parity and has been structured following Rust best practices, users should:
- Review the code thoroughly before using in production
- Conduct their own security audits
- Test extensively with their specific use cases
- Start with testnet (Amoy) before moving to mainnet
- Use at their own risk

## Contributing

Contributions are welcome! This is a community-maintained port of the official TypeScript client.

## License

MIT

## Acknowledgments

- Original TypeScript implementation: [Polymarket clob-client](https://github.com/Polymarket/clob-client)
