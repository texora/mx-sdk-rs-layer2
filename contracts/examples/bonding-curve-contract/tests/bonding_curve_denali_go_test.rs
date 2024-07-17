#[test]
fn deploy_rs() {
    dharitri_wasm_debug::denali_go("denali/deploy.scen.json");
}

#[test]
fn deposit_rs() {
    dharitri_wasm_debug::denali_go("denali/deposit.scen.json");
}

#[test]
fn set_bonding_curve_rs() {
    dharitri_wasm_debug::denali_go("denali/set_bonding_curve.scen.json");
}

#[test]
fn buy_rs() {
    dharitri_wasm_debug::denali_go("denali/buy.scen.json");
}

#[test]
fn sell_rs() {
    dharitri_wasm_debug::denali_go("denali/sell.scen.json");
}

#[test]
fn deposit_more_view_rs() {
    dharitri_wasm_debug::denali_go("denali/deposit_more_view.scen.json");
}

#[test]
fn claim_rs() {
    dharitri_wasm_debug::denali_go("denali/claim.scen.json");
}
