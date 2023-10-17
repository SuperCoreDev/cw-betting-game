use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
//use cw20::Cw20ReceiveMsg;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_balance: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Stake { amount: Uint128 },
    // Withdraw { amount: Uint128 },
    // ClaimReward {},
    // UpdateRewardRate { reward_rate: Uint64 },
    BettingPlays{gambled_fund : Uint128 , is_win : bool},
    SetUserBalance{balance : Uint128},
    //Receive(Cw20ReceiveMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config{},
    #[returns(UserInfoResponse)]
    UserInfo { user: String },
    // #[returns(RewardInfoResponse)]
    // RewardInfo { user: String },
    // #[returns(RewardParametersResponse)]
    // RewardParameters {},
}

#[cw_serde]
pub struct UserInfoResponse {
    pub user_balance: Uint128,
}

// #[cw_serde]
// pub struct RewardInfoResponse {
//     pub reward: Uint128,
// }

#[cw_serde]
pub struct ConfigResponse {
    pub admin: Addr,
    pub admin_balance: Uint128,
}

// #[cw_serde]
// pub struct RewardParametersResponse {
//     pub reward_rate: Uint64,
//     pub last_update_time: Uint64,
// }
