use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/bonding-curve-contract.wasm",
        bonding_curve_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn deploy_rs() {
    dharitri_wasm_debug::denali_rs("denali/deploy.scen.json", world());
}

#[test]
fn deposit_rs() {
    dharitri_wasm_debug::denali_rs("denali/deposit.scen.json", world());
}

#[test]
fn set_bonding_curve_rs() {
    dharitri_wasm_debug::denali_rs("denali/set_bonding_curve.scen.json", world());
}

#[test]
fn buy_rs() {
    dharitri_wasm_debug::denali_rs("denali/buy.scen.json", world());
}

#[test]
fn sell_rs() {
    dharitri_wasm_debug::denali_rs("denali/sell.scen.json", world());
}

#[test]
fn deposit_more_view_rs() {
    dharitri_wasm_debug::denali_rs("denali/deposit_more_view.scen.json", world());
}

#[test]
fn claim_rs() {
    dharitri_wasm_debug::denali_rs("denali/claim.scen.json", world());
}
