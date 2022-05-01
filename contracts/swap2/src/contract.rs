#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, DistributionMsg, Empty, Env,
    MessageInfo, QueryRequest, Response, StakingMsg, StdError, StdResult, SubMsg, Uint128, WasmMsg,
    WasmQuery,
};
use cw0::must_pay;
use cw2::set_contract_version;
//use cw20::Cw20ExecuteMsg;
// use shared::oracle::PriceResponse;

use cw20::Cw20ExecuteMsg;
// use shared::querier::query_balance;
use terra_cosmwasm::{create_swap_msg, ExchangeRatesResponse, TerraMsgWrapper, TerraQuerier};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg};
use crate::querier::query_balance;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// BlockNgine - 0% comission on testnet
const VALIDATOR: &str = "terravaloper1ze5dxzs4zcm60tg48m9unp8eh7maerma38dl84";

// StakeBin - 1% comission on testnet
// https://finder.terra.money/testnet/validator/terravaloper19ne0aqltndwxl0n32zyuglp2z8mm3nu0gxpfaw
// const VALIDATOR: &str = "terravaloper19ne0aqltndwxl0n32zyuglp2z8mm3nu0gxpfaw";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(
        Response::new().add_submessage(SubMsg::new(CosmosMsg::Distribution(
            DistributionMsg::SetWithdrawAddress {
                address: env.contract.address.into_string(),
            },
        ))),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("not implemented"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Buy {} => try_buy(deps, env, info),
        ExecuteMsg::Withdraw { amount } => {
            try_withdraw_step1_collect_rewards(deps, env, info, amount)
        }
        ExecuteMsg::WithdrawStep2ConvertRewardsToLuna { amount } => {
            try_withdraw_step2_convert_all_native_coins_to_luna(deps, env, info, amount)
        }
        ExecuteMsg::WithdrawStep3SendLuna { amount } => {
            try_withdraw_step3_send_luna(deps, env, info, amount)
        }
        ExecuteMsg::StartUndelegation { amount } => try_start_undelegation(deps, env, info, amount),
    }
}

pub fn try_buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    let payment_amt =
        must_pay(&info, "uluna").map_err(|error| StdError::generic_err(format!("{}", error)))?;

    let oracle_addr = String::from("terra1w7qj7e0nd9wexuz4p2v69cwnt9nh5u76pqh4zw");
    let token_addr = String::from("terra18lmfqfdupmlpfktxc8vple3uf6yg6t00dguxzv");

    let response: PriceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: oracle_addr,
        msg: to_binary(&QueryMsg::QueryPrice {})?,
    }))?;

    let tokens = match payment_amt.checked_div(Uint128::from(response.price as u128)) {
        Ok(n) => n,
        Err(_) => return Err(ContractError::DivisionError {}),
    };

    Ok(Response::<TerraMsgWrapper>::new()
        .add_submessage(SubMsg::new(CosmosMsg::Staking(StakingMsg::Delegate {
            validator: VALIDATOR.to_string(),
            amount: Coin {
                denom: "uluna".to_string(),
                amount: payment_amt,
            },
        })))
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: tokens,
            })?,
            contract_addr: token_addr,
            funds: vec![],
        })]))
}

pub fn try_withdraw_step1_collect_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: u64,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    // Step 1: Collect all rewards we have accrued.
    let mut messages: Vec<SubMsg<TerraMsgWrapper>> = vec![];

    let mut reward_submessages = collect_all_rewards(deps, &env)?;
    messages.append(&mut reward_submessages);
    Ok(Response::new().add_submessages(messages))
}

pub fn collect_all_rewards(
    deps: DepsMut,
    env: &Env,
) -> Result<Vec<SubMsg<TerraMsgWrapper>>, ContractError> {
    let mut messages: Vec<SubMsg<TerraMsgWrapper>> = vec![];
    let delegations = deps
        .querier
        .query_all_delegations(env.contract.address.to_string());

    if let Ok(delegations) = delegations {
        for delegation in delegations {
            let msg: CosmosMsg<TerraMsgWrapper> = CosmosMsg::<TerraMsgWrapper>::Distribution(
                DistributionMsg::WithdrawDelegatorReward {
                    validator: delegation.validator,
                },
            );
            messages.push(SubMsg::<TerraMsgWrapper>::new(msg));
        }
    }

    Ok(messages)
}

pub fn try_withdraw_step2_convert_all_native_coins_to_luna(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: u64,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    let balance = query_balance(
        &deps.querier,
        &env.contract.address.clone(),
        "uusd".to_string(),
    )?;

    Ok(Response::new().add_messages(vec![create_swap_msg(
        Coin {
            denom: "uusd".to_string(),
            amount: balance,
        },
        "uluna".to_string(),
    )]))
}

pub fn try_withdraw_step3_send_luna(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: u64,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    let new_amount = Uint128::from(amount);

    Ok(
        Response::new().add_messages(vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: "uluna".to_string(),
                amount: new_amount,
            }],
        })]),
    )
}

pub fn try_start_undelegation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response<TerraMsgWrapper>, ContractError> {
    let mut messages: Vec<SubMsg<TerraMsgWrapper>> = vec![];
    let delegations = deps
        .querier
        .query_all_delegations(env.contract.address.to_string());
    if let Ok(delegations) = delegations {
        for delegation in delegations {
            let msg: CosmosMsg<TerraMsgWrapper> =
                CosmosMsg::<TerraMsgWrapper>::Staking(StakingMsg::Undelegate {
                    validator: VALIDATOR.to_string(),
                    amount: Coin {
                        denom: "uluna".to_string(),
                        amount: delegation.amount.amount,
                    },
                });
            messages.push(SubMsg::<TerraMsgWrapper>::new(msg));
        }
    }

    Ok(Response::new().add_submessages(messages))
}

pub fn query_exchange_rates(
    deps: &DepsMut,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            token_address: Addr::unchecked("terra1hpajld8zs93md8zrs6sfy42zl0khqpmr07muw0"),
        };
        let info = mock_info("creator", &coins(10000000000, "uluna"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryTokenAddress {});
        assert_eq!(res, Err(StdError::generic_err("not implemented")));

        // let value: QueryTokenAddressResponse = from_binary(&res).unwrap();
        // assert_eq!(
        //     "terra1hpajld8zs93md8zrs6sfy42zl0khqpmr07muw0",
        //     value.token_address
        // );
    }
}
