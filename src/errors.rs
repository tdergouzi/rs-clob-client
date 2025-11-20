use thiserror::Error;

/// Errors that can occur when using the CLOB client
#[derive(Error, Debug)]
pub enum ClobError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Authentication error - L1 (wallet signature required)
    #[error("Signer is needed to interact with this endpoint")]
    L1AuthUnavailable,

    /// Authentication error - L2 (API credentials required)
    #[error("API Credentials are needed to interact with this endpoint")]
    L2AuthNotAvailable,

    /// Builder authentication error
    #[error("Builder API Credentials needed to interact with this endpoint")]
    BuilderAuthNotAvailable,

    /// Builder authentication failed
    #[error("Builder key auth failed")]
    BuilderAuthFailed,

    /// Invalid price
    #[error("Invalid price ({price}), min: {min} - max: {max}")]
    InvalidPrice { price: f64, min: f64, max: f64 },

    /// Invalid tick size
    #[error("Invalid tick size ({tick_size}), minimum for the market is {min_tick_size}")]
    InvalidTickSize {
        tick_size: String,
        min_tick_size: String,
    },

    /// Invalid fee rate
    #[error("Invalid user provided fee rate: {user_fee_rate}, fee rate for the market must be {market_fee_rate}")]
    InvalidFeeRate {
        user_fee_rate: u32,
        market_fee_rate: u32,
    },

    /// No orderbook available
    #[error("No orderbook available")]
    NoOrderbook,

    /// No match in orderbook
    #[error("No match found in orderbook")]
    NoMatch,

    /// Ethereum wallet error
    #[error("Ethereum wallet error: {0}")]
    WalletError(String),

    /// EIP-712 signing error
    #[error("EIP-712 signing error: {0}")]
    SigningError(String),

    /// Base64 decoding error
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// API error response
    #[error("API error: {message}")]
    ApiError { message: String, status: u16 },

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type alias for CLOB operations
pub type ClobResult<T> = Result<T, ClobError>;
