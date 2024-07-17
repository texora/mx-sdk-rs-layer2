#[test]
fn deploy_go() {
    dharitri_wasm_debug::denali_go("denali/deploy.scen.json");
}

#[test]
fn setup_fees_go() {
    dharitri_wasm_debug::denali_go("denali/setup_fees_and_transfer.scen.json");
}

#[test]
fn claim_go() {
    dharitri_wasm_debug::denali_go("denali/claim.scen.json");
}
