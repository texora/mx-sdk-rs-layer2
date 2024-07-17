use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/formatted-message-features");

    blockchain.register_contract(
        "file:output/formatted-message-features.wasm",
        formatted_message_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn managed_error_message_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_error_message.scen.json", world());
}

#[test]
fn sc_format_rs() {
    dharitri_wasm_debug::denali_rs("denali/sc_format.scen.json", world());
}
