use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::collections::HashMap;

type Address = String;

#[derive(Clone, CandidType, Deserialize, Debug)]
struct Token {
    owner: Address,
    balance: u64,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
struct Wallet {
    tokens: HashMap<Address, Token>,
}

impl Wallet {
    fn new() -> Self {
        Wallet {
            tokens: HashMap::new(),
        }
    }

    fn balance_of(&self, owner: &Address) -> u64 {
        self.tokens.get(owner).map_or(0, |token| token.balance)
    }

    fn send_tokens(&mut self, from: Address, to: Address, amount: u64) -> Result<(), String> {
        let sender_balance = self.balance_of(&from);

        if sender_balance < amount {
            return Err(String::from("Insufficient balance."));
        }

        let receiver_balance = self.balance_of(&to);

        self.tokens.insert(
            from.clone(),
            Token {
                owner: from.clone(),
                balance: sender_balance - amount,
            },
        );

        self.tokens.insert(
            to.clone(),
            Token {
                owner: to.clone(),
                balance: receiver_balance + amount,
            },
        );

        Ok(())
    }

    fn receive_tokens(&mut self, receiver: Address, amount: u64) {
        let receiver_balance = self.balance_of(&receiver);

        self.tokens.insert(
            receiver.clone(),
            Token {
                owner: receiver.clone(),
                balance: receiver_balance + amount,
            },
        );
    }
}

static mut WALLET: Option<Wallet> = None;

#[update]
fn send(from: Address, to: Address, amount: u64) -> Result<(), String> {
    unsafe {
        WALLET.get_or_insert_with(Wallet::new).send_tokens(from, to, amount)
    }
}

#[update]
fn receive(receiver: Address, amount: u64) {
    unsafe {
        WALLET.get_or_insert_with(Wallet::new).receive_tokens(receiver, amount)
    }
}

#[query]
fn balance(owner: Address) -> u64 {
    unsafe { WALLET.get_or_insert_with(Wallet::new).balance_of(&owner) }
}

#[update]
fn receive(to: Address, amount: u64) {
    ic_cdk::println!("Receiving {} tokens to address: {}", amount, to);

}