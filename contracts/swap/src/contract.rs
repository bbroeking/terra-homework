#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, ContractInfo, CosmosMsg, Deps, DepsMut, Empty, Env,
    MessageInfo, QueryRequest, Response, StdError, StdResult, Uint128, WasmMsg, WasmQuery,
};
use cw2::set_contract_version;
pub use cw20::Cw20ExecuteMsg;
use cw20::{BalanceResponse, Cw20ReceiveMsg};
use shared::oracle::PriceResponse;
use shared::querier::query_balance;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        token_address: msg.token_address,
        owner: info.sender,
    };

    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy {} => execute_buy(deps, env, info),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env, info),
        ExecuteMsg::WithdrawSome { amount } => execute_withdraw_some(deps, env, info)
        // ExecuteMsg::Receive(msg) => execute_receive_cw20(deps, env, info, msg), // ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, info, amount),
    }
}

pub fn execute_withdraw_some(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::new())
}
// pub fn execute_receive_cw20(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: Cw20ReceiveMsg,
// ) -> Result<Response, ContractError> {
//     Ok(Response::new())
// }

pub fn execute_buy(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let oracle_addr = String::from("terra1w7qj7e0nd9wexuz4p2v69cwnt9nh5u76pqh4zw");
    let token_addr = String::from("terra18lmfqfdupmlpfktxc8vple3uf6yg6t00dguxzv");

    let response: PriceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: oracle_addr,
        msg: to_binary(&QueryMsg::QueryPrice {})?,
    }))?;

    let funds_sent = info
        .funds
        .iter()
        .find(|x| x.denom == "uluna".to_string())
        .map(|x| Uint128::from(x.amount))
        .unwrap_or(Uint128::from(0 as u128));

    let tokens = match funds_sent.checked_div(Uint128::from(response.price as u128)) {
        Ok(n) => n,
        Err(_) => return Err(ContractError::DivisionError {}),
    };

    // let balance_response: BalanceResponse =
    // deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
    //     contract_addr: String::from(contract.address),
    //     msg: to_binary(&cw20::Cw20QueryMsg::Balance {
    //         address: token_addr,
    //     })?,
    // }))?;

    // let balance = query_balance(
    //     &deps.querier,
    //     &env.contract.address.clone(),
    //     "uusd".to_string(),
    // )?;

    // if balance < tokens {
    //     return Err(ContractError::NotEnoughTokenToSwap);
    // }

    Ok(Response::new()
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: tokens,
            })?,
            contract_addr: token_addr,
            funds: vec![],
        })])
        .add_attribute("method", "execute_buy"))

    // Ok(Response::new()
    //     .add_messages(vec![CosmosMsg::Bank(BankMsg::Send {
    //         to_address: info.sender.to_string(),
    //         amount: vec![Coin {
    //             denom: "uusd".to_string(),
    //             amount: tokens,
    //         }],
    //     })])
    //     .add_attribute("method", "execute_buy"))

    // let res = execute_transfer_from(
    //     deps,
    //     env,
    //     info,
    //     contract.address.into_string(),
    //     sender.to_string(),
    //     tokens);
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let balance = query_balance(
        &deps.querier,
        &env.contract.address.clone(),
        "uluna".to_string(),
    )?;

    Ok(
        Response::new().add_messages(vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: state.owner.to_string(),
            amount: vec![Coin {
                denom: "uluna".to_string(),
                amount: balance,
            }],
        })]),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr};

    #[test]
    fn proper_initialization() {
        // let mut deps = mock_dependencies(&[]);

        // let msg = InstantiateMsg {
        //     token_address: Addr::unchecked("terra13v3ncryrfhpfrk6a2lvsn0uhnz64lwqxwqt2tp"),
        // };
        // let info = mock_info("creator", &coins(1000, "uluna"));

        // // we can just call .unwrap() to assert this was a success
        // let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        // assert_eq!(0, res.messages.len());

        // // buy
        // let mut deps = mock_dependencies(&[]);
        // let msg = ExecuteMsg::Buy {};
        // let info = mock_info("creator", &coins(1000, "uluna"));
        // let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // query
    }
}
