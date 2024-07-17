#[test]
fn claim_moa_go() {
    dharitri_wasm_debug::denali_go("denali/claim-moa.scen.json");
}

#[test]
fn claim_dct_go() {
    dharitri_wasm_debug::denali_go("denali/claim-dct.scen.json");
}

#[test]
fn fund_moa_and_dct_go() {
    dharitri_wasm_debug::denali_go("denali/fund-moa-and-dct.scen.json");
}

#[test]
fn set_accounts_go() {
    dharitri_wasm_debug::denali_go("denali/set-accounts.scen.json");
}

#[test]
fn withdraw_moa_go() {
    dharitri_wasm_debug::denali_go("denali/withdraw-moa.scen.json");
}

#[test]
fn withdraw_dct_go() {
    dharitri_wasm_debug::denali_go("denali/withdraw-dct.scen.json");
}
