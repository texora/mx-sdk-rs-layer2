use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/linked-list-repeat");

    blockchain.register_contract(
        "file:output/linked-list-repeat.wasm",
        linked_list_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn linked_list_repeat_struct_rs() {
    dharitri_wasm_debug::denali_rs("denali/linked_list_repeat_struct.scen.json", world());
}

#[test]
fn linked_list_repeat_rs() {
    dharitri_wasm_debug::denali_rs("denali/linked_list_repeat.scen.json", world());
}
