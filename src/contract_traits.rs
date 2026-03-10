use soroban_sdk::{Address, BytesN, Env, String};

use crate::{
    data::{ErrandState, Parameters, Rates, Reserve},
    error::ContractError,
};

pub trait ErrandGoTrait {
    fn __constructor(
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
    );

    fn update_rates(
        e: Env,
        off_ramp: i128,
        on_ramp: i128,
        tx_fee: i128,
        platform_fee: i128,
        commission_percent: u32,
        precision: u32,
    ) -> Result<(), ContractError>;

    fn update_admin(e: Env, id: Address) -> Result<(), ContractError>;
    fn update_errandgo_account(e: Env, id: Address) -> Result<(), ContractError>;
    fn update_paymaster(e: Env, id: Address) -> Result<(), ContractError>;
    fn deposit_to_reserve(e: Env, from: Address, amount: i128) -> Result<(), ContractError>;
    fn withdraw_from_reserve(e: Env, to: Address, amount: i128) -> Result<(), ContractError>;
    fn on_ramp(e: Env, to: Address, fiat_amount: i128) -> Result<(Address, i128), ContractError>;
    fn off_ramp(
        e: Env,
        from: Address,
        customer_id: String,
        amount: i128,
    ) -> Result<(String, i128), ContractError>;
    fn fund_errand_escrow(
        e: Env,
        errand_id: String,
        customer_id: String,
        customer_wallet: Address,
        runner_id: String,
        runner_wallet: Address,
        fiat_amount: i128,
    ) -> Result<(String, i128), ContractError>;
    fn mark_errand_completed(
        e: &Env,
        errand_id: String,
    ) -> Result<(String, ErrandState), ContractError>;
    fn mark_errand_disputed(
        e: &Env,
        errand_id: String,
    ) -> Result<(String, ErrandState), ContractError>;
    fn release_errand_escrow(
        e: &Env,
        errand_id: String,
    ) -> Result<(String, ErrandState), ContractError>;
    fn resolve_disputed_errand(
        e: &Env,
        errand_id: String,
        cust_percent: u32,
    ) -> Result<(String, ErrandState), ContractError>;
    fn get_rates(e: Env) -> Rates;
    fn get_parameters(e: Env) -> Result<Parameters, ContractError>;
    fn get_reserve(e: Env) -> Result<Reserve, ContractError>;

    fn upgrade(e: Env, wasm: BytesN<32>) -> Result<(), ContractError>;
}
