use soroban_sdk::Env;

use crate::data::{DataKey, Rates};

pub fn read_rates(e: &Env) -> Rates {
    let key = DataKey::FxRate;
    e.storage().instance().get(&key).unwrap_or(Rates {
        on_ramp: 0,
        off_ramp: 0,
        tx_fee: 0,
        platform_fee: 0,
        commission_percent: 0,
        precision: 7,
    })
}

pub fn write_rates(
    e: &Env,
    off_ramp: i128,
    on_ramp: i128,
    tx_fee: i128,
    platform_fee: i128,
    commission_percent: u32,
    precision: u32,
) {
    let key = DataKey::FxRate;
    e.storage().instance().set(
        &key,
        &Rates {
            off_ramp,
            on_ramp,
            tx_fee,
            platform_fee,
            commission_percent,
            precision,
        },
    );
}
