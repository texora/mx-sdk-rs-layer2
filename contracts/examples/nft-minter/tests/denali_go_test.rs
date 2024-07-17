#[test]
fn init_go() {
    dharitri_wasm_debug::denali_go("denali/init.scen.json");
}

#[test]
fn create_nft_go() {
    dharitri_wasm_debug::denali_go("denali/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    dharitri_wasm_debug::denali_go("denali/buy_nft.scen.json");
}
