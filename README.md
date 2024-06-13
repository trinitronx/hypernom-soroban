# Hypernom Soroban

Hypernom Soroban is a leaderboard system implemented as a Smart Contract for use with the Solana SDK on the Stellar blockchain. It is written in Rust and designed to be used in gaming applications for maintaining player scores in a decentralized and transparent manner.

The current proof-of-concept game is [Hypernom][1], a game in which the player
moves around a non-euclidean geometry space in order to "`nom`" all the 3D geometric sections. The hyperbolic geometry
is projected down into 3D, but the movement through the 4D curved space allows the player to explore this unconventional space and shapes. Originally written by Henry Segerman, Vi Hart, and Andrea Hawksley, and Marc ten Bosch.

## Features

- Player score recording: Each player's scores are stored on the blockchain, ensuring transparency and reliability.
- Leaderboard generation: Get the leaderboard for a specific level, sorted by scores.
- Player management: Manage player names and IDs.

## Usage

This project includes a smart contract that is a Rust library which compiles to WebAssembly.
It is designed to be deployed as a smart contract on the Stellar blockchain using Soroban.

## Development

This project uses `#![no_std]` Rust, which means it does not use the Rust standard library. This is a requirement for Smart Contracts on the Stellar blockchain.  For more information, see the [Sorobon docs: Contract Development -> Contract Rust Dialect][2]

## Contributing

Contributions are welcome! Please submit a pull request or create an issue to get started.

## License

This project is licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for details.


[1]: http://hypernom.com
[2]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/rust-dialect
