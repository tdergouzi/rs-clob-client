use crate::constants::{get_contract_config, COLLATERAL_TOKEN_DECIMALS};
use crate::errors::{ClobError, ClobResult};
use crate::types::{
    Chain, CreateOrderOptions, OrderSummary, OrderType, RoundConfig, Side, TickSize,
    UserMarketOrder, UserOrder,
};
use crate::utilities::{decimal_places, round_down, round_normal, round_up};
use alloy_primitives::{Address, U256};
use alloy_signer_local::PrivateKeySigner;
use rs_order_utils::{ExchangeOrderBuilder, OrderData, SignatureType, SignedOrder};
use std::str::FromStr;

pub fn get_rounding_config(tick_size: TickSize) -> RoundConfig {
    match tick_size {
        TickSize::ZeroPointOne => RoundConfig {
            price: 1,
            size: 2,
            amount: 3,
        },
        TickSize::ZeroPointZeroOne => RoundConfig {
            price: 2,
            size: 2,
            amount: 4,
        },
        TickSize::ZeroPointZeroZeroOne => RoundConfig {
            price: 3,
            size: 2,
            amount: 5,
        },
        TickSize::ZeroPointZeroZeroZeroOne => RoundConfig {
            price: 4,
            size: 2,
            amount: 6,
        },
    }
}

pub struct RawAmounts {
    pub side: Side,
    pub raw_maker_amt: f64,
    pub raw_taker_amt: f64,
}

pub fn get_order_raw_amounts(
    side: Side,
    size: f64,
    price: f64,
    round_config: &RoundConfig,
) -> RawAmounts {
    let raw_price = round_normal(price, round_config.price);

    match side {
        Side::Buy => {
            let raw_taker_amt = round_down(size, round_config.size);
            let mut raw_maker_amt = raw_taker_amt * raw_price;

            if decimal_places(raw_maker_amt) > round_config.amount {
                raw_maker_amt = round_up(raw_maker_amt, round_config.amount + 4);
                if decimal_places(raw_maker_amt) > round_config.amount {
                    raw_maker_amt = round_down(raw_maker_amt, round_config.amount);
                }
            }

            RawAmounts {
                side: Side::Buy,
                raw_maker_amt,
                raw_taker_amt,
            }
        }
        Side::Sell => {
            let raw_maker_amt = round_down(size, round_config.size);
            let mut raw_taker_amt = raw_maker_amt * raw_price;

            if decimal_places(raw_taker_amt) > round_config.amount {
                raw_taker_amt = round_up(raw_taker_amt, round_config.amount + 4);
                if decimal_places(raw_taker_amt) > round_config.amount {
                    raw_taker_amt = round_down(raw_taker_amt, round_config.amount);
                }
            }

            RawAmounts {
                side: Side::Sell,
                raw_maker_amt,
                raw_taker_amt,
            }
        }
    }
}

pub fn get_market_order_raw_amounts(
    side: Side,
    amount: f64,
    price: f64,
    round_config: &RoundConfig,
) -> RawAmounts {
    let raw_price = round_down(price, round_config.price);

    match side {
        Side::Buy => {
            let raw_maker_amt = round_down(amount, round_config.size);
            let mut raw_taker_amt = raw_maker_amt / raw_price;

            if decimal_places(raw_taker_amt) > round_config.amount {
                raw_taker_amt = round_up(raw_taker_amt, round_config.amount + 4);
                if decimal_places(raw_taker_amt) > round_config.amount {
                    raw_taker_amt = round_down(raw_taker_amt, round_config.amount);
                }
            }

            RawAmounts {
                side: Side::Buy,
                raw_maker_amt,
                raw_taker_amt,
            }
        }
        Side::Sell => {
            let raw_maker_amt = round_down(amount, round_config.size);
            let mut raw_taker_amt = raw_maker_amt * raw_price;

            if decimal_places(raw_taker_amt) > round_config.amount {
                raw_taker_amt = round_up(raw_taker_amt, round_config.amount + 4);
                if decimal_places(raw_taker_amt) > round_config.amount {
                    raw_taker_amt = round_down(raw_taker_amt, round_config.amount);
                }
            }

            RawAmounts {
                side: Side::Sell,
                raw_maker_amt,
                raw_taker_amt,
            }
        }
    }
}

pub fn calculate_buy_market_price(
    positions: &[OrderSummary],
    amount_to_match: f64,
    order_type: OrderType,
) -> ClobResult<f64> {
    if positions.is_empty() {
        return Err(ClobError::NoMatch);
    }

    let mut sum = 0.0;

    for i in (0..positions.len()).rev() {
        let p = &positions[i];
        let price: f64 = p
            .price
            .parse()
            .map_err(|_| ClobError::Other("Invalid price in orderbook".to_string()))?;
        let size: f64 = p
            .size
            .parse()
            .map_err(|_| ClobError::Other("Invalid size in orderbook".to_string()))?;

        sum += size * price;
        if sum >= amount_to_match {
            return Ok(price);
        }
    }

    if order_type == OrderType::Fok {
        return Err(ClobError::NoMatch);
    }

    let first_price: f64 = positions[0]
        .price
        .parse()
        .map_err(|_| ClobError::Other("Invalid price in orderbook".to_string()))?;
    Ok(first_price)
}

pub fn calculate_sell_market_price(
    positions: &[OrderSummary],
    amount_to_match: f64,
    order_type: OrderType,
) -> ClobResult<f64> {
    if positions.is_empty() {
        return Err(ClobError::NoMatch);
    }

    let mut sum = 0.0;

    for i in (0..positions.len()).rev() {
        let p = &positions[i];
        let price: f64 = p
            .price
            .parse()
            .map_err(|_| ClobError::Other("Invalid price in orderbook".to_string()))?;
        let size: f64 = p
            .size
            .parse()
            .map_err(|_| ClobError::Other("Invalid size in orderbook".to_string()))?;

        sum += size;
        if sum >= amount_to_match {
            return Ok(price);
        }
    }

    if order_type == OrderType::Fok {
        return Err(ClobError::NoMatch);
    }

    let first_price: f64 = positions[0]
        .price
        .parse()
        .map_err(|_| ClobError::Other("Invalid price in orderbook".to_string()))?;
    Ok(first_price)
}

pub async fn build_order(
    signer: PrivateKeySigner,
    exchange_address: &str,
    chain_id: u64,
    order_data: OrderData,
) -> ClobResult<SignedOrder> {
    let exchange_addr = Address::from_str(exchange_address)
        .map_err(|e| ClobError::Other(format!("Invalid exchange address: {}", e)))?;

    let builder = ExchangeOrderBuilder::new(exchange_addr, chain_id, signer, None);

    builder
        .build_signed_order(order_data)
        .await
        .map_err(|e| ClobError::SigningError(e.to_string()))
}

fn parse_units(value: f64, decimals: u8) -> U256 {
    let multiplier = 10_f64.powi(decimals as i32);
    let raw_value = (value * multiplier) as u128;
    U256::from(raw_value)
}

pub fn build_order_creation_args(
    signer_address: Address,
    maker: Address,
    signature_type: SignatureType,
    user_order: &UserOrder,
    round_config: &RoundConfig,
) -> ClobResult<OrderData> {
    let raw_amounts = get_order_raw_amounts(
        user_order.side,
        user_order.size,
        user_order.price,
        round_config,
    );

    let maker_amount = parse_units(raw_amounts.raw_maker_amt, COLLATERAL_TOKEN_DECIMALS);
    let taker_amount = parse_units(raw_amounts.raw_taker_amt, COLLATERAL_TOKEN_DECIMALS);

    let taker = user_order.taker.unwrap_or(Address::ZERO);

    let fee_rate_bps = U256::from(user_order.fee_rate_bps.unwrap_or(0));
    let nonce = U256::from(user_order.nonce.unwrap_or(0));
    let expiration = user_order.expiration.map(U256::from);

    let token_id = U256::from_str(&user_order.token_id)
        .map_err(|e| ClobError::Other(format!("Invalid token_id: {}", e)))?;

    let side = match raw_amounts.side {
        Side::Buy => rs_order_utils::Side::Buy,
        Side::Sell => rs_order_utils::Side::Sell,
    };

    Ok(OrderData {
        maker,
        taker,
        token_id,
        maker_amount,
        taker_amount,
        side,
        fee_rate_bps,
        nonce,
        signer: Some(signer_address),
        expiration,
        signature_type: Some(signature_type),
    })
}

pub async fn create_order(
    wallet: PrivateKeySigner,
    chain_id: Chain,
    signature_type: SignatureType,
    funder_address: Option<Address>,
    user_order: &UserOrder,
    options: &CreateOrderOptions,
) -> ClobResult<SignedOrder> {
    use alloy_signer::Signer;

    let signer_address = wallet.address();
    let maker = funder_address.unwrap_or(signer_address);
    let contract_config =
        get_contract_config(chain_id.chain_id()).map_err(|e| ClobError::Other(e))?;

    let round_config = get_rounding_config(options.tick_size);

    let order_data = build_order_creation_args(
        signer_address,
        maker,
        signature_type,
        user_order,
        &round_config,
    )?;

    let exchange_contract = if options.neg_risk.unwrap_or(false) {
        contract_config.neg_risk_exchange
    } else {
        contract_config.exchange
    };

    build_order(wallet, exchange_contract, chain_id.chain_id(), order_data).await
}

pub fn build_market_order_creation_args(
    signer_address: Address,
    maker: Address,
    signature_type: SignatureType,
    user_market_order: &UserMarketOrder,
    round_config: &RoundConfig,
) -> ClobResult<OrderData> {
    let price = user_market_order.price.unwrap_or(1.0);

    let raw_amounts = get_market_order_raw_amounts(
        user_market_order.side,
        user_market_order.amount,
        price,
        round_config,
    );

    let maker_amount = parse_units(raw_amounts.raw_maker_amt, COLLATERAL_TOKEN_DECIMALS);
    let taker_amount = parse_units(raw_amounts.raw_taker_amt, COLLATERAL_TOKEN_DECIMALS);

    let taker = user_market_order.taker.unwrap_or(Address::ZERO);

    let fee_rate_bps = U256::from(user_market_order.fee_rate_bps.unwrap_or(0));
    let nonce = U256::from(user_market_order.nonce.unwrap_or(0));

    let token_id = U256::from_str(&user_market_order.token_id)
        .map_err(|e| ClobError::Other(format!("Invalid token_id: {}", e)))?;

    let side = match raw_amounts.side {
        Side::Buy => rs_order_utils::Side::Buy,
        Side::Sell => rs_order_utils::Side::Sell,
    };

    Ok(OrderData {
        maker,
        taker,
        token_id,
        maker_amount,
        taker_amount,
        side,
        fee_rate_bps,
        nonce,
        signer: Some(signer_address),
        expiration: Some(U256::ZERO),
        signature_type: Some(signature_type),
    })
}

pub async fn create_market_order(
    wallet: PrivateKeySigner,
    chain_id: Chain,
    signature_type: SignatureType,
    funder_address: Option<Address>,
    user_market_order: &UserMarketOrder,
    options: &CreateOrderOptions,
) -> ClobResult<SignedOrder> {
    use alloy_signer::Signer;

    let signer_address = wallet.address();
    let maker = funder_address.unwrap_or(signer_address);
    let contract_config =
        get_contract_config(chain_id.chain_id()).map_err(|e| ClobError::Other(e))?;

    let round_config = get_rounding_config(options.tick_size);

    let order_data = build_market_order_creation_args(
        signer_address,
        maker,
        signature_type,
        user_market_order,
        &round_config,
    )?;

    let exchange_contract = if options.neg_risk.unwrap_or(false) {
        contract_config.neg_risk_exchange
    } else {
        contract_config.exchange
    };

    build_order(wallet, exchange_contract, chain_id.chain_id(), order_data).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rounding_config() {
        let config = get_rounding_config(TickSize::ZeroPointZeroOne);
        assert_eq!(config.price, 2);
        assert_eq!(config.size, 2);
        assert_eq!(config.amount, 4);
    }

    #[test]
    fn test_get_order_raw_amounts_buy() {
        let round_config = RoundConfig {
            price: 2,
            size: 2,
            amount: 4,
        };
        let result = get_order_raw_amounts(Side::Buy, 100.0, 0.55, &round_config);
        assert_eq!(result.side, Side::Buy);
        assert_eq!(result.raw_taker_amt, 100.0);
        assert_eq!(result.raw_maker_amt, 55.0);
    }

    #[test]
    fn test_get_order_raw_amounts_sell() {
        let round_config = RoundConfig {
            price: 2,
            size: 2,
            amount: 4,
        };
        let result = get_order_raw_amounts(Side::Sell, 100.0, 0.55, &round_config);
        assert_eq!(result.side, Side::Sell);
        assert_eq!(result.raw_maker_amt, 100.0);
        assert_eq!(result.raw_taker_amt, 55.0);
    }

    #[test]
    fn test_calculate_buy_market_price() {
        let positions = vec![
            OrderSummary {
                price: "0.6".to_string(),
                size: "100".to_string(),
            },
            OrderSummary {
                price: "0.55".to_string(),
                size: "100".to_string(),
            },
            OrderSummary {
                price: "0.5".to_string(),
                size: "100".to_string(),
            },
        ];

        let price = calculate_buy_market_price(&positions, 150.0, OrderType::Fok).unwrap();
        assert_eq!(price, 0.6);
    }

    #[test]
    fn test_calculate_sell_market_price() {
        let positions = vec![
            OrderSummary {
                price: "0.4".to_string(),
                size: "100".to_string(),
            },
            OrderSummary {
                price: "0.45".to_string(),
                size: "100".to_string(),
            },
            OrderSummary {
                price: "0.5".to_string(),
                size: "100".to_string(),
            },
        ];

        let price = calculate_sell_market_price(&positions, 300.0, OrderType::Fok).unwrap();
        assert_eq!(price, 0.4);
    }

    #[test]
    fn test_fok_fails_on_insufficient_liquidity() {
        let positions = vec![OrderSummary {
            price: "0.5".to_string(),
            size: "10".to_string(),
        }];

        let result = calculate_buy_market_price(&positions, 100.0, OrderType::Fok);
        assert!(result.is_err());
    }

    #[test]
    fn test_fak_accepts_partial_fill() {
        let positions = vec![OrderSummary {
            price: "0.5".to_string(),
            size: "10".to_string(),
        }];

        let result = calculate_buy_market_price(&positions, 100.0, OrderType::Fak);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.5);
    }

    #[test]
    fn test_empty_orderbook() {
        let positions: Vec<OrderSummary> = vec![];
        let result = calculate_buy_market_price(&positions, 10.0, OrderType::Fok);
        assert!(matches!(result, Err(ClobError::NoMatch)));
    }
}
