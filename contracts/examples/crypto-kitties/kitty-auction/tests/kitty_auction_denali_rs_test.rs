use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract(
        "file:../kitty-ownership/output/kitty-ownership.wasm",
        kitty_ownership::ContractBuilder,
    );
    blockchain.register_contract(
        "file:output/kitty-auction.wasm",
        kitty_auction::ContractBuilder,
    );

    blockchain
}
#[test]
fn bid_first_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_first.scen.json", world());
}

#[test]
fn bid_second_max_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_second_max.scen.json", world());
}

#[test]
fn bid_second_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_second_ok.scen.json", world());
}

#[test]
fn bid_second_too_low_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_second_too_low.scen.json", world());
}

#[test]
fn bid_siring_auction_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_siring_auction.scen.json", world());
}

#[test]
fn create_and_auction_gen_zero_kitty_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/create_and_auction_gen_zero_kitty.scen.json",
        world(),
    );
}

#[test]
fn create_sale_auction_not_owner_rs() {
    dharitri_wasm_debug::denali_rs("denali/create_sale_auction_not_owner.scen.json", world());
}

#[test]
fn create_sale_auction_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/create_sale_auction_ok.scen.json", world());
}

#[test]
fn create_siring_auction_not_owner_rs() {
    dharitri_wasm_debug::denali_rs("denali/create_siring_auction_not_owner.scen.json", world());
}

#[test]
fn create_siring_auction_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/create_siring_auction_ok.scen.json", world());
}

#[test]
fn end_auction_no_bids_rs() {
    dharitri_wasm_debug::denali_rs("denali/end_auction_no_bids.scen.json", world());
}

#[test]
fn end_auction_second_bid_max_early_rs() {
    dharitri_wasm_debug::denali_rs("denali/end_auction_second_bid_max_early.scen.json", world());
}

#[test]
fn end_auction_second_bid_ok_early_rs() {
    dharitri_wasm_debug::denali_rs("denali/end_auction_second_bid_ok_early.scen.json", world());
}

#[test]
fn end_auction_second_bid_ok_late_rs() {
    dharitri_wasm_debug::denali_rs("denali/end_auction_second_bid_ok_late.scen.json", world());
}

#[test]
fn end_siring_auction_rs() {
    dharitri_wasm_debug::denali_rs("denali/end_siring_auction.scen.json", world());
}

#[test]
fn init_rs() {
    dharitri_wasm_debug::denali_rs("denali/init.scen.json", world());
}
