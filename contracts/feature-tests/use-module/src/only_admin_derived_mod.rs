dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait OnlyAdminDerivedTestModule {
    #[view]
    fn call_derived_not_admin_only(&self) {}
}
