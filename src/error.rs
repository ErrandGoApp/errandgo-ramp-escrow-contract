use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    InvalidPrecision = 301,
    NegativeAmount = 303,
    Underflow = 305,
    Overflow = 306,
    InsufficientPayment = 307,
    InsufficientBalance = 309,
    PaymentAssetNotFound = 311,
    AdminNotFound = 313,
    EGAccountNotFound = 315,
    PaymasterNotFound = 317,
    ErrandNotFound = 319,
    ErrandAlreadyFunded = 320,
    InvalidErrandState = 321,
    UndisputedErrandState = 323,
    UnCompletedErrandState = 325,
    InvalidPercent = 327,
}
