# Public Module API

> **Auth Level: None** - All methods in this module require no authentication.

---

## Table of Contents

- [Server](#server)
- [Tags](#tags)
- [Events](#events)
- [Markets](#markets)
- [Orderbook](#orderbook)
- [Token Info](#token-info)
- [Prices](#prices)

---

## Server

### `get_ok`

Health check endpoint.

| | |
|---|---|
| **Description** | Returns OK status to verify API connectivity |
| **Params** | None |
| **Returns** | `ClobResult<serde_json::Value>` |
| **Auth** | None |

---

### `get_server_time`

Get current server timestamp.

| | |
|---|---|
| **Description** | Returns the server's current Unix timestamp in milliseconds |
| **Params** | None |
| **Returns** | `ClobResult<u64>` - Unix timestamp |
| **Auth** | None |

---

## Tags

### `get_tags`

List all tags with pagination.

| | |
|---|---|
| **Description** | Retrieves a list of tags used to categorize markets |
| **Params** | `params: TagParams` - Pagination options (limit, offset, order, ascending) |
| **Returns** | `ClobResult<Vec<Tag>>` - List of tags |
| **Auth** | None |

---

### `get_tag_by_slug`

Get a specific tag by its slug.

| | |
|---|---|
| **Description** | Retrieves detailed information for a single tag |
| **Params** | `slug: &str` - Tag's URL slug identifier |
| **Returns** | `ClobResult<Tag>` - Tag details |
| **Auth** | None |

---

### `get_popular_tags`

Get popular/featured tags.

| | |
|---|---|
| **Description** | Returns a curated list of popular tags (from constants) |
| **Params** | None |
| **Returns** | `ClobResult<Vec<Tag>>` - List of popular tags |
| **Auth** | None |

---

## Events

### `get_events`

List events with filtering and pagination.

| | |
|---|---|
| **Description** | Retrieves events (prediction market topics) with optional filters |
| **Params** | `params: EventParams` - Filter options (limit, offset, tag_id, closed, order, ascending) |
| **Returns** | `ClobResult<Vec<Event>>` - List of events |
| **Auth** | None |

---

### `get_events_by_id`

Get an event by its ID.

| | |
|---|---|
| **Description** | Retrieves detailed information for a specific event |
| **Params** | `id: &str` - Event ID |
| **Returns** | `ClobResult<Event>` - Event details |
| **Auth** | None |

---

### `get_event_by_slug`

Get an event by its URL slug.

| | |
|---|---|
| **Description** | Retrieves event details using its human-readable slug |
| **Params** | `slug: &str` - Event's URL slug |
| **Returns** | `ClobResult<Event>` - Event details |
| **Auth** | None |

---

## Markets

### `get_markets`

List markets with filtering and pagination.

| | |
|---|---|
| **Description** | Retrieves markets (tradeable outcomes) with optional filters |
| **Params** | `params: MarketParams` - Filter options (limit, offset, order, ascending, condition_id, closed) |
| **Returns** | `ClobResult<Vec<Market>>` - List of markets |
| **Auth** | None |

---

### `get_market_by_id`

Get a market by its ID.

| | |
|---|---|
| **Description** | Retrieves detailed information for a specific market |
| **Params** | `id: &str` - Market ID |
| **Returns** | `ClobResult<Market>` - Market details |
| **Auth** | None |

---

### `get_market_by_slug`

Get a market by its URL slug.

| | |
|---|---|
| **Description** | Retrieves market details using its human-readable slug |
| **Params** | `slug: &str` - Market's URL slug |
| **Returns** | `ClobResult<Market>` - Market details |
| **Auth** | None |

---

## Orderbook

### `get_order_book`

Get orderbook for a specific token.

| | |
|---|---|
| **Description** | Retrieves the current orderbook (bids and asks) for a token |
| **Params** | `token_id: &str` - Token ID to query |
| **Returns** | `ClobResult<OrderBookSummary>` - Orderbook with bids and asks |
| **Auth** | None |

---

### `get_order_books`

Get orderbooks for multiple tokens.

| | |
|---|---|
| **Description** | Batch retrieval of orderbooks for multiple tokens |
| **Params** | `params: Vec<OrderBookParams>` - List of token IDs to query |
| **Returns** | `ClobResult<Vec<OrderBookSummary>>` - List of orderbooks |
| **Auth** | None |

---

### `get_order_book_hash`

Calculate a hash for an orderbook.

| | |
|---|---|
| **Description** | Generates a deterministic hash of the orderbook state (useful for change detection) |
| **Params** | `orderbook: &mut OrderBookSummary` - Orderbook to hash |
| **Returns** | `String` - Hash string |
| **Auth** | None |

---

## Token Info

### `get_spreads`

Get bid-ask spreads for tokens.

| | |
|---|---|
| **Description** | Retrieves spread information for multiple tokens |
| **Params** | `params: Vec<SpreadsParams>` - List of tokens to query |
| **Returns** | `ClobResult<serde_json::Value>` - Spread data |
| **Auth** | None |

---

### `get_tick_size`

Get minimum tick size for a token.

| | |
|---|---|
| **Description** | Returns the minimum price increment for a token (cached after first call) |
| **Params** | `token_id: &str` - Token ID |
| **Returns** | `ClobResult<TickSize>` - Tick size enum (0.1, 0.01, 0.001, 0.0001) |
| **Auth** | None |

---

### `get_neg_risk`

Check if token is negative risk.

| | |
|---|---|
| **Description** | Returns whether a token uses the negative risk exchange contract (cached) |
| **Params** | `token_id: &str` - Token ID |
| **Returns** | `ClobResult<bool>` - True if negative risk |
| **Auth** | None |

---

### `get_fee_rate_bps`

Get fee rate in basis points.

| | |
|---|---|
| **Description** | Returns the trading fee rate for a token in basis points |
| **Params** | `token_id: &str` - Token ID |
| **Returns** | `ClobResult<u32>` - Fee rate in bps |
| **Auth** | None |

---

## Prices

### `get_price`

Get price for a token and side.

| | |
|---|---|
| **Description** | Returns the current best price for buying or selling a token |
| **Params** | `params: PriceParams` - Token ID and side (BUY/SELL) |
| **Returns** | `ClobResult<Price>` - Price information |
| **Auth** | None |

---

### `get_prices`

Get prices for multiple tokens.

| | |
|---|---|
| **Description** | Batch retrieval of prices for multiple token/side combinations |
| **Params** | `params: Vec<PriceParams>` - List of token/side pairs |
| **Returns** | `ClobResult<serde_json::Value>` - Price data |
| **Auth** | None |

---

### `get_midpoint`

Get midpoint price for a token.

| | |
|---|---|
| **Description** | Returns the midpoint between best bid and ask |
| **Params** | `token_id: &str` - Token ID |
| **Returns** | `ClobResult<Midpoint>` - Midpoint price |
| **Auth** | None |

---

### `get_midpoints`

Get midpoints for multiple tokens.

| | |
|---|---|
| **Description** | Batch retrieval of midpoint prices |
| **Params** | `params: Vec<OrderBookParams>` - List of token IDs |
| **Returns** | `ClobResult<serde_json::Value>` - Midpoint data |
| **Auth** | None |

---

### `get_prices_history`

Get historical price data.

| | |
|---|---|
| **Description** | Retrieves historical price data for charting. Requires either (start_ts AND end_ts) OR interval |
| **Params** | `params: PriceHistoryParams` - Token ID, fidelity, time range or interval |
| **Returns** | `ClobResult<HistoryPrice>` - Historical price data |
| **Auth** | None |

---

### `get_last_trade_price`

Get last trade price for a token.

| | |
|---|---|
| **Description** | Returns the price of the most recent trade |
| **Params** | `token_id: &str` - Token ID |
| **Returns** | `ClobResult<serde_json::Value>` - Last trade price |
| **Auth** | None |

---

### `get_last_trades_prices`

Get last trade prices for multiple tokens.

| | |
|---|---|
| **Description** | Batch retrieval of last trade prices |
| **Params** | `params: Vec<LastTradePriceParams>` - List of token IDs |
| **Returns** | `ClobResult<serde_json::Value>` - Last trade prices |
| **Auth** | None |

