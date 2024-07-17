#[test]
fn token_release_add_group_go() {
    dharitri_wasm_debug::denali_go("denali/test-add-group.scen.json");
}

#[test]
fn token_release_add_user_go() {
    dharitri_wasm_debug::denali_go("denali/test-add-user.scen.json");
}

#[test]
fn token_release_change_user_go() {
    dharitri_wasm_debug::denali_go("denali/test-change-user.scen.json");
}

#[test]
fn token_release_claim_go() {
    dharitri_wasm_debug::denali_go("denali/test-claim.scen.json");
}

#[test]
fn token_release_end_setup_go() {
    dharitri_wasm_debug::denali_go("denali/test-end-setup.scen.json");
}

#[test]
fn token_release_init_go() {
    dharitri_wasm_debug::denali_go("denali/test-init.scen.json");
}
