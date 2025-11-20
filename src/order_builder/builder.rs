use crate::errors::ClobResult;
use crate::types::{Chain, CreateOrderOptions, UserMarketOrder, UserOrder};
use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use rs_order_utils::{SignatureType, SignedOrder};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::helpers::{create_market_order, create_order};

/// Type alias for dynamic signer resolver function
type GetSignerFn = Arc<
    dyn Fn() -> Pin<Box<dyn Future<Output = ClobResult<PrivateKeySigner>> + Send>> + Send + Sync,
>;

/// OrderBuilder creates and signs orders for the Polymarket CLOB
pub struct OrderBuilder {
    wallet: PrivateKeySigner,
    chain_id: Chain,
    signature_type: SignatureType,
    /// Optional funder address for smart contract wallets
    funder_address: Option<Address>,
    /// Optional function to dynamically resolve the signer
    get_signer: Option<GetSignerFn>,
}

impl OrderBuilder {
    /// Creates a new OrderBuilder
    pub fn new(
        wallet: PrivateKeySigner,
        chain_id: Chain,
        signature_type: Option<SignatureType>,
        funder_address: Option<Address>,
        get_signer: Option<GetSignerFn>,
    ) -> Self {
        Self {
            wallet,
            chain_id,
            signature_type: signature_type.unwrap_or(SignatureType::Eoa),
            funder_address,
            get_signer,
        }
    }

    /// Generates and signs a limit order
    pub async fn build_order(
        &self,
        user_order: &UserOrder,
        options: &CreateOrderOptions,
    ) -> ClobResult<SignedOrder> {
        let signer = self.resolve_signer().await?;
        create_order(
            signer,
            self.chain_id,
            self.signature_type,
            self.funder_address,
            user_order,
            options,
        )
        .await
    }

    /// Generates and signs a market order
    pub async fn build_market_order(
        &self,
        user_market_order: &UserMarketOrder,
        options: &CreateOrderOptions,
    ) -> ClobResult<SignedOrder> {
        let signer = self.resolve_signer().await?;
        create_market_order(
            signer,
            self.chain_id,
            self.signature_type,
            self.funder_address,
            user_market_order,
            options,
        )
        .await
    }

    /// Resolves the signer: uses get_signer if provided, otherwise returns the static wallet
    async fn resolve_signer(&self) -> ClobResult<PrivateKeySigner> {
        if let Some(ref get_signer_fn) = self.get_signer {
            get_signer_fn().await
        } else {
            Ok(self.wallet.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_signer::Signer;

    #[test]
    fn test_order_builder_creation() {
        let wallet = PrivateKeySigner::random();
        let builder = OrderBuilder::new(wallet, Chain::Amoy, None, None, None);

        assert_eq!(builder.chain_id, Chain::Amoy);
        assert_eq!(builder.signature_type, SignatureType::Eoa);
        assert!(builder.funder_address.is_none());
        assert!(builder.get_signer.is_none());
    }

    #[test]
    fn test_order_builder_with_options() {
        let wallet = PrivateKeySigner::random();
        let funder = wallet.address();

        let builder = OrderBuilder::new(
            wallet,
            Chain::Polygon,
            Some(SignatureType::PolyProxy),
            Some(funder),
            None,
        );

        assert_eq!(builder.chain_id, Chain::Polygon);
        assert_eq!(builder.signature_type, SignatureType::PolyProxy);
        assert_eq!(builder.funder_address, Some(funder));
    }

    #[tokio::test]
    async fn test_resolve_signer_without_get_signer() {
        let wallet = PrivateKeySigner::random();
        let original_address = wallet.address();

        let builder = OrderBuilder::new(wallet, Chain::Amoy, None, None, None);

        let resolved = builder.resolve_signer().await.unwrap();
        assert_eq!(resolved.address(), original_address);
    }
}
