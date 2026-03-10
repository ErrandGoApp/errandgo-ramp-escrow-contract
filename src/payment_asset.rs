use soroban_sdk::{Address, Env};

use crate::{data::DataKey, error::ContractError};

pub fn read_payment_asset(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::PaymentAsset;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::PaymentAssetNotFound)
}

pub fn write_payment_asset(e: &Env, asset: &Address) {
    let key = DataKey::PaymentAsset;
    e.storage().instance().set(&key, asset);
}
