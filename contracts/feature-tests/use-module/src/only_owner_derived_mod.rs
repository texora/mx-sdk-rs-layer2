dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait OnlyOwnerDerivedTestModule {
    #[view]
    fn call_derived_not_owner_only(&self) {}
}
