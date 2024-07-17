use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-bubbles");

    blockchain.register_contract(
        "file:output/crypto-bubbles.wasm",
        crypto_bubbles::ContractBuilder,
    );
    blockchain
}

#[test]
fn balanceof_rs() {
    dharitri_wasm_debug::denali_rs("denali/balanceOf.scen.json", world());
}

#[test]
fn create_rs() {
    dharitri_wasm_debug::denali_rs("denali/create.scen.json", world());
}

#[test]
fn exceptions_rs() {
    dharitri_wasm_debug::denali_rs("denali/exceptions.scen.json", world());
}

#[test]
fn joingame_rs() {
    dharitri_wasm_debug::denali_rs("denali/joinGame.scen.json", world());
}

#[test]
fn rewardandsendtowallet_rs() {
    dharitri_wasm_debug::denali_rs("denali/rewardAndSendToWallet.scen.json", world());
}

#[test]
fn rewardwinner_rs() {
    dharitri_wasm_debug::denali_rs("denali/rewardWinner.scen.json", world());
}

#[test]
fn rewardwinner_last_rs() {
    dharitri_wasm_debug::denali_rs("denali/rewardWinner_Last.scen.json", world());
}

#[test]
fn topup_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/topUp_ok.scen.json", world());
}

#[test]
fn topup_withdraw_rs() {
    dharitri_wasm_debug::denali_rs("denali/topUp_withdraw.scen.json", world());
}

#[test]
fn withdraw_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/withdraw_Ok.scen.json", world());
}

#[test]
fn withdraw_toomuch_rs() {
    dharitri_wasm_debug::denali_rs("denali/withdraw_TooMuch.scen.json", world());
}
