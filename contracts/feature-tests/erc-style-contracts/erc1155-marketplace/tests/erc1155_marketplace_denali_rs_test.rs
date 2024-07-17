use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/erc1155-marketplace.wasm",
        erc1155_marketplace::ContractBuilder,
    );
    blockchain.register_contract(
        "file:../erc1155/output/erc1155.wasm",
        erc1155::ContractBuilder,
    );

    blockchain
}

#[test]
fn auction_single_token_moa_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/auction_single_token_moa.scen.json", world());
}

#[test]
fn auction_batch_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/auction_batch.scen.json", world());
}

#[test]
fn bid_first_moa_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_first_moa.scen.json", world());
}

#[test]
fn bid_second_moa_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_second_moa.scen.json", world());
}

#[test]
fn bid_third_moa_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_third_moa.scen.json", world());
}

#[test]
fn end_auction_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/end_auction.scen.json", world());
}
