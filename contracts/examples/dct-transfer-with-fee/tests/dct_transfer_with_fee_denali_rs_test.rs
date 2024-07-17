use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/dct-transfer-with-fee");

    blockchain.register_contract(
        "file:output/dct-transfer-with-fee.wasm",
        dct_transfer_with_fee::ContractBuilder,
    );
    blockchain
}

#[test]
fn deploy_rs() {
    dharitri_wasm_debug::denali_rs("denali/deploy.scen.json", world());
}

#[test]
fn setup_fees_rs() {
    dharitri_wasm_debug::denali_rs("denali/setup_fees_and_transfer.scen.json", world());
}

#[test]
fn claim_rs() {
    dharitri_wasm_debug::denali_rs("denali/claim.scen.json", world());
}
