#[test]
fn auction_batch_go() {
    dharitri_wasm_debug::denali_go("denali/auction_batch.scen.json");
}

#[test]
fn auction_single_token_moa_go() {
    dharitri_wasm_debug::denali_go("denali/auction_single_token_moa.scen.json");
}

#[test]
fn bid_first_moa_go() {
    dharitri_wasm_debug::denali_go("denali/bid_first_moa.scen.json");
}

#[test]
fn bid_second_moa_go() {
    dharitri_wasm_debug::denali_go("denali/bid_second_moa.scen.json");
}

#[test]
fn bid_third_moa_go() {
    dharitri_wasm_debug::denali_go("denali/bid_third_moa.scen.json");
}

#[test]
fn end_auction_go() {
    dharitri_wasm_debug::denali_go("denali/end_auction.scen.json");
}
