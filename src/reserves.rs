use crate::{
    data::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD},
    error::ContractError,
    payment_asset::read_payment_asset,
};
use soroban_sdk::{token, Address, Env};

pub fn read_reserve_deposit(e: &Env, asset: Address) -> i128 {
    let key = DataKey::ReserveDeposit(asset);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

fn write_reserve_deposit(e: &Env, asset: Address, amount: i128) {
    let key = DataKey::ReserveDeposit(asset);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn write_add_reserve(e: &Env, asset: Address, amount: i128) {
    let balance = read_reserve_deposit(e, asset.clone());
    write_reserve_deposit(e, asset, balance.saturating_add(amount));
}

pub fn write_remove_reserve(e: &Env, asset: Address, amount: i128) -> Result<(), ContractError> {
    let deposit = read_reserve_deposit(e, asset.clone());
    let reserve_balance = read_reserve_balance(&e)?;
    if reserve_balance < amount {
        return Err(ContractError::InsufficientBalance);
    }

    let new_deposit = deposit
        .checked_sub(amount)
        .ok_or(ContractError::Underflow)?;

    write_reserve_deposit(e, asset, new_deposit);
    Ok(())
}

pub fn read_reserve_balance(e: &Env) -> Result<i128, ContractError> {
    let asset = read_payment_asset(e)?;
    let asset_client = token::Client::new(e, &asset);
    let contract_address = e.current_contract_address();
    Ok(asset_client.balance(&contract_address))
}

// pub fn read_pending_payments(e: &Env, asset: Address) -> i128 {
//     let key = DataKey::ReservePendingPayments(asset);
//     if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
//         e.storage()
//             .persistent()
//             .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
//         balance
//     } else {
//         0
//     }
// }

// fn write_pending_payments(e: &Env, asset: Address, amount: i128) {
//     let key = DataKey::ReserveDeposit(asset);
//     e.storage().persistent().set(&key, &amount);
//     e.storage()
//         .persistent()
//         .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
// }

// pub fn write_add_pending_payments(e: &Env, asset: Address, amount: i128) {
//     let payments = read_pending_payments(e, asset.clone());
//     write_pending_payments(e, asset, payments.saturating_add(amount));
// }

// pub fn write_remove_pending_payments(
//     e: &Env,
//     asset: Address,
//     amount: i128,
// ) -> Result<(), ContractError> {
//     let payments = read_pending_payments(e, asset.clone());
//     if payments < amount {
//         return Err(ContractError::InsufficientPayment);
//     }

//     let new_payment = payments
//         .checked_sub(amount)
//         .ok_or(ContractError::Underflow)?;

//     write_pending_payments(e, asset, new_payment);

//     Ok(())
// }
