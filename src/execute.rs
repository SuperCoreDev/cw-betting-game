use cw20::{Cw20ReceiveMsg, Denom};

use cosmwasm_std::{Addr,DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, Uint64, BankMsg, Coin};

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{Config,CONFIG,UserState,USER_STATES};
use serde_json;

use crate::contract::execute;

// pub fn execute_update_admin(
//     mut deps: DepsMut,
//     info: MessageInfo,
//     admin : Addr,
// ) -> Result<Response, ContractError> {
    
// }

pub fn execute_playwin(
    deps: DepsMut,
    info: MessageInfo,
    gambled_fund: Uint128,
    is_win : bool,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // update_rewards(deps.branch(), &env)?;
    // update_rewards_for_user(deps.branch(), &env, &info.sender.to_string())?;
    
    let mut user_state = USER_STATES
        .may_load(deps.storage, info.sender.clone())?
        .unwrap_or_default();

    // user_state.staked_amount += amount;
    // let user_amount = cw_utils::must_pay(&info, "ustars")?.u128();
    // user_state.set_balance(user_amount.into());

    // config.total_staked += amount;
    let reward:Uint128  = gambled_fund/Uint128::from(20u128);
    let messages: BankMsg;
    if config.admin_balance <= reward
    {
            return Err(ContractError::Std(StdError::generic_err("Insufficient fffunds")));
    }
    if is_win
    {
        config.admin_balance -= reward;
        user_state.user_balance += reward;
        // messages = BankMsg::Send { to_address: info.sender.to_string(), amount: vec![Coin{
        //     denom:"ustars".to_string() , amount:reward.into()
        // }] }
    }
    else
    {
        if user_state.user_balance <= reward
        {
            return Err(ContractError::Std(StdError::generic_err("Insufficient Gambled Token")));
        }
        config.admin_balance += reward;
        user_state.user_balance -= reward;
        // messages = BankMsg::Send { to_address: config.admin.to_string(), amount: vec![Coin{
        //     denom:"ustars".to_string() , amount:reward.into()
        // }] }
    }

    CONFIG.save(deps.storage, &config)?;
    USER_STATES.save(deps.storage, info.sender.clone(), &user_state)?;

    Ok(Response::new()
        .add_attribute("action", "betting success")
        .add_attribute("admin_amount", config.admin_balance)
        .add_attribute("user", info.sender.as_str())
        .add_attribute("user_amount",user_state.user_balance))
        //.add_message(messages))
}

pub fn execute_setuserbalance(
    deps:DepsMut,
    info:MessageInfo,
    balance:Uint128,
 ) -> Result<Response, ContractError> {

    let mut user_state = USER_STATES
        .may_load(deps.storage, info.sender.clone())?
        .unwrap_or_default();

    user_state.set_balance(balance);
    USER_STATES.save(deps.storage, info.sender.clone(), &user_state)?;
    Ok(Response::new()
        .add_attribute("action", "set_user_balance")
        .add_attribute("user_amount",user_state.user_balance))
}

// pub fn execute_updateadmin(
//     mut deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     admin : Addr,
// ) ->  Result<Response, ContractError> {
//     let mut config = CONFIG.load(deps.storage)?;
//     config.admin = admin;
//     config.total_reward
// }
// pub fn execute_withdraw(
//     mut deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     amount: Uint128,
// ) -> Result<Response, ContractError> {
//     let mut config = CONFIG.load(deps.storage)?;

//     // update_rewards(deps, &info.sender, &mut config, env.block.time.seconds())?;

//     update_rewards(deps.branch(), &env)?;

//     let mut user_state = USER_STATES.load(deps.storage, info.sender.clone())?;
//     if user_state.staked_amount < amount {
//         return Err(ContractError::Std(StdError::generic_err(
//             "Insufficient staked_amount",
//         )));
//     }

//     user_state.staked_amount -= amount;
//     USER_STATES.save(deps.storage, info.sender.clone(), &user_state)?;

//     config.total_staked -= amount;
//     CONFIG.save(deps.storage, &config)?;

//     Ok(Response::new()
//         .add_attribute("action", "withdraw")
//         .add_attribute("user", info.sender.to_string())
//         .add_attribute("amount", amount.to_string()))
// }

// pub fn execute_claim_reward(
//     mut deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
// ) -> Result<Response, ContractError> {
//     update_rewards(deps.branch(), &env)?;

//     let mut user_state = USER_STATES
//         .may_load(deps.storage, info.sender.clone())?
//         .unwrap_or_default();
//     let reward = user_state.reward;
//     user_state.reward = Uint128::zero();
//     USER_STATES.save(deps.storage, info.sender.clone(), &user_state)?;

//     Ok(Response::new()
//         .add_attribute("action", "claim_reward")
//         .add_attribute("user", info.sender.to_string())
//         .add_attribute("reward", reward.to_string()))
// }

// pub fn execute_update_reward_rate(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     reward_rate: Uint64,
// ) -> Result<Response, ContractError> {
//     let mut config = CONFIG.load(deps.storage)?;

//     if info.sender != config.admin {
//         return Err(ContractError::Std(StdError::generic_err("Unauthorized")));
//     }

//     config.reward_rate = reward_rate;
//     CONFIG.save(deps.storage, &config)?;

//     Ok(Response::new()
//         .add_attribute("action", "update_reward_rate")
//         .add_attribute("reward_rate", reward_rate.to_string()))
// }

// //################################
// //must take a look back about it.#
// //################################
// pub fn execute_receive(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     receive_msg: Cw20ReceiveMsg,
// ) -> Result<Response, ContractError> {
//     let msg: ExecuteMsg = serde_json::from_slice(receive_msg.msg.as_slice())?;
//     match m ExecuteMsg::Stake {
//                 amount: receive_msg.amount,
//             };
//             execute(deps, env, info, exec_sg {
//         ExecuteMsg::Stake { .. } => {
//             let exec_msg =msg)
//         }
//         _ => Err(ContractError::Std(StdError::generic_err(
//             "Unsupported cw20 receive message",
//         ))),
//     }
// }
// pub fn update_rewards(deps: DepsMut, env: &Env) -> StdResult<()> {
//     let mut config: Config = CONFIG.load(deps.storage)?;

//     let current_time: Uint128 = env.block.time.seconds().into();
//     let time_since_last_update: Uint128 = current_time - Uint128::from(config.last_update_time);

//     let reward_amount: Uint128 = time_since_last_update * Uint128::from(config.reward_rate);

//     config.total_staked += reward_amount;
//     config.last_update_time = env.block.time.seconds().into();

//     CONFIG.save(deps.storage, &config)?;

//     Ok(())
// }

// pub fn update_rewards_for_user(mut deps: DepsMut, env: &Env, user: &str) -> StdResult<()> {
//     update_rewards(deps.branch(), env)?;

//     let user_addr = deps.api.addr_validate(user)?;

//     let config: Config = CONFIG.load(deps.storage)?;
//     let mut user_state: UserState = USER_STATES
//         .may_load(deps.storage, user_addr.clone())?
//         .unwrap_or_default();

//     let time_since_last_update: Uint128 =
//         Uint128::from(env.block.time.seconds()) - Uint128::from(config.last_update_time);

//     let reward_per_token =
//         (Uint128::from(config.reward_rate) * time_since_last_update) / config.total_staked;

//     let rewards_diff = reward_per_token - user_state.reward_per_token_paid;

//     if rewards_diff > Uint128::zero() {
//         let user_reward = user_state.staked_amount * rewards_diff;
//         user_state.reward = user_state.reward + user_reward;
//     }

//     user_state.reward_per_token_paid = reward_per_token;
//     USER_STATES.save(deps.storage, user_addr.clone(), &user_state)?;

//     Ok(())
// }
