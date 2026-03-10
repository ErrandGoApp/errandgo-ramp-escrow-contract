use soroban_sdk::{Address, Env};

use crate::{data::DataKey, error::ContractError};

pub fn read_administrator(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Admin;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::AdminNotFound)
}

pub fn write_errandgo_account(e: &Env, id: &Address) {
    let key = DataKey::ErrandGoAccount;
    e.storage().instance().set(&key, id);
}
pub fn read_errandgo_account(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::ErrandGoAccount;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::EGAccountNotFound)
}

pub fn write_paymaster(e: &Env, id: &Address) {
    let key = DataKey::PayMaster;
    e.storage().instance().set(&key, id);
}
pub fn read_paymaster(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::PayMaster;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::PaymasterNotFound)
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}
