use dharitri_wasm::{api::ManagedTypeApi, types::BigUint};

dharitri_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum UnlockType<M: ManagedTypeApi> {
    FixedAmount {
        period_unlock_amount: BigUint<M>,
        release_period: u64,
        release_ticks: u64,
    },
    Percentage {
        period_unlock_percentage: u8,
        release_period: u64,
        release_ticks: u64,
    },
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, TypeAbi, Clone)]
pub struct Schedule<M: ManagedTypeApi> {
    pub group_total_amount: BigUint<M>,
    pub unlock_type: UnlockType<M>,
}
