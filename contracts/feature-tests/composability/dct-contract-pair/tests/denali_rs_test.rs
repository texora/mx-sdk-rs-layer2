use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:first-contract/output/first-contract.wasm",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract(
        "file:second-contract/output/second-contract.wasm",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    dharitri_wasm_debug::denali_rs("denali/init.scen.json", world());
}

#[test]
fn simple_transfer_full_rs() {
    dharitri_wasm_debug::denali_rs("denali/simple_transfer_full.scen.json", world());
}

#[test]
fn simple_transfer_half_rs() {
    dharitri_wasm_debug::denali_rs("denali/simple_transfer_half.scen.json", world());
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/simple_transfer_full_wrong_token.scen.json", world());
}

// TODO: implement DCTTransfer + async call
// #[test]
// fn rejected_transfer_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/reject_transfer.scen.json", world());
// }
