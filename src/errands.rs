use soroban_sdk::{xdr::ToXdr, Address, Env, String};

use crate::{
    data::{DataKey, ErrandData, ErrandState},
    error::ContractError,
};

pub fn read_errand_funded(e: &Env, errand_id: String) -> bool {
    let key = DataKey::Errands(errand_id.to_xdr(e));
    e.storage().instance().has(&key)
}
pub fn read_errand(e: &Env, errand_id: String) -> Result<ErrandData, ContractError> {
    let key = DataKey::Errands(errand_id.to_xdr(e));
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::ErrandNotFound)
}

pub fn write_errand(
    e: &Env,
    errand_id: String,
    customer_id: String,
    customer_wallet: Address,
    amount: i128,
    runner_id: String,
    runner_wallet: Address,
) {
    let key = DataKey::Errands(errand_id.to_xdr(e));
    e.storage().instance().set(
        &key,
        &ErrandData {
            customer_id,
            customer_wallet,
            amount,
            runner_id,
            runner_wallet,
            state: ErrandState::Funded,
        },
    );
}

pub fn write_update_errand(e: &Env, errand_id: String, errand: ErrandData) {
    let key = DataKey::Errands(errand_id.to_xdr(e));
    e.storage().instance().set(&key, &errand);
}

pub fn write_remove_errand(e: &Env, errand_id: String) -> Result<(), ContractError> {
    let key = DataKey::Errands(errand_id.to_xdr(e));

    if !e.storage().instance().has(&key) {
        return Err(ContractError::ErrandNotFound);
    }

    e.storage().instance().remove(&key);

    Ok(())
}
