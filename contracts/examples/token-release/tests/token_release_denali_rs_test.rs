use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/token-release");

    blockchain.register_contract(
        "file:output/token-release.wasm",
        token_release::ContractBuilder,
    );
    blockchain
}

#[test]
fn token_release_add_group_rs() {
    dharitri_wasm_debug::denali_rs("denali/test-add-group.scen.json", world());
}

#[test]
fn token_release_add_user_rs() {
    dharitri_wasm_debug::denali_rs("denali/test-add-user.scen.json", world());
}

#[test]
fn token_release_change_user_rs() {
    dharitri_wasm_debug::denali_rs("denali/test-change-user.scen.json", world());
}

#[test]
fn token_release_claim_rs() {
    dharitri_wasm_debug::denali_rs("denali/test-claim.scen.json", world());
}

#[test]
fn token_release_end_setup_rs() {
    dharitri_wasm_debug::denali_rs("denali/test-end-setup.scen.json", world());
}

#[test]
fn token_release_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/test-init.scen.json", world());
}
