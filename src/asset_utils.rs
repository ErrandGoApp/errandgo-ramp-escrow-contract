use soroban_sdk::{token, Address, Env};

pub fn take_asset(e: &Env, from: &Address, asset: Address, amount: i128) {
    let token = token::Client::new(e, &asset);
    e.current_contract_address();
    token.transfer(from, &e.current_contract_address(), &amount);
}

pub fn send_asset(e: &Env, to: &Address, asset: Address, amount: i128) {
    let token = token::Client::new(e, &asset);
    token.transfer(&e.current_contract_address(), to, &amount);
}
