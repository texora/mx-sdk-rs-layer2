#[test]
fn deploy_erc20_and_crowdfunding_go() {
    dharitri_wasm_debug::denali_go("denali/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
fn fund_with_insufficient_allowance_go() {
    dharitri_wasm_debug::denali_go("denali/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_go() {
    dharitri_wasm_debug::denali_go("denali/fund_with_sufficient_allowance.scen.json");
}

#[test]
fn fund_without_allowance_go() {
    dharitri_wasm_debug::denali_go("denali/fund_without_allowance.scen.json");
}
