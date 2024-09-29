use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    StdError,
};
use cw2::set_contract_version;
use crate::state::{State, STATE};
use crate::msg::InstantiateMsg;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { message: msg.message };
    STATE.save(deps.storage, &state)?;
    set_contract_version(deps.storage, "hello_world", "1.0.0")?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryUserList { user, start_after, limit } => {
            to_binary(&query_user_list(deps, user, start_after, limit)?)
        }
    }
}
