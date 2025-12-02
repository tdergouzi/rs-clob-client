# Trading Module API

> This module handles order creation, submission, querying, and cancellation.

---

## Table of Contents

- [Order Creation (L1)](#order-creation-l1)
- [Order Submission (L2)](#order-submission-l2)
- [Order Queries (L2)](#order-queries-l2)
- [Order Cancellation (L2)](#order-cancellation-l2)
- [Builder Trades](#builder-trades)
- [Utilities](#utilities)

---

## Order Creation (L1)

### `create_limit_order`

Create a signed limit order.

| | |
|---|---|
| **Description** | Creates and signs a limit order ready for submission. Does not submit to exchange. |
| **Params** | `user_limit_order: &UserLimitOrder` - Order params (token_id, price, size, side)<br>`options: Option<CreateOrderOptions>` - Optional tick_size and neg_risk overrides |
| **Returns** | `ClobResult<serde_json::Value>` - Signed order JSON |
| **Auth** | **L1** |

---

### `create_market_order`

Create a signed market order.

| | |
|---|---|
| **Description** | Creates and signs a market order. Automatically calculates execution price if not provided. |
| **Params** | `user_market_order: &UserMarketOrder` - Order params (token_id, amount, side)<br>`options: Option<CreateOrderOptions>` - Optional tick_size and neg_risk overrides |
| **Returns** | `ClobResult<serde_json::Value>` - Signed order JSON |
| **Auth** | **L1** |

---

## Order Submission (L2)

### `create_and_post_limit_order`

Create and submit a limit order.

| | |
|---|---|
| **Description** | Creates, signs, and submits a limit order in one call. Size is in shares for both buy and sell. |
| **Params** | `user_limit_order: &UserLimitOrder` - Order params (token_id, price, size, side)<br>`options: Option<CreateOrderOptions>` - Optional overrides<br>`order_type: OrderType` - GTC, FOK, FAK, or GTD |
| **Returns** | `ClobResult<serde_json::Value>` - API response with order status |
| **Auth** | **L2** |

---

### `create_and_post_market_order`

Create and submit a market order.

| | |
|---|---|
| **Description** | Creates, signs, and submits a market order in one call |
| **Params** | `user_market_order: &UserMarketOrder` - Order params (token_id, amount, side)<br>`options: Option<CreateOrderOptions>` - Optional overrides<br>`order_type: OrderType` - Typically FOK or FAK |
| **Returns** | `ClobResult<serde_json::Value>` - API response with order status |
| **Auth** | **L2** |

---

### `post_order`

Submit a signed order.

| | |
|---|---|
| **Description** | Submits a pre-signed order to the exchange |
| **Params** | `order: serde_json::Value` - Signed order from create_* methods<br>`order_type: OrderType` - GTC, FOK, FAK, or GTD |
| **Returns** | `ClobResult<serde_json::Value>` - API response with order status |
| **Auth** | **L2** |

---

### `post_orders`

Submit multiple orders.

| | |
|---|---|
| **Description** | Batch submission of multiple signed orders |
| **Params** | `orders: Vec<PostOrdersArgs>` - List of orders with their types |
| **Returns** | `ClobResult<serde_json::Value>` - API response with order statuses |
| **Auth** | **L2** |

---

## Order Queries (L2)

### `get_trades`

Get all trade history.

| | |
|---|---|
| **Description** | Retrieves complete trade history with automatic pagination. Only includes executed trades, not open limit orders. |
| **Params** | `params: Option<TradeParams>` - Optional filters (id, market, asset_id, maker_address, before, after) |
| **Returns** | `ClobResult<Vec<Trade>>` - List of all trades |
| **Auth** | **L2** |

---

### `get_trades_paginated`

Get trades with pagination control.

| | |
|---|---|
| **Description** | Retrieves trades with manual pagination control |
| **Params** | `params: Option<TradeParams>` - Optional filters<br>`cursor: Option<String>` - Pagination cursor |
| **Returns** | `ClobResult<TradesPaginatedResponse>` - Trades with next_cursor |
| **Auth** | **L2** |

---

### `get_open_order`

Get an open order by ID.

| | |
|---|---|
| **Description** | Retrieves details for a specific open order |
| **Params** | `order_id: &str` - Order ID |
| **Returns** | `ClobResult<OpenOrder>` - Order details |
| **Auth** | **L2** |

---

### `get_open_orders`

Get all open orders.

| | |
|---|---|
| **Description** | Retrieves all open (unfilled) orders for the user |
| **Params** | `params: Option<OpenOrderParams>` - Optional filters (id, market, asset_id) |
| **Returns** | `ClobResult<OpenOrdersResponse>` - List of open orders |
| **Auth** | **L2** |

---

## Order Cancellation (L2)

### `cancel_order`

Cancel a single order.

| | |
|---|---|
| **Description** | Cancels a specific order by ID |
| **Params** | `order_id: &str` - Order ID to cancel |
| **Returns** | `ClobResult<serde_json::Value>` - Cancellation confirmation |
| **Auth** | **L2** |

---

### `cancel_orders`

Cancel multiple orders.

| | |
|---|---|
| **Description** | Cancels multiple orders by their IDs |
| **Params** | `order_ids: Vec<String>` - List of order IDs to cancel |
| **Returns** | `ClobResult<serde_json::Value>` - Cancellation confirmation |
| **Auth** | **L2** |

---

### `cancel_all`

Cancel all open orders.

| | |
|---|---|
| **Description** | Cancels all open orders for the authenticated user |
| **Params** | None |
| **Returns** | `ClobResult<serde_json::Value>` - Cancellation confirmation |
| **Auth** | **L2** |

---

### `cancel_market_orders`

Cancel orders for a specific market.

| | |
|---|---|
| **Description** | Cancels all orders for a specific market or asset |
| **Params** | `params: OrderMarketCancelParams` - Market or asset identifier |
| **Returns** | `ClobResult<serde_json::Value>` - Cancellation confirmation |
| **Auth** | **L2** |

---

## Builder Trades

### `get_builder_trades`

Get builder/market maker trades.

| | |
|---|---|
| **Description** | Retrieves trades made through builder API with pagination |
| **Params** | `params: Option<TradeParams>` - Optional filters (id, market, asset_id)<br>`cursor: Option<String>` - Pagination cursor |
| **Returns** | `ClobResult<BuilderTradesResponse>` - Builder trades with next_cursor |
| **Auth** | **Builder** |

---

## Utilities

### `calculate_market_price`

Calculate market execution price.

| | |
|---|---|
| **Description** | Calculates the expected execution price for a market order based on current orderbook |
| **Params** | `token_id: &str` - Token to trade<br>`side: Side` - Buy or Sell<br>`amount: f64` - Amount in USDC (Buy) or tokens (Sell)<br>`order_type: OrderType` - FOK or FAK |
| **Returns** | `ClobResult<f64>` - Calculated execution price with buffer |
| **Auth** | **None** |

---

## Order Types Reference

| Type | Description |
|------|-------------|
| **GTC** | Good Till Cancelled - Remains open until filled or cancelled |
| **GTD** | Good Till Date - Remains open until specified expiration |
| **FOK** | Fill Or Kill - Must fill completely or cancel entirely |
| **FAK** | Fill And Kill - Fill what's possible, cancel the rest |

