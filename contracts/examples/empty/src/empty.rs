#![no_std]

dharitri_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[dharitri_wasm::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}
}
