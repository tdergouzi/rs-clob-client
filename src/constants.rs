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
