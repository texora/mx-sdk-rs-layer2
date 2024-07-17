use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/single-value-repeat");

    blockchain.register_contract(
        "file:output/single-value-repeat.wasm",
        single_value_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn single_value_repeat_struct_rs() {
    dharitri_wasm_debug::denali_rs("denali/single_value_repeat_struct.scen.json", world());
}

#[test]
fn single_value_repeat_rs() {
    dharitri_wasm_debug::denali_rs("denali/single_value_repeat.scen.json", world());
}
