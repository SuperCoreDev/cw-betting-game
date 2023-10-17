#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,Uint128};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::execute::*;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::*;

use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cw-betting-game";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut config = Config::new(
        info.sender.clone(),
    );
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    //let admin_amount = cw_utils::must_pay(&info, "ustars")?.u128();
    config.set_balance(msg.admin_balance);

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender)
        .add_attribute("total_supply_of_admin" , config.admin_balance.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    //use ExecuteMsg::*;
    match msg {
        ExecuteMsg::BettingPlays{gambled_fund,is_win} => execute_playwin(deps,info,gambled_fund,is_win),
        ExecuteMsg::SetUserBalance { balance } => execute_setuserbalance(deps,info,balance),
        // ExecuteMsg::Stake { amount } => execute_stake(deps, env, info, amount),
        // ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, info, amount),
        // ExecuteMsg::ClaimReward {} => execute_claim_reward(deps, env, info),
        // ExecuteMsg::UpdateRewardRate { reward_rate } => {
        //     execute_update_reward_rate(deps, env, info, reward_rate)
        // }
        // ExecuteMsg::Receive(receive_msg) => execute_receive(deps, env, info, receive_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::UserInfo { user } => query_user_info(deps, user),
        // QueryMsg::RewardInfo { user } => query_reward_info(deps, env, user),
        // QueryMsg::RewardParameters {} => query_reward_parameters(deps),
    }
}
