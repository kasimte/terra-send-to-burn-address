use cosmwasm_std::{
    Api, BankMsg, Binary, Coin, CosmosMsg, Decimal, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, Querier, StdResult, Storage, Uint128,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use terra_cosmwasm::TerraQuerier;

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::SendToBurnAccount {} => send_to_burn_account(deps, env),
    }
}

fn send_to_burn_account<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    // Query all coin balances for the input address.
    let balances: Vec<Coin> = deps.querier.query_all_balances(&env.contract.address)?;
    // Calculate the amount to be burnt, based on the tax rate.
    let amount = deduct_tax(deps, balances)?;
    // Process a message to send from the input address to the burn address.
    Ok(HandleResponse {
        messages: vec![CosmosMsg::Bank(BankMsg::Send {
            from_address: env.contract.address,
            to_address: HumanAddr::from("terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu"),
            amount,
        })],
        log: vec![],
        data: None,
    })
}

static DECIMAL_FRACTION: Uint128 = Uint128(1_000_000_000_000_000_000u128);
fn deduct_tax<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    coins: Vec<Coin>,
) -> StdResult<Vec<Coin>> {
    // Instantiate an object for querying the terra blockchain.
    let terra_querier = TerraQuerier::new(&deps.querier);
    // Fetch the tax rate.
    let tax_rate: Decimal = (terra_querier.query_tax_rate()?).rate;

    coins
        .into_iter()
        .map(|v| {
	    // There is cap on tax paid per transaction.
            let tax_cap: Uint128 = (terra_querier.query_tax_cap(v.denom.to_string())?).cap;

            Ok(Coin {
                amount: (v.amount
                    - std::cmp::min(
                        v.amount.multiply_ratio(
                            DECIMAL_FRACTION,
                            DECIMAL_FRACTION * tax_rate + DECIMAL_FRACTION,
                        ),
                        tax_cap,
                    ))?,
                denom: v.denom,
            })
        })
        .collect()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);
        let msg = InitMsg {};
        let env = mock_env("creator", &coins(1000, "earth"));
        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
