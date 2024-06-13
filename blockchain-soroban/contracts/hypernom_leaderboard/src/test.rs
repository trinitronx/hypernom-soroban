#![cfg(test)]

extern crate std; // for println! macro in tests ONLY
use super::*;
// use soroban_sdk::{symbol_short, vec, Env};
use soroban_sdk::{testutils::Address as _, Address, Env, vec};
//use soroban_sdk::testutils::arbitrary::{arbitrary, Arbitrary};

/// Helper function to initialize the test environment and contract client.
fn _init_test_env() -> (Env, Address, HypernomLeaderboardContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register_contract(None, HypernomLeaderboardContract);
    let client = HypernomLeaderboardContractClient::new(&env, &contract_id);
    (env, contract_id, client)
}

#[test]
fn test_add_player() {
    let (env, _contract_id, client) = _init_test_env();
    env.mock_all_auths();

    let address = Address::generate(&env);
    let player_name = String::from_str(&env, "Barney Rubble");
    let result = client.add_player(&player_name, &address); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", result);

    std::assert_eq!(result, ());
}

#[test]
fn test_get_list() {
    let (env, _contract_id, client) = _init_test_env();

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
    let (env, _contract_id, client) = _init_test_env();

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
        let result = client.add_score(&address1, &level, &score);
        assert_eq!(
            result,
            expected
        )
    }
}


#[test]
fn test_get_scores() {
    let (env, _contract_id, client) = _init_test_env();

    env.mock_all_auths();

    let address = Address::generate(&env);
    let player_name = String::from_str(&env, "Barney Rubble");
    let _ = client.add_player(&player_name, &address); // Result<(), Error>
    let player_scores = vec![&env, (0u32, 5200i64), (1u32, 11700i64), (2u32, 11500i64), (3u32, 28200i64), (4u32, 82900i64), (5u32, 204600i64)];

    for (level, score) in player_scores.iter() {
        let _result = client.add_score(&address, &level, &score);
    }

    let result = client.get_scores(&address);
    std::println!("Contract get_scores() returned: {:?}", result.as_ref());
    std::assert_eq!(result.as_ref().unwrap().player_id, address);
    std::assert_eq!(result.as_ref().unwrap().scores.get(0).unwrap(), 5200);
    for (i, score) in (0u32..).zip(result.as_ref().unwrap().scores.iter()) {
        // std::println!("Level {}: {}", i, score);
        // std::println!("Level {}: {:?}", i, player_scores.get(i));
        assert!(player_scores.get(i).is_some());
        std::assert_eq!(score, player_scores.get(i).unwrap().1);
    }
}


/// Test adding scores the same player twice, and checking that the leaderboard
/// is updated correctly with only the fastest times for each level.
#[test]
fn test_add_score_2x_only_fastest_added() {
    let (env, _contract_id, client) = _init_test_env();

    let address1 = Address::generate(&env);
    std::println!("Address: {:?}", address1);
    let player1_name = String::from_str(&env, "Barney Rubble");
    let player1_result = client.add_player(&player1_name, &address1); // Result<(), Error>
    std::println!("Contract add_player() returned: {:?}", player1_result);

    // Lowest time for each level should win
    let player1_scores_old = vec![&env, (0u32, 5200i64), (1u32, 11700i64), (2u32, 11500i64), (3u32, 28200i64), (4u32, 82900i64), (5u32, 204600i64)];
    let player1_scores_new = vec![&env, (0u32, 5000i64), (1u32, 10000i64), (2u32, 20000i64), (3u32, 30000i64), (4u32, 90000i64), (5u32, 100000i64)];
    let expected_p1_update = vec![&env, (0u32, 5000i64), (1u32, 10000i64), (2u32, 11500i64), (3u32, 28200i64), (4u32, 82900i64), (5u32, 100000i64)];
    for (level, score) in player1_scores_old {
        let _first_result = client.add_score(&address1, &level, &score);
    }
    for (level, score) in player1_scores_new.iter() {
        let second_result = client.add_score(&address1, &level, &score);
        assert_eq!(
            second_result,
            ()
        )
    }
    let update_result = client.get_scores(&address1);
    for (i, score) in (0u32..).zip(update_result.as_ref().unwrap().scores.iter()) {
        std::println!("Level {}: {}", i, score);
        std::println!("Level {}: {:?}", i, expected_p1_update.get(i));
        assert!(expected_p1_update.get(i).is_some());
        std::assert_eq!(score, expected_p1_update.get(i).unwrap().1);
    }
}