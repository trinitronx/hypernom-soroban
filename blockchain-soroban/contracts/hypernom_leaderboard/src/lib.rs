#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, contracterror, Env, Address, String, Vec, vec, Map, log};

// Unused: symbol_short, vec, Symbol

/// Standard error codes for the Hypernom Leaderboard contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The player already exists.
    PlayerAlreadyExists = 1,
}

// Seems that CONTRACT_DATA is already implemented as Key -> value store...
// So, we don't need a top-level struct or Map type, just use a keyed enum
/// The persistent data keys for the Hypernom Leaderboard contract.
///
/// Currently stored as `persistent` type in Stellar `CONTRACT_DATA` ledger entries
#[contracttype]
pub enum DataKey {
    // TODO: Decide if we need any of these helpers or if we can just use the Players map
    /// Maps the player's wallet address to their name.
    PlayerId(Address), // Wallet pubkey ID
    /// Maps the player's name to their address.
    PlayerName(String), // user-provided name
    /// A map of player IDs to player names.
    Players,          // Map of player ID -> Player Name Map<Address, String>
    /// A map of player IDs to their scores for each level.
    Scores(Address), // Player's scores (Vec<i64>)
}

/// A struct to store a player's scores.
///
/// Stores the player's wallet address and a list of scores for each level.
#[contracttype]
#[derive(Debug)]
pub struct Scores {
    player_id: Address,
    scores: Vec<i64>, // Unix timestamps (in ms) for each game level
    // Hypernom game currently has 6 levels
}

/// # The Leaderboard contract for Hypernom
#[contract]
pub struct HypernomLeaderboardContract;

/// The implementation of the Hypernom Leaderboard contract.
///
/// This contract allows players to register and submit their scores.
#[contractimpl]
impl HypernomLeaderboardContract {

    // Test using persistent storage
    /// Add a player to the leaderboard.
    ///
    /// # Arguments
    /// * `player_name` - The name of the player.
    /// * `player_id` - The player's wallet address.
    /// * `env` - The contract environment.
    ///
    /// # Returns
    /// * `Result<(), Error>` - The result of the operation.
    ///   If the player already exists, an error is returned. (`Error::PlayerAlreadyExists`)
    ///   If the player does not exist, it is added and `Ok(())` is returned.
    pub fn add_player(env: Env, player_name: String, player_id: Address) -> Result<(), Error> {
        // player_id.require_auth();
        if env.storage().persistent().has(&DataKey::PlayerId(player_id.clone())) {
            log!(&env, "PlayerId {} already exists.", player_id);

            return Err(Error::PlayerAlreadyExists);
        }
        // Add the player to the list of players.
        let mut players: Map<Address, String> = env.storage().persistent().get(&DataKey::Players)
        .unwrap_or(Map::new(&env));
        players.set(player_id.clone(), player_name);
        env.storage().persistent().set(&DataKey::Players, &players);
        Ok(())
    }

    /* TODO: Implement players array, scores, leaderboard sort, etc... */
    pub fn get_list(env: Env) -> Map<Address, String> {
        // todo!("Implement get_list");
        env.storage().persistent().get(&DataKey::Players)
        .unwrap_or(Map::new(&env))
    }
    pub fn get_player(env: Env, player_id: Address) -> Option<String> {
        todo!("Implement get_player");
        // env.storage().persistent().get(&DataKey::PlayerId(player_id))
    }
    pub fn get_scores(env: Env, player_id: Address) -> Option<Scores> {
        env.storage().persistent().get(&DataKey::Scores(player_id)).unwrap_or(None)
    }
    pub fn add_score(env: Env, player_id: Address, level: u32, score: i64) {
        // todo!("Implement set_scores");
        let mut scores = env.storage().persistent().get(&DataKey::Scores(player_id.clone())).unwrap_or(Scores {
            player_id: player_id.clone(),
            scores: Vec::from_array(&env, [0i64; 6]), // Initialize with 0 scores for each level
        });
        match scores.scores.get(level) {
            Some(old_score) => {
                // Only update the score if it is better than the current score,
                // or if the old_score is still the initial 0.
                // A real game can't have a time score of 0 ms, so this is a
                // safe initial value.
                if old_score == 0 || score <= old_score {
                    scores.scores.set(level, score);
                }
                else {
                    log!(&env, "Score {} is not better than the current score {} for level {}.", score, scores.scores.get(level), level);
                }
            },
            None => { }
        }

        env.storage().persistent().set(&DataKey::Scores(player_id), &scores);
    }
}

mod test;
