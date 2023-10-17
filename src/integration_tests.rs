#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, Uint64};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor, BankKeeper};

    use crate::msg::{
        ExecuteMsg, InstantiateMsg, QueryMsg,ConfigResponse, UserInfoResponse,
        //RewardInfoResponse, RewardParametersResponse,UserInfoResponse,
    };

    pub fn contract_staking_rewards() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }
//stars1f84h4xeyzfq4lsrrrwzhuffqykw4n3upzslu84
    const ADMIN: &str = "admin";
    const USER: &str = "user";
    // const STAKING_TOKEN: &str = "staking_token";
    // const REWARD_TOKEN: &str = "reward_token";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            let bank = BankKeeper::new();
                bank.init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: "ustars".to_string(),
                        amount: Uint128::new(300),
                    }]
                )
                .unwrap();
                bank.init_balance(
                    storage,
                    &Addr::unchecked(ADMIN),
                    vec![Coin {
                        denom: "ustars".to_string(),
                        amount: Uint128::new(500),
                    }]
                )
                .unwrap();
            router.bank = bank;
        })
    }

    fn proper_instantiate() -> (App, Addr) {
        let mut app = mock_app();
        let cw_staking_rewards_id = app.store_code(contract_staking_rewards());
        
        let msg = InstantiateMsg {
            admin_balance: Uint128::new(500),
        };
        let contract_addr = app
            .instantiate_contract(
                cw_staking_rewards_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "staking_rewards",
                None,
            )
            .unwrap();

        (app, contract_addr)
    }

    #[test]
    fn stake_and_query() {
        let (mut app, contract_addr) = proper_instantiate();
        let user = Addr::unchecked(USER);
        
        let setuserbalancemsg = ExecuteMsg::SetUserBalance { balance: Uint128::new(300) };
        app.execute_contract(user.clone(), contract_addr.clone(), &setuserbalancemsg, &[])
            .unwrap();

        let stake_msg = ExecuteMsg::BettingPlays {
            gambled_fund: Uint128::new(100),
            is_win : false
        };

        // // User stakes 100 tokens
        app.execute_contract(user.clone(), contract_addr.clone(), &stake_msg, &[])
            .unwrap();

        
        // let user_info_query = QueryMsg::UserInfo {
        //     user: user.to_string(),
        // };
        // let query_user_info : UserInfoResponse = app.wrap()
        //     .query_wasm_smart(&contract_addr, &user_info_query)
        //     .unwrap();
        //println!("Current Admin Balance :{}",query_user_info.user_balance.to_string());
        //assert_eq!(query_user_info.user_balance , Uint)
        
        // let reward_info_query = QueryMsg::RewardInfo {
        //     user: user.to_string(),
        // };

        // let reward_parameters_query = QueryMsg::RewardParameters {};
        // Query staked amount and reward
        // let config_info_query = QueryMsg::Config{};
        // let user_info_query = QueryMsg::UserInfo { user: USER.to_string()};
        // let query_config : ConfigResponse = app
        //     .wrap()
        //     .query_wasm_smart(&contract_addr, &config_info_query)
        //     .unwrap();
        // let query_user_info : UserInfoResponse = app
        //     .wrap()
        //     .query_wasm_smart(&contract_addr, &user_info_query)
        //     .unwrap();
        // let user_info: UserInfoResponse = app
        //     .wrap()
        //     .query_wasm_smart(&contract_addr, &user_info_query)
        //     .unwrap();
        // let reward_info: RewardInfoResponse = app
        //     .wrap()
        //     .query_wasm_smart(&contract_addr, &reward_info_query)
        //     .unwrap();

        // let reward_parameters: RewardParametersResponse = app
        //     .wrap()
        //     .query_wasm_smart(&contract_addr, &reward_parameters_query)
        //     .unwrap();

        // let current_time = Uint64::from(app.block_info().time.seconds());
        // let time_since_last_update = current_time - reward_parameters.last_update_time;
	    // println!("Current total_reward :{}",query_config.admin_balance);
        // println!("User total amount :{}",query_user_info.user_balance);
        // // println!("Current time : {}", current_time);
        // // println!("Reward rate: {}", reward_parameters.reward_rate);
        // // println!("Time since last update: {}", time_since_last_update);
        // // println!("Reward: {}", reward_info.reward);

        // assert_eq!(query_config.admin_balance, Uint128::new(5));
        

        // Due to the time it takes to run the test, the reward may not be exactly 0.
        // Check if the reward is within an acceptable range (e.g., 0 to 10).
        // assert!(
        //     reward_info.reward >= Uint128::from(0u128)
        //         && reward_info.reward <= Uint128::from(10u128),
        //     "Reward should be within 0 to 10, but got {}",
        //     reward_info.reward
        // );
    }
}
