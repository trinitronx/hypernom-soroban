#![cfg(test)]

extern crate std; // for println! macro in tests ONLY
use super::*;
// use soroban_sdk::{symbol_short, vec, Env};
use soroban_sdk::{testutils::Address as _, Address, Env};

// #[test]
// fn test() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, HelloContract);
//     let client = HelloContractClient::new(&env, &contract_id);

//     let words = client.hello(&symbol_short!("Dev"));
//     std::println!("Contract hello() returned: {:?}", words);
//     let expected = vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),];
//     std::println!("Expected: {:?}", expected);
//     let one_item = symbol_short!("Hello");
//     std::println!("One item: {:?}", one_item);
//     let one_item_vec = vec![&env, symbol_short!("Hello")];
//     std::println!("One item Vec: {:?}", one_item_vec);
//     assert_eq!(
//         words,
//         vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
//     );
// }

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, HypernomLeaderboardContract);
    let client = HypernomLeaderboardContractClient::new(&env, &contract_id);

    let address = Address::generate(&env);
    let player_name = String::from_str(&env, "Barney Rubble");
    // let result = client.add_player(&env, player_name, address); // Result<String, String>
    let result = client.add_player(&player_name, &address); // Result<String, String>
    std::println!("Contract hello() returned: {:?}", result);
    // let expected = vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),];
    // std::println!("Expected: {:?}", expected);
    // let one_item = symbol_short!("Hello");
    // std::println!("One item: {:?}", one_item);
    // let one_item_vec = vec![&env, symbol_short!("Hello")];
    // std::println!("One item Vec: {:?}", one_item_vec);
    std::assert_eq!(result, ());
    // assert_eq!(
    //     words,
    //     vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    // );
}
