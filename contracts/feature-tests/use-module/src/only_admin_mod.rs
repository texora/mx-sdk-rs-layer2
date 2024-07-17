use dharitri_wasm_modules::only_admin;

dharitri_wasm::imports!();

#[dharitri_wasm::module]
#[only_admin]
pub trait OnlyAdminTestModule:
    super::only_admin_derived_mod::OnlyAdminDerivedTestModule + only_admin::OnlyAdminModule
{
    #[endpoint]
    fn only_admin_mod_endpoint(&self) {}
}
