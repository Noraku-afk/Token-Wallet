Token Wallet - Steps to Run the Project

This guide will help you set up, run, and test the Token Wallet project on a local ICP (Internet Computer Protocol) network.

Prerequisites :

Before starting, make sure you have the following installed:

1. Rust
2. DFX (Dfinity SDK)
3. Node.js
4. Git: For cloning the repository

Steps to Run the Project :-

1. Clone the Project Repository

Open a terminal and run the following command to clone the project:

bash
git clone https://github.com/Noraku-afk/token_wallet.git
cd token_wallet


2. Install Dependencies

Navigate to the frontend directory and install the Node.js dependencies:

bash
cd src/token_wallet_frontend
npm install
cd ../../


3. Start the Local ICP Network

You need to start the Internet Computer local replica. Run the following command:

bash
dfx start --background


4. Build the Canisters

Build the smart contract backend:

bash
dfx build token_wallet_backend


5. Deploy the Canisters

Deploy the backend and frontend canisters to the local ICP network:

bash
dfx deploy


After deploying, you will see the canister IDs for both `token_wallet_backend` and `token_wallet_frontend`. These canisters will be used to interact with the smart contracts and frontend UI.

6. Interacting with the Token Wallet (via CLI)

Use these commands to interact with the token wallet:

- Send Tokens :
  
  bash
  dfx canister call token_wallet_backend send "(\"br5f7-7uaaa-aaaaa-qaaca-cai\", \"r4nzy-faeaa-aaaaa-qaaca-cai\", 100:nat64)"
  

- Receive Tokens :

  bash
  dfx canister call token_wallet_backend receive "(\"r4nzy-faeaa-aaaaa-qaaca-cai\", 100)"
  

- Check Token Balance :

  bash
  dfx canister call token_wallet_backend balance "(\"owner-address\")"
  

7. Running Unit Tests

To verify the functionality of the smart contracts, run the unit tests:

bash
cargo test


The tests will validate the sending, receiving, and balance-checking functionalities of the wallet.


Troubleshooting

- DFX Commands Not Working : Make sure that `dfx start` is running in the background.
- Smart Contract Errors : Check that all dependencies are installed, and try running `dfx deploy` again.
- Unit Test Failures : Ensure your local ICP environment is properly set up and Rust is installed.

