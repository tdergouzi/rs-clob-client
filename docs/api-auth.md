# Auth Module API

> This module handles API key management, account status, balance, and notifications.

---

## Table of Contents

- [API Key Management (L1)](#api-key-management-l1)
- [API Key Management (L2)](#api-key-management-l2)
- [Builder API Key (L2/Builder)](#builder-api-key-l2builder)
- [Balance & Allowance](#balance--allowance)
- [Notifications](#notifications)

---

## API Key Management (L1)

### `create_api_key`

Create a new API key.

| | |
|---|---|
| **Description** | Creates a new API key pair for L2 authentication |
| **Params** | `nonce: Option<u64>` - Optional nonce for signature |
| **Returns** | `ClobResult<ApiKeyCreds>` - API key credentials (key, secret, passphrase) |
| **Auth** | **L1** |

---

### `derive_api_key`

Derive existing API key.

| | |
|---|---|
| **Description** | Retrieves an existing API key associated with the wallet |
| **Params** | `nonce: Option<u64>` - Optional nonce for signature |
| **Returns** | `ClobResult<ApiKeyCreds>` - API key credentials |
| **Auth** | **L1** |

---

### `create_or_derive_api_key`

Create or derive API key.

| | |
|---|---|
| **Description** | Attempts to derive existing key first; creates new one if not found |
| **Params** | `nonce: Option<u64>` - Optional nonce for signature |
| **Returns** | `ClobResult<ApiKeyCreds>` - API key credentials |
| **Auth** | **L1** |

---

## API Key Management (L2)

### `get_api_keys`

List all API keys.

| | |
|---|---|
| **Description** | Retrieves all API keys associated with the account |
| **Params** | None |
| **Returns** | `ClobResult<ApiKeysResponse>` - List of API keys |
| **Auth** | **L2** |

---

### `delete_api_key`

Delete current API key.

| | |
|---|---|
| **Description** | Revokes the currently used API key |
| **Params** | None |
| **Returns** | `ClobResult<serde_json::Value>` - Deletion confirmation |
| **Auth** | **L2** |

---

### `get_closed_only_mode`

Check account closed-only status.

| | |
|---|---|
| **Description** | Returns whether the account is restricted to closing positions only (ban status) |
| **Params** | None |
| **Returns** | `ClobResult<BanStatus>` - Account restriction status |
| **Auth** | **L2** |

---

## Builder API Key (L2/Builder)

### `create_builder_api_key`

Create a builder API key.

| | |
|---|---|
| **Description** | Creates a new API key for builder/market maker operations |
| **Params** | None |
| **Returns** | `ClobResult<BuilderApiKey>` - Builder API key details |
| **Auth** | **L2** |

---

### `get_builder_api_keys`

List builder API keys.

| | |
|---|---|
| **Description** | Retrieves all builder API keys for the account |
| **Params** | None |
| **Returns** | `ClobResult<Vec<BuilderApiKeyResponse>>` - List of builder API keys |
| **Auth** | **L2** |

---

### `revoke_builder_api_key`

Revoke builder API key.

| | |
|---|---|
| **Description** | Revokes the currently configured builder API key |
| **Params** | None |
| **Returns** | `ClobResult<serde_json::Value>` - Revocation confirmation |
| **Auth** | **Builder** |

---

## Balance & Allowance

### `get_balance_allowance`

Get balance and allowance information.

| | |
|---|---|
| **Description** | Returns USDC.e balance and approval status for exchange contracts |
| **Params** | `params: BalanceAllowanceParams` - Asset type (COLLATERAL/CONDITIONAL) and optional token_id |
| **Returns** | `ClobResult<serde_json::Value>` - Balance and allowance data |
| **Auth** | **L2** |

**Note:** 
- USDC.e contract: `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174`
- Exchange contracts:
  - Main: `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E`
  - Neg risk: `0xC5d563A36AE78145C45a50134d48A1215220f80a`
  - Neg risk adapter: `0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296`

---

## Notifications

### `get_notifications`

Get user notifications.

| | |
|---|---|
| **Description** | Retrieves all notifications for the authenticated user |
| **Params** | None |
| **Returns** | `ClobResult<Vec<Notification>>` - List of notifications |
| **Auth** | **L2** |

---

### `drop_notifications`

Delete notifications.

| | |
|---|---|
| **Description** | Removes specified notifications from the user's list |
| **Params** | `params: DropNotificationParams` - List of notification IDs to delete |
| **Returns** | `ClobResult<()>` - Empty on success |
| **Auth** | **L2** |

