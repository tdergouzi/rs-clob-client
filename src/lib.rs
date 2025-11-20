// Public modules
pub mod client;
pub mod constants;
pub mod endpoints;
pub mod errors;
pub mod types;
pub mod utilities;
pub mod headers;
pub mod signing;

// Internal modules
mod http;
mod order_builder;

// Re-exports for convenience
pub use client::ClobClient;
pub use errors::{ClobError, ClobResult};
pub use types::*;

// Prelude module for common imports
pub mod prelude {
    pub use crate::client::ClobClient;
    pub use crate::errors::{ClobError, ClobResult};
    pub use crate::types::{ApiKeyCreds, Chain, OrderType, Side, UserMarketOrder, UserOrder};
}
