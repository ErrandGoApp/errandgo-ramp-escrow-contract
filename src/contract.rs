use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};

use crate::{
    access::{
        read_administrator, read_errandgo_account, read_paymaster, write_administrator,
        write_errandgo_account, write_paymaster,
    },
    asset_utils::{send_asset, take_asset},
    contract_traits::ErrandGoTrait,
    data::{
        ErrandData, ErrandState, Parameters, Rates, Reserve, INSTANCE_BUMP_AMOUNT,
        INSTANCE_LIFETIME_THRESHOLD,
    },
    errands::{
        read_errand, read_errand_funded, write_errand, write_remove_errand, write_update_errand,
    },
    error::ContractError,
    events::{ErrandStateEvent, OffRampEvent, OnRampEvent},
    payment_asset::{read_payment_asset, write_payment_asset},
    rates::{read_rates, write_rates},
    reserves::{read_reserve_deposit, write_add_reserve, write_remove_reserve},
};

fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        return Err(ContractError::NegativeAmount);
    }
    Ok(())
}

#[contract]
pub struct ErrandGo;

#[contractimpl]
impl ErrandGoTrait for ErrandGo {
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
    ) {
        if precision > 18 {
            panic!("precision must not be greater than 18");
        }
        write_administrator(&e, &admin);
        write_errandgo_account(&e, &errandgo_account);
        write_paymaster(&e, &paymaster);
        write_payment_asset(&e, &asset);
        write_rates(
            &e,
            off_ramp,
            on_ramp,
            tx_fee,
            platform_fee,
            commission_percent,
            precision,
        );
    }

    fn update_rates(
        e: Env,
        off_ramp: i128,
        on_ramp: i128,
        tx_fee: i128,
        platform_fee: i128,
        commission_percent: u32,
        precision: u32,
    ) -> Result<(), ContractError> {
        read_administrator(&e)?.require_auth();
        write_rates(
            &e,
            off_ramp,
            on_ramp,
            tx_fee,
            platform_fee,
            commission_percent,
            precision,
        );
        Ok(())
    }

    fn update_admin(e: Env, id: Address) -> Result<(), ContractError> {
        read_administrator(&e)?.require_auth();
        write_administrator(&e, &id);
        Ok(())
    }
    fn update_errandgo_account(e: Env, id: Address) -> Result<(), ContractError> {
        read_administrator(&e)?.require_auth();
        write_errandgo_account(&e, &id);
        Ok(())
    }
    fn update_paymaster(e: Env, id: Address) -> Result<(), ContractError> {
        read_administrator(&e)?.require_auth();
        write_paymaster(&e, &id);
        Ok(())
    }

    fn deposit_to_reserve(e: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();
        check_nonnegative_amount(amount)?;

        let asset = read_payment_asset(&e)?;
        take_asset(&e, &from, asset.clone(), amount);
        write_add_reserve(&e, asset, amount);
        Ok(())
    }
    fn withdraw_from_reserve(e: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        read_administrator(&e)?.require_auth();
        check_nonnegative_amount(amount)?;
        let asset = read_payment_asset(&e)?;
        send_asset(&e, &to, asset.clone(), amount);
        write_remove_reserve(&e, asset, amount)?;
        Ok(())
    }

    fn on_ramp(e: Env, to: Address, fiat_amount: i128) -> Result<(Address, i128), ContractError> {
        read_paymaster(&e)?.require_auth();
        check_nonnegative_amount(fiat_amount)?;
        let rates = read_rates(&e);

        let denominator = 10_i128
            .checked_pow(rates.precision)
            .ok_or(ContractError::Overflow)?;

        let numerator = fiat_amount
            .checked_mul(rates.on_ramp)
            .ok_or(ContractError::Overflow)?;

        let amount = numerator
            .checked_div(denominator)
            .ok_or(ContractError::Overflow)?;

        let asset = read_payment_asset(&e)?;
        send_asset(&e, &to, asset.clone(), amount);

        OnRampEvent {
            account: to.clone(),
            crypto_amount: amount,
        }
        .publish(&e);
        Ok((to, amount))
    }

    fn off_ramp(
        e: Env,
        from: Address,
        customer_id: String,
        amount: i128,
    ) -> Result<(String, i128), ContractError> {
        from.require_auth();
        // read_paymaster(&e)?.require_auth();
        check_nonnegative_amount(amount)?;
        let asset = read_payment_asset(&e)?;
        let rates = read_rates(&e);

        let total_amount = amount
            .checked_add(rates.tx_fee)
            .ok_or(ContractError::Overflow)?;

        take_asset(&e, &from, asset.clone(), total_amount);
        let errand_account = read_errandgo_account(&e)?;
        send_asset(&e, &errand_account, asset.clone(), rates.tx_fee);

        let denominator = 10_i128
            .checked_pow(rates.precision)
            .ok_or(ContractError::Overflow)?;

        let numerator = amount
            .checked_mul(rates.off_ramp)
            .ok_or(ContractError::Overflow)?;

        let fiat_amount = numerator
            .checked_div(denominator)
            .ok_or(ContractError::Overflow)?;

        OffRampEvent {
            customer_id: customer_id.clone(),
            fiat_amount: fiat_amount,
        }
        .publish(&e);

        Ok((customer_id, fiat_amount))
    }
    fn fund_errand_escrow(
        e: Env,
        errand_id: String,
        customer_id: String,
        customer_wallet: Address,
        runner_id: String,
        runner_wallet: Address,
        fiat_amount: i128,
    ) -> Result<(String, i128), ContractError> {
        if read_errand_funded(&e, errand_id.clone()) {
            return Err(ContractError::ErrandAlreadyFunded);
        }
        customer_wallet.require_auth();
        // read_paymaster(&e)?.require_auth();
        check_nonnegative_amount(fiat_amount)?;
        let rates = read_rates(&e);

        let denominator = 10_i128
            .checked_pow(rates.precision)
            .ok_or(ContractError::Overflow)?;

        let numerator = fiat_amount
            .checked_mul(rates.on_ramp)
            .ok_or(ContractError::Overflow)?;

        let amount = numerator
            .checked_div(denominator)
            .ok_or(ContractError::Overflow)?;

        let rate = read_rates(&e);
        let amount_total = amount
            .checked_add(rate.platform_fee)
            .ok_or(ContractError::Overflow)?;
        let asset = read_payment_asset(&e)?;
        take_asset(&e, &customer_wallet, asset.clone(), amount_total);

        write_errand(
            &e,
            errand_id.clone(),
            customer_id,
            customer_wallet,
            amount.clone(),
            runner_id.clone(),
            runner_wallet,
        );

        ErrandStateEvent {
            errand_id: errand_id.clone(),
            state: ErrandState::Funded,
        }
        .publish(&e);

        Ok((errand_id, amount))
    }

    fn mark_errand_completed(
        e: &Env,
        errand_id: String,
    ) -> Result<(String, ErrandState), ContractError> {
        read_paymaster(&e)?.require_auth();

        let mut errand: ErrandData = read_errand(e, errand_id.clone())?;

        match errand.state {
            ErrandState::Funded => {
                errand.state = ErrandState::Completed;
            }
            _ => return Err(ContractError::InvalidErrandState),
        }

        write_update_errand(e, errand_id.clone(), errand);

        ErrandStateEvent {
            errand_id: errand_id.clone(),
            state: ErrandState::Completed,
        }
        .publish(&e);

        Ok((errand_id, ErrandState::Completed))
    }

    fn mark_errand_disputed(
        e: &Env,
        errand_id: String,
    ) -> Result<(String, ErrandState), ContractError> {
        read_paymaster(&e)?.require_auth();

        let mut errand: ErrandData = read_errand(e, errand_id.clone())?;

        match errand.state {
            ErrandState::Funded => {
                errand.state = ErrandState::Disputed;
            }
            _ => return Err(ContractError::InvalidErrandState),
        }

        write_update_errand(e, errand_id.clone(), errand);

        ErrandStateEvent {
            errand_id: errand_id.clone(),
            state: ErrandState::Disputed,
        }
        .publish(&e);

        Ok((errand_id, ErrandState::Disputed))
    }

    fn release_errand_escrow(
        e: &Env,
        errand_id: String,
    ) -> Result<(String, ErrandState), ContractError> {
        let rates = read_rates(e);

        let runner_percent = 100_i128
            .checked_sub(rates.commission_percent as i128)
            .ok_or(ContractError::InvalidPercent)?;

        check_nonnegative_amount(runner_percent)?;

        read_paymaster(e)?.require_auth();

        let mut errand: ErrandData = read_errand(e, errand_id.clone())?;

        if errand.state != ErrandState::Completed {
            return Err(ContractError::UnCompletedErrandState);
        }

        let asset = read_payment_asset(e)?;

        let comm_amt = errand
            .amount
            .checked_mul(rates.commission_percent as i128)
            .ok_or(ContractError::Overflow)?
            .checked_div(100)
            .ok_or(ContractError::Overflow)?;
        let comm_total = rates
            .platform_fee
            .checked_add(comm_amt)
            .ok_or(ContractError::Overflow)?;

        let runner_amt = errand
            .amount
            .checked_mul(runner_percent)
            .ok_or(ContractError::Overflow)?
            .checked_div(100)
            .ok_or(ContractError::Overflow)?;

        let errand_go_acct = read_errandgo_account(e)?;
        send_asset(e, &errand_go_acct, asset.clone(), comm_total);

        send_asset(e, &errand.runner_wallet, asset, runner_amt);

        match errand.state {
            ErrandState::Completed => {
                errand.state = ErrandState::Released;
            }
            _ => return Err(ContractError::InvalidErrandState),
        }

        write_remove_errand(e, errand_id.clone())?;

        Ok((errand_id, ErrandState::Released))
    }

    fn resolve_disputed_errand(
        e: &Env,
        errand_id: String,
        cust_percent: u32,
    ) -> Result<(String, ErrandState), ContractError> {
        let rates = read_rates(e);
        let diff = rates
            .commission_percent
            .checked_add(cust_percent)
            .ok_or(ContractError::Overflow)?;

        let runner_percent = 100_i128
            .checked_sub(diff as i128)
            .ok_or(ContractError::InvalidPercent)?;
        check_nonnegative_amount(runner_percent)?;

        read_administrator(&e)?.require_auth();

        let mut errand: ErrandData = read_errand(e, errand_id.clone())?;

        if errand.state != ErrandState::Disputed {
            return Err(ContractError::UndisputedErrandState);
        }

        let asset = read_payment_asset(e)?;

        let comm_amt = errand
            .amount
            .checked_mul(rates.commission_percent as i128)
            .ok_or(ContractError::Overflow)?
            .checked_div(100)
            .ok_or(ContractError::Overflow)?;
        let comm_total = rates
            .platform_fee
            .checked_add(comm_amt)
            .ok_or(ContractError::Overflow)?;

        let cust_amt = errand
            .amount
            .checked_mul(cust_percent as i128)
            .ok_or(ContractError::Overflow)?
            .checked_div(100)
            .ok_or(ContractError::Overflow)?;
        let runner_amt = errand
            .amount
            .checked_mul(runner_percent)
            .ok_or(ContractError::Overflow)?
            .checked_div(100)
            .ok_or(ContractError::Overflow)?;

        let errand_go_acct = read_errandgo_account(e)?;
        send_asset(e, &errand_go_acct, asset.clone(), comm_total);

        send_asset(e, &errand.customer_wallet, asset.clone(), cust_amt);
        send_asset(e, &errand.runner_wallet, asset, runner_amt);

        match errand.state {
            ErrandState::Disputed => {
                errand.state = ErrandState::Resolved;
            }
            _ => return Err(ContractError::InvalidErrandState),
        }

        write_remove_errand(e, errand_id.clone())?;

        Ok((errand_id, ErrandState::Resolved))
    }

    fn get_reserve(e: Env) -> Result<Reserve, ContractError> {
        let asset = read_payment_asset(&e)?;
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        Ok(Reserve {
            deposit: read_reserve_deposit(&e, asset),
            balance: 0,
        })
    }

    fn get_rates(e: Env) -> Rates {
        let rates = read_rates(&e);

        rates
    }
    fn get_parameters(e: Env) -> Result<Parameters, ContractError> {
        let rates = read_rates(&e);

        Ok(Parameters {
            on_ramp: rates.on_ramp,
            off_ramp: rates.off_ramp,
            tx_fee: rates.tx_fee,
            platform_fee: rates.platform_fee,
            commission_percent: rates.commission_percent,
            precision: rates.precision,
            paymaster: read_paymaster(&e)?,
            errandgo_account: read_errandgo_account(&e)?,
        })
    }

    ///Upgrade Contract
    fn upgrade(e: Env, wasm: BytesN<32>) -> Result<(), ContractError> {
        read_administrator(&e)?.require_auth();
        e.deployer().update_current_contract_wasm(wasm.clone());
        Ok(())
    }
}
