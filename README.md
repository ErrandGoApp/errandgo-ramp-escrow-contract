# ErrandGo Soroban Contract

This is ErrandGo's escrow, errand management and payment contract. It powers ErrandGo's escrow-based errands, fiat on-ramp/off-ramp settlement, reserve liquidity management, payments and automated commission distribution for the ErrandGo ecosystem.

The contract enables a marketplace where:

- customers fund errands in escrow
- runners complete errands and receive payouts
- the platform collects commissions and service fees
- disputes can be resolved transparently
- fiat conversions are handled via configurable on-ramp and off-ramp rates

---

# Overview

The contract manages four main components:

## 1. Platform Configuration

- administrator management
- paymaster management
- ErrandGo treasury account management
- payment asset configuration
- platform rates and fee updates

## 2. Reserve Management

- deposit liquidity into reserve
- withdraw reserve funds
- track reserve deposits

## 3. Fiat On-Ramp / Off-Ramp

- convert fiat value into crypto value
- convert crypto value into fiat equivalent
- emit events for off-chain settlement processors

## 4. Escrow-Based Errands

- fund errands in escrow
- track errand lifecycle
- release escrow to runners
- resolve disputes

---

# Contract Roles

## Administrator

The administrator has full control over platform configuration.

Admin permissions include:

- update rates
- update administrator address
- update paymaster
- update ErrandGo treasury account
- withdraw reserve funds
- resolve disputed errands
- upgrade contract

---

## Paymaster

The paymaster acts as an operational backend account responsible for system automation.

Permissions include:

- execute on-ramp settlement
- mark errands completed
- mark errands disputed
- release escrow after completion

---

## Customer

Customers interact with the contract by:

- depositing funds to reserve
- initiating off-ramp conversions
- funding errand escrows

---

## Runner

Runners complete errands and receive payouts when escrow is released.

---

# Contract Initialization

The contract is initialized using the constructor below:

```rust
__constructor(
    e: Env,
    admin: Address,
    errandgo_account: Address,
    paymaster: Address,
    asset: Address,
    on_ramp: i128,
    off_ramp: i128,
    tx_fee: i128,
    platform_fee: i128,
    commission_percent: u32,
    precision: u32,
)
```

## Constructor Parameters

| Parameter          | Type    | Description                                     |
| ------------------ | ------- | ----------------------------------------------- |
| admin              | Address | Platform administrator                          |
| errandgo_account   | Address | ErrandGo treasury account                       |
| paymaster          | Address | Operational backend account                     |
| asset              | Address | Payment asset used for transfers                |
| on_ramp            | i128    | Fiat-to-crypto conversion rate                  |
| off_ramp           | i128    | Crypto-to-fiat conversion rate                  |
| tx_fee             | i128    | Off-ramp transaction fee                        |
| platform_fee       | i128    | Fee charged when funding errands                |
| commission_percent | u32     | Platform commission deducted from errand amount |
| precision          | u32     | Decimal precision used in rate calculations     |

### Notes

- `precision` must not be greater than **18**
- the constructor stores the initial platform addresses and rates
- the configured payment asset is used throughout reserve, ramp, and escrow operations

---

# Rate Management

Rates define how fiat and crypto values are translated and what fees are applied.

## Stored Rate Parameters

- on_ramp
- off_ramp
- tx_fee
- platform_fee
- commission_percent
- precision

---

# Admin Methods

## update_rates

```rust
fn update_rates(
    e: Env,
    off_ramp: i128,
    on_ramp: i128,
    tx_fee: i128,
    platform_fee: i128,
    commission_percent: u32,
    precision: u32,
) -> Result<(), ContractError>
```

Updates the platform rates and fee configuration.

Authorization: **administrator**

---

## update_admin

```rust
fn update_admin(e: Env, id: Address) -> Result<(), ContractError>
```

Updates the administrator address.

Authorization: **administrator**

---

## update_errandgo_account

```rust
fn update_errandgo_account(e: Env, id: Address) -> Result<(), ContractError>
```

Updates the ErrandGo treasury account.

Authorization: **administrator**

---

## update_paymaster

```rust
fn update_paymaster(e: Env, id: Address) -> Result<(), ContractError>
```

Updates the paymaster address.

Authorization: **administrator**

---

# Reserve Management

The reserve provides liquidity for settlement operations such as on-ramp payouts.

## deposit_to_reserve

```rust
fn deposit_to_reserve(e: Env, from: Address, amount: i128) -> Result<(), ContractError>
```

Deposits funds into the reserve.

### Flow

- `from` authorizes the transaction
- `amount` is validated to ensure it is non-negative
- the configured payment asset is taken from the caller
- reserve deposit tracking is increased

Authorization: **from**

---

## withdraw_from_reserve

```rust
fn withdraw_from_reserve(e: Env, to: Address, amount: i128) -> Result<(), ContractError>
```

Withdraws funds from reserve to a destination address.

### Flow

- administrator authorizes the transaction
- amount is validated to ensure it is non-negative
- payment asset is sent to the destination
- reserve deposit tracking is reduced

Authorization: **administrator**

---

## get_reserve

```rust
fn get_reserve(e: Env) -> Result<Reserve, ContractError>
```

Returns reserve information.

### Return Value

```rust
Reserve {
    deposit,
    balance,
}
```

### Notes

- `deposit` reflects tracked reserve deposits
- `balance` is currently returned as `0` in the present implementation
- the method also extends the contract instance TTL

---

# On-Ramp

```rust
fn on_ramp(e: Env, to: Address, fiat_amount: i128) -> Result<(Address, i128), ContractError>
```

Used by the paymaster to convert fiat value into crypto asset value and send the resulting asset amount to a user.

### Calculation

```
crypto_amount = (fiat_amount * on_ramp) / 10^precision
```

### Flow

- paymaster authorizes the transaction
- fiat amount is validated
- crypto amount is calculated using the configured on-ramp rate
- payment asset is sent to `to`
- `OnRampEvent` is emitted

### Returns

```
(to, crypto_amount)
```

---

# Off-Ramp

```rust
fn off_ramp(
    e: Env,
    from: Address,
    customer_id: String,
    amount: i128,
) -> Result<(String, i128), ContractError>
```

Used when a customer wants to convert crypto into fiat value.

### Calculation

```
fiat_amount = (amount * off_ramp) / 10^precision
```

### Flow

- `from` authorizes the transaction
- amount is validated
- total amount collected is calculated

```
total_amount = amount + tx_fee
```

- `amount + tx_fee` is taken from the user
- `tx_fee` is sent to the ErrandGo treasury account
- fiat value is computed using the configured off-ramp rate
- `OffRampEvent` is emitted for off-chain settlement

### Returns

```
(customer_id, fiat_amount)
```

---

# Errand Escrow Lifecycle

The contract manages the full lifecycle of an errand escrow.

---

## fund_errand_escrow

```rust
fn fund_errand_escrow(
    e: Env,
    errand_id: String,
    customer_id: String,
    customer_wallet: Address,
    runner_id: String,
    runner_wallet: Address,
    fiat_amount: i128,
) -> Result<(String, i128), ContractError>
```

Creates and funds a new errand escrow.

### Flow

- checks that the errand is not already funded
- customer authorizes the transaction
- fiat amount is validated
- fiat amount is converted to crypto amount using on_ramp

```
amount_total = crypto_amount + platform_fee
```

- payment asset is taken from the customer wallet
- errand data is written to storage
- errand state is set to **Funded**
- `ErrandStateEvent` is emitted

### Returns

```
(errand_id, crypto_amount)
```

---

## mark_errand_completed

```rust
fn mark_errand_completed(
    e: &Env,
    errand_id: String,
) -> Result<(String, ErrandState), ContractError>
```

Requirements:

- only **paymaster** can call this
- errand must be in **Funded** state

### State Transition

```
Funded -> Completed
```

Emits:

```
ErrandStateEvent
```

---

## mark_errand_disputed

```rust
fn mark_errand_disputed(
    e: &Env,
    errand_id: String,
) -> Result<(String, ErrandState), ContractError>
```

Requirements:

- only **paymaster**
- errand must be **Funded**

### State Transition

```
Funded -> Disputed
```

Emits:

```
ErrandStateEvent
```

---

## release_errand_escrow

```rust
fn release_errand_escrow(
    e: &Env,
    errand_id: String,
) -> Result<(String, ErrandState), ContractError>
```

Requirements:

- only **paymaster**
- errand must be **Completed**

### Calculation

```
commission_amount = errand.amount * commission_percent / 100
platform_total = platform_fee + commission_amount
runner_percent = 100 - commission_percent
runner_amount = errand.amount * runner_percent / 100
```

### Distribution

| Recipient                 | Amount                           |
| ------------------------- | -------------------------------- |
| ErrandGo treasury account | platform_fee + commission_amount |
| Runner wallet             | runner_amount                    |

### Result

- platform funds sent to treasury
- runner payout sent
- errand removed from storage

### Returns

```
(errand_id, ErrandState::Released)
```

Note: the contract removes the errand from storage rather than persisting the final released state.

---

## resolve_disputed_errand

```rust
fn resolve_disputed_errand(
    e: &Env,
    errand_id: String,
    cust_percent: u32,
) -> Result<(String, ErrandState), ContractError>
```

Resolves a disputed errand by splitting funds.

Requirements:

- only **administrator**
- errand must be **Disputed**

### Calculation

```
runner_percent = 100 - (commission_percent + cust_percent)
commission_amount = errand.amount * commission_percent / 100
platform_total = platform_fee + commission_amount
customer_amount = errand.amount * cust_percent / 100
runner_amount = errand.amount * runner_percent / 100
```

### Distribution

| Recipient         | Amount                           |
| ----------------- | -------------------------------- |
| ErrandGo treasury | platform_fee + commission_amount |
| Customer wallet   | customer_amount                  |
| Runner wallet     | runner_amount                    |

### Returns

```
(errand_id, ErrandState::Resolved)
```

---

# Data Structures

## Rates

```rust
Rates {
    on_ramp,
    off_ramp,
    tx_fee,
    platform_fee,
    commission_percent,
    precision,
}
```

## Parameters

```rust
Parameters {
    on_ramp,
    off_ramp,
    tx_fee,
    platform_fee,
    commission_percent,
    precision,
    paymaster,
    errandgo_account,
}
```

## Reserve

```rust
Reserve {
    deposit,
    balance,
}
```

## ErrandData

```rust
ErrandData {
    errand_id,
    customer_id,
    customer_wallet,
    amount,
    runner_id,
    runner_wallet,
    state,
}
```

## ErrandState

- Funded
- Completed
- Disputed
- Released
- Resolved

---

# Events

## OnRampEvent

```rust
OnRampEvent {
    account,
    crypto_amount,
}
```

## OffRampEvent

```rust
OffRampEvent {
    customer_id,
    fiat_amount,
}
```

## ErrandStateEvent

```rust
ErrandStateEvent {
    errand_id,
    state,
}
```

---

# Read Methods

## get_rates

```rust
fn get_rates(e: Env) -> Rates
```

Returns the current stored rate configuration.

---

## get_parameters

```rust
fn get_parameters(e: Env) -> Result<Parameters, ContractError>
```

Returns active rates and platform accounts.

---

## get_reserve

```rust
fn get_reserve(e: Env) -> Result<Reserve, ContractError>
```

Returns reserve information.

---

# Upgradeability

## upgrade

```rust
fn upgrade(e: Env, wasm: BytesN<32>) -> Result<(), ContractError>
```

Allows administrator to upgrade the contract.

Authorization: **administrator**

---

# Error Handling

The contract uses `ContractError`.

Examples:

- negative input amounts
- arithmetic overflow
- invalid errand state transitions
- errand already funded
- invalid payout percentages
- missing required configuration

Helper:

```rust
fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        return Err(ContractError::NegativeAmount);
    }
    Ok(())
}
```

---

# Security Model

## Authorization

| Operation               | Required Authorization |
| ----------------------- | ---------------------- |
| update rates            | administrator          |
| update admin            | administrator          |
| update ErrandGo account | administrator          |
| update paymaster        | administrator          |
| withdraw reserve        | administrator          |
| resolve disputed errand | administrator          |
| upgrade contract        | administrator          |
| on-ramp                 | paymaster              |
| mark errand completed   | paymaster              |
| mark errand disputed    | paymaster              |
| release errand escrow   | paymaster              |
| deposit reserve         | caller                 |
| off-ramp                | caller                 |
| fund errand escrow      | customer wallet        |

---

# Checked Arithmetic

The contract uses:

- checked_add
- checked_mul
- checked_div
- checked_sub
- checked_pow

---

# State Validation

Valid transitions:

```
Funded -> Completed
Funded -> Disputed
Completed -> Released
Disputed -> Resolved
```

---

# Formula Summary

## On-Ramp

```
crypto_amount = (fiat_amount * on_ramp) / 10^precision
```

## Off-Ramp

```
fiat_amount = (amount * off_ramp) / 10^precision
total_amount = amount + tx_fee
```

## Completed Errand Release

```
commission_amount = errand.amount * commission_percent / 100
platform_total = platform_fee + commission_amount
runner_amount = errand.amount * (100 - commission_percent) / 100
```

## Dispute Resolution

```
runner_percent = 100 - (commission_percent + cust_percent)
customer_amount = errand.amount * cust_percent / 100
runner_amount = errand.amount * runner_percent / 100
platform_total = platform_fee + commission_amount
```

---

# Typical Flow Example

## 1. Customer Funds an Errand

- customer chooses errand
- contract converts fiat → crypto
- customer pays escrow + platform fee
- errand stored as **Funded**

## 2. Platform Marks Completion

- paymaster confirms completion
- state becomes **Completed**

## 3. Escrow Release

- paymaster releases escrow
- commission sent to treasury
- runner receives payout
- errand removed

## 4. Dispute Case

- paymaster marks **Disputed**
- admin sets refund percentage
- funds distributed between platform, customer, and runner
- errand removed

---

# Public Interface Summary

## Admin / Configuration

- update_rates
- update_admin
- update_errandgo_account
- update_paymaster
- upgrade

## Reserve

- deposit_to_reserve
- withdraw_from_reserve
- get_reserve

## Ramp

- on_ramp
- off_ramp

## Errands

- fund_errand_escrow
- mark_errand_completed
- mark_errand_disputed
- release_errand_escrow
- resolve_disputed_errand

## Read Methods

- get_rates
- get_parameters
- get_reserve

---

# Implementation Notes

- all fiat and asset values are handled as integers
- conversions depend on configured precision
- payment asset is used for all transfers
- ramp settlement relies on off-chain operators
- events should be indexed by backend services
- released/resolved errands are removed from storage
- `get_reserve()` currently returns tracked deposits only

---

# Future Improvements

Potential enhancements:

- emit dedicated events for escrow release
- expose real reserve token balance
- validate precision in `update_rates`
- add pause / emergency stop
- add errand query methods
- support multiple payment assets

---

# License

Add your project license here.

Example:

```
MIT License
```
