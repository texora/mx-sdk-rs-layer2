dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait ContractBaseFullPathTestModule: dharitri_wasm::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
