#[derive(Debug, Clone)]
pub struct ContractConfig {
    pub exchange: &'static str,
    pub neg_risk_adapter: &'static str,
    pub neg_risk_exchange: &'static str,
    pub collateral: &'static str,
    pub conditional_tokens: &'static str,
}

pub const AMOY_CONTRACTS: ContractConfig = ContractConfig {
    exchange: "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
    neg_risk_adapter: "0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296",
    neg_risk_exchange: "0xC5d563A36AE78145C45a50134d48A1215220f80a",
    collateral: "0x9c4e1703476e875070ee25b56a58b008cfb8fa78",
    conditional_tokens: "0x69308FB512518e39F9b16112fA8d994F4e2Bf8bB",
};

pub const MATIC_CONTRACTS: ContractConfig = ContractConfig {
    exchange: "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
    neg_risk_adapter: "0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296",
    neg_risk_exchange: "0xC5d563A36AE78145C45a50134d48A1215220f80a",
    collateral: "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
    conditional_tokens: "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
};

pub const COLLATERAL_TOKEN_DECIMALS: u8 = 6;
pub const CONDITIONAL_TOKEN_DECIMALS: u8 = 6;

// Pagination cursors
pub const INITIAL_CURSOR: &str = "MA==";
pub const END_CURSOR: &str = "LTE=";

// EIP-712 constants for CLOB authentication
pub const CLOB_DOMAIN_NAME: &str = "ClobAuthDomain";
pub const CLOB_VERSION: &str = "1";
pub const MSG_TO_SIGN: &str = "This message attests that I control the given wallet";

pub fn get_contract_config(chain_id: u64) -> Result<&'static ContractConfig, String> {
    match chain_id {
        137 => Ok(&MATIC_CONTRACTS),
        80002 => Ok(&AMOY_CONTRACTS),
        _ => Err(format!("Invalid network: chain ID {}", chain_id)),
    }
}

// Popular tags data
use crate::types::Tag;

pub fn get_popular_tags() -> Vec<Tag> {
    vec![
        Tag {
            id: "1".to_string(),
            label: "Sports".to_string(),
            slug: "sports".to_string(),
            force_show: false,
            published_at: Some("2023-10-24 22:37:50.296+00".to_string()),
            updated_by: Some(15),
            created_at: Some("2023-10-24T22:37:50.31Z".to_string()),
            updated_at: Some("2024-07-05T21:07:21.800664Z".to_string()),
            force_hide: Some(true),
            created_by: None,
            is_carousel: None,
        },
        Tag {
            id: "2".to_string(),
            label: "Politics".to_string(),
            slug: "politics".to_string(),
            force_show: false,
            published_at: Some("2023-10-25 18:55:50.674+00".to_string()),
            updated_by: Some(13),
            created_at: Some("2023-10-25T18:55:50.681Z".to_string()),
            updated_at: Some("2024-07-05T21:07:05.081707Z".to_string()),
            force_hide: Some(true),
            created_by: None,
            is_carousel: None,
        },
        Tag {
            id: "120".to_string(),
            label: "Finance".to_string(),
            slug: "finance".to_string(),
            force_show: false,
            published_at: Some("2023-11-02 21:22:21.615+00".to_string()),
            created_by: None,
            updated_by: None,
            created_at: Some("2023-11-02T21:22:21.62Z".to_string()),
            updated_at: Some("2025-10-15T03:29:11.255997Z".to_string()),
            force_hide: None,
            is_carousel: Some(false),
        },
        Tag {
            id: "21".to_string(),
            label: "Crypto".to_string(),
            slug: "crypto".to_string(),
            force_show: false,
            published_at: Some("2023-11-02 21:03:54.55+00".to_string()),
            created_by: None,
            updated_by: Some(15),
            created_at: Some("2023-11-02T21:03:54.564Z".to_string()),
            updated_at: Some("2024-07-05T21:07:09.171822Z".to_string()),
            force_hide: None,
            is_carousel: None,
        },
        Tag {
            id: "100265".to_string(),
            label: "Geopolitics".to_string(),
            slug: "geopolitics".to_string(),
            force_show: true,
            created_at: Some("2024-06-12T20:13:03.615956Z".to_string()),
            updated_at: Some("2024-06-12T20:14:49.256191Z".to_string()),
            published_at: None,
            created_by: None,
            updated_by: None,
            force_hide: None,
            is_carousel: None,
        },
        Tag {
            id: "1013".to_string(),
            label: "Earnings".to_string(),
            slug: "earnings".to_string(),
            force_show: false,
            created_at: Some("2024-02-06T19:27:48.029Z".to_string()),
            updated_at: Some("2025-09-15T01:55:15.0242Z".to_string()),
            published_at: Some("2024-02-06 19:27:48.024+00".to_string()),
            created_by: None,
            updated_by: None,
            force_hide: None,
            is_carousel: Some(false),
        },
        Tag {
            id: "1401".to_string(),
            label: "Tech".to_string(),
            slug: "tech".to_string(),
            force_show: false,
            created_at: Some("2024-02-21T23:06:12.324Z".to_string()),
            updated_at: Some("2024-06-18T16:48:23.829512Z".to_string()),
            published_at: Some("2024-02-21 23:06:12.305+00".to_string()),
            created_by: None,
            updated_by: None,
            force_hide: None,
            is_carousel: None,
        },
        Tag {
            id: "101970".to_string(),
            label: "World".to_string(),
            slug: "world".to_string(),
            force_show: false,
            created_at: Some("2025-03-19T23:36:08.498099Z".to_string()),
            updated_at: None,
            published_at: None,
            created_by: None,
            updated_by: None,
            force_hide: None,
            is_carousel: None,
        },
        Tag {
            id: "100328".to_string(),
            label: "Economy".to_string(),
            slug: "economy".to_string(),
            force_show: false,
            created_at: Some("2024-08-05T05:34:54.235643Z".to_string()),
            updated_at: Some("2024-08-19T16:50:54.755406Z".to_string()),
            published_at: None,
            created_by: None,
            updated_by: None,
            force_hide: None,
            is_carousel: None,
        },
        Tag {
            id: "144".to_string(),
            label: "Elections".to_string(),
            slug: "elections".to_string(),
            force_show: false,
            created_at: Some("2023-11-02T21:24:31.198Z".to_string()),
            updated_at: Some("2024-05-03T15:51:20.233636Z".to_string()),
            published_at: Some("2023-11-02 21:24:31.008+00".to_string()),
            created_by: None,
            updated_by: Some(13),
            force_hide: Some(true),
            is_carousel: None,
        },
    ]
}
