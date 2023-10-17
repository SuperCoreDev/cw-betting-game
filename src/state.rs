use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use std::default::Default;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub admin_balance: Uint128,
}

impl Config {
    pub fn new(
        admin: Addr,
    ) -> Config {
        Config {
            admin,
            admin_balance: Uint128::zero(),
        }
    }
    pub fn set_balance(&mut self , balance : Uint128){
        self.admin_balance = balance;
    }
}
#[cw_serde]
pub struct UserState {
    pub user_balance : Uint128,
}

impl UserState{
    pub fn set_balance(&mut self, balance:Uint128){
        self.user_balance = balance;
    }
}
impl Default for UserState {
    fn default() -> Self {
        UserState {
            user_balance : Uint128::zero(),
        }
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const USER_STATES: Map<Addr, UserState> = Map::new("user_states");
