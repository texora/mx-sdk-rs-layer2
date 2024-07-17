#[test]
fn init_go() {
    dharitri_wasm_debug::denali_go("denali/init.scen.json");
}

// #[test]
// fn reject_transfer_go() {
// 	dharitri_wasm_debug::denali_go("denali/reject_transfer.scen.json");
// }

#[test]
fn simple_transfer_full_go() {
    dharitri_wasm_debug::denali_go("denali/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_go() {
    dharitri_wasm_debug::denali_go("denali/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_go() {
    dharitri_wasm_debug::denali_go("denali/simple_transfer_half.scen.json");
}
