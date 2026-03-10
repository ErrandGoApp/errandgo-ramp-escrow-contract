use soroban_sdk::{contracttype, Address, Bytes, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum ErrandState {
    Created = 0,
    Funded = 1,
    Completed = 2,
    Disputed = 3,
    Resolved = 4,
    Released = 5,
}

#[derive(Clone)]
#[contracttype]
pub struct Reserve {
    // pub pending_payments: i128,
    pub deposit: i128,
    pub balance: i128,
}
#[derive(Clone)]
#[contracttype]
pub struct Rates {
    pub off_ramp: i128,
    pub on_ramp: i128,
    pub tx_fee: i128,
    pub platform_fee: i128,
    pub commission_percent: u32,
    pub precision: u32,
}
#[derive(Clone)]
#[contracttype]
pub struct Parameters {
    pub off_ramp: i128,
    pub on_ramp: i128,
    pub tx_fee: i128,
    pub platform_fee: i128,
    pub commission_percent: u32,
    pub precision: u32,
    pub paymaster: Address,
    pub errandgo_account: Address,
}
#[derive(Clone)]
#[contracttype]
pub struct ErrandData {
    pub customer_id: String,
    pub customer_wallet: Address,
    pub amount: i128,
    pub runner_id: String,
    pub runner_wallet: Address,
    pub state: ErrandState,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ReserveStatus,
    ErrandGoAccount,
    PayMaster,
    ReserveDeposit(Address),
    ReservePendingPayments(Address),
    PaymentAsset,
    State(Address),
    Admin,
    FxRate,
    CommissionPercent,
    Errands(Bytes),
}
