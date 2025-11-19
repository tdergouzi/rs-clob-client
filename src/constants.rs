/// Application constants
pub const CREDS_CREATION_WARNING: &str = r#"🚨🚨🚨
Your credentials CANNOT be recovered after they've been created. 
Be sure to store them safely!
🚨🚨🚨"#;

/// Initial cursor for pagination
pub const INITIAL_CURSOR: &str = "MA==";

/// End cursor indicating no more pages
pub const END_CURSOR: &str = "LTE=";

/// Message to sign for CLOB authentication
pub const MSG_TO_SIGN: &str = "This message attests that I control the given wallet";

/// CLOB domain name for EIP-712
pub const CLOB_DOMAIN_NAME: &str = "ClobAuthDomain";

/// CLOB version for EIP-712
pub const CLOB_VERSION: &str = "1";

// Contract addresses for Polygon (Matic) - Chain ID 137
pub const POLYGON_EXCHANGE: &str = "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E";
pub const POLYGON_NEG_RISK_EXCHANGE: &str = "0xC5d563A36AE78145C45a50134d48A1215220f80a";
pub const POLYGON_NEG_RISK_ADAPTER: &str = "0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296";
pub const POLYGON_COLLATERAL: &str = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
pub const POLYGON_CONDITIONAL_TOKENS: &str = "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045";

// Contract addresses for Amoy (Testnet) - Chain ID 80002
pub const AMOY_EXCHANGE: &str = "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40";
pub const AMOY_NEG_RISK_EXCHANGE: &str = "0xC5d563A36AE78145C45a50134d48A1215220f80a";
pub const AMOY_NEG_RISK_ADAPTER: &str = "0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296";
pub const AMOY_COLLATERAL: &str = "0x9c4e1703476e875070ee25b56a58b008cfb8fa78";
pub const AMOY_CONDITIONAL_TOKENS: &str = "0x69308FB512518e39F9b16112fA8d994F4e2Bf8bB";

/// Token decimals
pub const COLLATERAL_TOKEN_DECIMALS: u8 = 6;
pub const CONDITIONAL_TOKEN_DECIMALS: u8 = 6;

