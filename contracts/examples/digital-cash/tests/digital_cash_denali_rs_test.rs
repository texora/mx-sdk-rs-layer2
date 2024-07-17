use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

// verify_ed25519 not implemented
// #[test]
// fn claim_moa_rs() {
//     dharitri_wasm_debug::denali_rs("denali/claim-moa.scen.json", world());
// }

// verify_ed25519 not implemented
// #[test]
// fn claim_dct_rs() {
//     dharitri_wasm_debug::denali_rs("denali/claim-dct.scen.json", world());
// }

#[test]
fn fund_moa_and_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/fund-moa-and-dct.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    dharitri_wasm_debug::denali_rs("denali/set-accounts.scen.json", world());
}

#[test]
fn withdraw_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/withdraw-moa.scen.json", world());
}

#[test]
fn withdraw_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/withdraw-dct.scen.json", world());
}
