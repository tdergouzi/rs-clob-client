// Order Builder - creates and signs orders using rs_order_utils

use crate::constants::{
    AMOY_EXCHANGE, AMOY_NEG_RISK_EXCHANGE, POLYGON_EXCHANGE, POLYGON_NEG_RISK_EXCHANGE,
};
use crate::errors::{ClobError, ClobResult};
use crate::types::{Chain, CreateOrderOptions, Side, UserMarketOrder, UserOrder};
use crate::utilities::price_valid;
use alloy_primitives::{Address, U256};
use alloy_signer_local::PrivateKeySigner;
use ethers::signers::LocalWallet;
use rs_order_utils::{ExchangeOrderBuilder, OrderData, SignedOrder};
use std::str::FromStr;

pub struct OrderBuilder {
    chain_id: Chain,
    wallet: Option<LocalWallet>,
    signature_type: u8, // 0 = EOA, 1 = Poly Proxy, 2 = EIP-1271
    funder_address: Option<String>,
}

impl OrderBuilder {
    pub fn new(chain_id: Chain) -> Self {
        Self {
            chain_id,
            wallet: None,
            signature_type: 0,
            funder_address: None,
        }
    }

    pub fn with_wallet(mut self, wallet: LocalWallet) -> Self {
        self.wallet = Some(wallet);
        self
    }

    pub fn with_signature_type(mut self, sig_type: u8) -> Self {
        self.signature_type = sig_type;
        self
    }

    pub fn with_funder_address(mut self, address: String) -> Self {
        self.funder_address = Some(address);
        self
    }

    /// Get the exchange contract address for the chain and neg_risk setting
    fn get_exchange_address(&self, neg_risk: bool) -> ClobResult<Address> {
        let addr_str = match self.chain_id {
            Chain::Polygon => {
                if neg_risk {
                    POLYGON_NEG_RISK_EXCHANGE
                } else {
                    POLYGON_EXCHANGE
                }
            }
            Chain::Amoy => {
                if neg_risk {
                    AMOY_NEG_RISK_EXCHANGE
                } else {
                    AMOY_EXCHANGE
                }
            }
        };

        Address::from_str(addr_str).map_err(|e| ClobError::Other(format!("Invalid exchange address: {}", e)))
    }

    /// Convert ethers LocalWallet to alloy PrivateKeySigner
    fn convert_wallet(&self) -> ClobResult<PrivateKeySigner> {
        let wallet = self.wallet.as_ref().ok_or(ClobError::L1AuthUnavailable)?;
        
        // Get the private key bytes from ethers wallet
        let private_key_bytes = wallet.signer().to_bytes();
        
        // Convert GenericArray to B256 (FixedBytes<32>)
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&private_key_bytes[..]);
        let b256 = alloy_primitives::B256::from(bytes);
        
        // Create alloy PrivateKeySigner from B256
        PrivateKeySigner::from_bytes(&b256)
            .map_err(|e| ClobError::SigningError(format!("Failed to convert wallet: {}", e)))
    }

    /// Convert price and size to raw amounts (maker_amount, taker_amount)
    fn get_order_raw_amounts(&self, side: Side, size: f64, price: f64) -> (U256, U256) {
        let raw_size = (size * 10f64.powi(6)) as u128; // 6 decimals for CTF tokens
        let raw_price = (price * 10f64.powi(6)) as u128; // 6 decimals for USDC

        match side {
            Side::Buy => {
                // Buy: maker_amount (USDC) = size * price, taker_amount (tokens) = size
                let maker_amount = (raw_size * raw_price) / 10u128.pow(6);
                (U256::from(maker_amount), U256::from(raw_size))
            }
            Side::Sell => {
                // Sell: maker_amount (tokens) = size, taker_amount (USDC) = size * price
                let taker_amount = (raw_size * raw_price) / 10u128.pow(6);
                (U256::from(raw_size), U256::from(taker_amount))
            }
        }
    }

    /// Builds and signs a limit order
    pub async fn build_order(
        &self,
        user_order: &UserOrder,
        options: &CreateOrderOptions,
    ) -> ClobResult<SignedOrder> {
        // Validate price against tick size
        if !price_valid(user_order.price, options.tick_size) {
            let tick = options.tick_size.as_f64();
            return Err(ClobError::InvalidPrice {
                price: user_order.price,
                min: tick,
                max: 1.0 - tick,
            });
        }

        // Convert wallet
        let signer = self.convert_wallet()?;
        let signer_address = signer.address();

        // Get maker address (funder_address if provided, otherwise signer)
        let maker = if let Some(ref funder) = self.funder_address {
            Address::from_str(funder)
                .map_err(|e| ClobError::Other(format!("Invalid funder address: {}", e)))?
        } else {
            signer_address
        };

        // Get exchange address
        let exchange_address = self.get_exchange_address(options.neg_risk.unwrap_or(false))?;

        // Convert token_id to U256
        let token_id = U256::from_str(&user_order.token_id)
            .map_err(|e| ClobError::Other(format!("Invalid token_id: {}", e)))?;

        // Get raw amounts
        let (maker_amount, taker_amount) =
            self.get_order_raw_amounts(user_order.side, user_order.size, user_order.price);

        // Convert side
        let side = match user_order.side {
            Side::Buy => rs_order_utils::Side::Buy,
            Side::Sell => rs_order_utils::Side::Sell,
        };

        // Convert signature type
        let signature_type = match self.signature_type {
            0 => rs_order_utils::SignatureType::Eoa,
            1 => rs_order_utils::SignatureType::PolyProxy,
            2 => rs_order_utils::SignatureType::PolyGnosisSafe,
            _ => rs_order_utils::SignatureType::Eoa,
        };

        // Build OrderData
        let order_data = OrderData {
            maker,
            taker: Address::ZERO, // Public order
            token_id,
            maker_amount,
            taker_amount,
            side,
            fee_rate_bps: U256::from(user_order.fee_rate_bps.unwrap_or(0)),
            nonce: U256::from(user_order.nonce.unwrap_or(0)),
            signer: Some(signer_address),
            expiration: user_order.expiration.map(U256::from),
            signature_type: Some(signature_type),
        };

        // Create ExchangeOrderBuilder
        let builder = ExchangeOrderBuilder::new(
            exchange_address,
            self.chain_id.chain_id(),
            signer,
            None, // Use default salt generator
        );

        // Build and sign order
        builder
            .build_signed_order(order_data)
            .await
            .map_err(|e| ClobError::SigningError(e.to_string()))
    }

    /// Builds and signs a market order
    pub async fn build_market_order(
        &self,
        user_market_order: &UserMarketOrder,
        options: &CreateOrderOptions,
    ) -> ClobResult<SignedOrder> {
        // Validate price if provided
        if let Some(price) = user_market_order.price {
            if !price_valid(price, options.tick_size) {
                let tick = options.tick_size.as_f64();
                return Err(ClobError::InvalidPrice {
                    price,
                    min: tick,
                    max: 1.0 - tick,
                });
            }
        }

        // Market orders use the price calculated from orderbook
        // or provided by user
        let price = user_market_order.price.unwrap_or(0.5); // Default mid-price

        // Calculate size based on amount and side
        let size = match user_market_order.side {
            Side::Buy => user_market_order.amount / price, // amount in USDC -> size in tokens
            Side::Sell => user_market_order.amount,         // amount is already in tokens
        };

        // Create a UserOrder from the market order
        let user_order = UserOrder {
            token_id: user_market_order.token_id.clone(),
            price,
            size,
            side: user_market_order.side,
            fee_rate_bps: user_market_order.fee_rate_bps,
            nonce: user_market_order.nonce,
            expiration: None, // Market orders typically don't have expiration
            taker: user_market_order.taker.clone(),
        };

        // Use the build_order method
        self.build_order(&user_order, options).await
    }
}
