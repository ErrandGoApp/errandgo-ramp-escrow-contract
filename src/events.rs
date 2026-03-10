use soroban_sdk::{contractevent, Address, String};

use crate::data::ErrandState;

#[contractevent(topics=["OffRamp"])]
pub struct OffRampEvent {
    pub customer_id: String,
    pub fiat_amount: i128,
}
#[contractevent(topics=["OnRamp"])]
pub struct OnRampEvent {
    pub account: Address,
    pub crypto_amount: i128,
}
#[contractevent(topics=["ErrandState"])]
pub struct ErrandStateEvent {
    pub errand_id: String,
    pub state: ErrandState,
}
