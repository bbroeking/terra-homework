#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PriceResponse};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
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
        price: msg.price,
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
        ExecuteMsg::UpdatePrice { price } => execute_update_price(deps, env, info, price)
    }
}

pub fn execute_update_price(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    price: u64,
) -> Result<Response, ContractError>  {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.price = price;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "execute_update_price"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryPrice {} => to_binary(&query_oracle_info(deps)?)
    }
}

pub fn query_oracle_info(deps: Deps) -> StdResult<PriceResponse> {
    let info = STATE.load(deps.storage)?;
    let res = PriceResponse { price: info.price };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { price: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query_oracle_info(deps.as_ref()).unwrap();
        // assert_eq!(res, Err(StdError::generic_err("not implemented")));

        assert_eq!(res, PriceResponse { price: 17 });
    }

    #[test]
    fn update_price() {
        // init
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { price: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // check init
        let res = query_oracle_info(deps.as_ref()).unwrap();
        assert_eq!(res, PriceResponse { price: 17 });

        // update price
        let msg = ExecuteMsg::UpdatePrice { price: 19 };
        let info = mock_info("creator", &coins(1000, "earth"));

        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // let _res = execute_update_price(deps.as_mut(), mock_env(), info, 3).unwrap();

        // check updated
        let res2 = query_oracle_info(deps.as_ref()).unwrap();

        assert_eq!(res2, PriceResponse { price: 19 });
    }
}
