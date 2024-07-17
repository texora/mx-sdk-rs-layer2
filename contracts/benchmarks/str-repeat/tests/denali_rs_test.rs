use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract("file:output/str-repeat.wasm", str_repeat::ContractBuilder);
    blockchain
}

#[test]
fn test_str_repeat_denali_rs() {
    dharitri_wasm_debug::denali_rs("denali/str_repeat.scen.json", world());
}
