use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/dct-system-sc-mock");
    blockchain.register_contract(
        "file:output/dct-system-sc-mock.wasm",
        dct_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    dharitri_wasm_debug::denali_rs("denali/dct_system_sc.scen.json", world());
}
