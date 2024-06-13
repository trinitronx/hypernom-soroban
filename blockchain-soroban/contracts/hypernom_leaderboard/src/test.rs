#![cfg(test)]

extern crate std; // for println! macro in tests ONLY
use super::*;
// use soroban_sdk::{symbol_short, vec, Env};
use soroban_sdk::{testutils::Address as _, Address, Env, vec};
//use soroban_sdk::testutils::arbitrary::{arbitrary, Arbitrary};

#[test]
fn test_add_player() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HypernomLeaderboardContract);
    let client = HypernomLeaderboardContractClient::new(&env, &contract_id);

    let address = Address::generate(&env);
    let player_name = String::from_str(&env, "Barney Rubble");
    let result = client.add_player(&player_name, &address); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", result);

    std::assert_eq!(result, ());
}

#[test]
fn test_get_list() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HypernomLeaderboardContract);
    let client = HypernomLeaderboardContractClient::new(&env, &contract_id);

    let address1 = Address::generate(&env);
    std::println!("Address: {:?}", address1);
    let player1_name = String::from_str(&env, "Barney Rubble");
    let player1_result = client.add_player(&player1_name, &address1); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", player1_result);

    // Somehow arbitrary address generation is not working... so we'll just hardcode another randomly generated address
    let address2 = Address::from_string(&String::from_str(&env, "GCXSYUGCPEW44PLHCPVZXVTSG4O6FKUSEYXJO5POCLSMD7MUZ4CNPMKV"));

    let player2_name = String::from_str(&env, "Fred Flintstone");
    let player2_result = client.add_player(&player2_name, &address2); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", player2_result);

    let result = client.get_list();
    std::println!("Contract get_list() returned: {:?}", result);

    let mut expected = Map::new(&env);
    expected.set(address1.clone(), player1_name.clone());
    expected.set(address2.clone(), player2_name.clone());
    assert_eq!(
        result,
        expected
        // env.storage().persistent().get(&DataKey::Players).unwrap_or(Map::new(&env))
    );
}

#[test]
fn test_add_score() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HypernomLeaderboardContract);
    let client = HypernomLeaderboardContractClient::new(&env, &contract_id);

    let address1 = Address::generate(&env);
    std::println!("Address: {:?}", address1);
    let player1_name = String::from_str(&env, "Barney Rubble");
    let player1_result = client.add_player(&player1_name, &address1); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", player1_result);

    // Somehow arbitrary address generation is not working... so we'll just hardcode another randomly generated address
    let address2 = Address::from_string(&String::from_str(&env, "GCXSYUGCPEW44PLHCPVZXVTSG4O6FKUSEYXJO5POCLSMD7MUZ4CNPMKV"));

    let player2_name = String::from_str(&env, "Fred Flintstone");
    let player2_result = client.add_player(&player2_name, &address2); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", player2_result);

    let result = client.get_list();
    std::println!("Contract get_list() returned: {:?}", result);

    let player1_scores = vec![&env, (0u32, 5200i64), (1u32, 11700i64), (2u32, 11500i64), (3u32, 28200i64), (4u32, 82900i64), (5u32, 204600i64)];
    // let player2_scores = vec![(1, 5000),  (2, 200), (3, 300)];
    let expected = ();
    for (level, score) in player1_scores {
        let mut result = client.add_score(&address1, &level, &score);
        assert_eq!(
            result,
            expected
        )
    }
}