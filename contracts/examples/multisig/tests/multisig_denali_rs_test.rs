use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig.wasm",
        multisig::ContractBuilder,
        "multisig",
    );
    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig-view.wasm",
        multisig::ContractBuilder,
        "multisig-view",
    );

    blockchain.register_contract("file:test-contracts/adder.wasm", adder::ContractBuilder);

    blockchain.register_contract(
        "file:test-contracts/factorial.wasm",
        factorial::ContractBuilder,
    );

    blockchain
}

#[ignore]
#[test]
fn call_other_shard_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/call_other_shard-1.scen.json", world());
}

#[ignore]
#[test]
fn call_other_shard_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/call_other_shard-2.scen.json", world());
}

#[test]
fn changeboard_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeBoard.scen.json", world());
}

#[test]
fn changequorum_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeQuorum.scen.json", world());
}

#[test]
fn changequorum_toobig_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeQuorum_tooBig.scen.json", world());
}

#[test]
fn deployadder_err_rs() {
    dharitri_wasm_debug::denali_rs("denali/deployAdder_err.scen.json", world());
}

#[test]
fn deployadder_then_call_rs() {
    dharitri_wasm_debug::denali_rs("denali/deployAdder_then_call.scen.json", world());
}

#[test]
fn deployfactorial_rs() {
    dharitri_wasm_debug::denali_rs("denali/deployFactorial.scen.json", world());
}

#[test]
fn deployothermultisig_rs() {
    dharitri_wasm_debug::denali_rs("denali/deployOtherMultisig.scen.json", world());
}

#[test]
fn deploy_duplicate_bm_rs() {
    dharitri_wasm_debug::denali_rs("denali/deploy_duplicate_bm.scen.json", world());
}

#[test]
fn remove_everyone_rs() {
    dharitri_wasm_debug::denali_rs("denali/remove_everyone.scen.json", world());
}

#[test]
fn senddct_rs() {
    dharitri_wasm_debug::denali_rs("denali/sendDct.scen.json", world());
}

#[test]
fn upgrade_rs() {
    dharitri_wasm_debug::denali_rs("denali/upgrade.scen.json", world());
}

#[test]
fn upgrade_from_source_rs() {
    dharitri_wasm_debug::denali_rs("denali/upgrade_from_source.scen.json", world());
}
