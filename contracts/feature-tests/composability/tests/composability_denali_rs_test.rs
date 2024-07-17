use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");

    blockchain.register_contract(
        "file:forwarder/output/forwarder.wasm",
        forwarder::ContractBuilder,
    );
    blockchain.register_contract(
        "file:forwarder-raw/output/forwarder-raw.wasm",
        forwarder_raw::ContractBuilder,
    );
    blockchain.register_contract(
        "file:promises-features/output/promises-features.wasm",
        promises_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:proxy-test-first/output/proxy-test-first.wasm",
        proxy_test_first::ContractBuilder,
    );
    blockchain.register_contract(
        "file:proxy-test-second/output/proxy-test-second.wasm",
        proxy_test_second::ContractBuilder,
    );
    blockchain.register_contract(
        "file:recursive-caller/output/recursive-caller.wasm",
        recursive_caller::ContractBuilder,
    );
    blockchain.register_contract("file:vault/output/vault.wasm", vault::ContractBuilder);
    blockchain
}

#[test]
fn forw_raw_async_accept_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_async_accept_moa.scen.json", world());
}

#[test]
fn forw_raw_async_accept_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_async_accept_dct.scen.json", world());
}

#[test]
fn forw_raw_async_echo_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_async_echo.scen.json", world());
}

#[test]
fn forw_raw_async_send_and_retrieve_multi_transfer_funds_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forw_raw_async_send_and_retrieve_multi_transfer_funds.scen.json",
        world(),
    );
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_async_call_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forw_raw_builtin_nft_local_mint_via_async_call.scen.json",
        world(),
    );
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_sync_call_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forw_raw_builtin_nft_local_mint_via_sync_call.scen.json",
        world(),
    );
}

#[test]
fn forw_raw_call_async_retrieve_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forw_raw_call_async_retrieve_multi_transfer.scen.json",
        world(),
    );
}

#[test]
fn forw_raw_contract_deploy_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_contract_deploy.scen.json", world());
}

#[test]
fn forw_raw_contract_upgrade_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_contract_upgrade.scen.json", world());
}

#[test]
fn forw_raw_contract_upgrade_self_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_contract_upgrade_self.scen.json", world());
}

#[test]
fn forw_raw_direct_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_direct_moa.scen.json", world());
}

#[test]
fn forw_raw_direct_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_direct_dct.scen.json", world());
}

#[test]
fn forw_raw_direct_multi_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_direct_multi_dct.scen.json", world());
}

#[test]
fn forw_raw_sync_echo_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_echo.scen.json", world());
}

#[test]
fn forw_raw_sync_echo_caller_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_echo_caller.scen.json", world());
}

#[test]
fn forw_raw_sync_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_moa.scen.json", world());
}

// #[test]
// fn forw_raw_sync_readonly_rs() {
//     dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_readonly.scen.json", world());
// }

// #[test]
// fn forw_raw_sync_same_context_rs() {
//     dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_same_context.scen.json", world());
// }

// #[test]
// fn forw_raw_sync_same_context_moa_rs() {
//     dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_same_context_moa.scen.json", world());
// }

#[test]
fn forw_raw_transf_exec_accept_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_transf_exec_accept_moa.scen.json", world());
}

#[test]
fn forw_raw_transf_exec_reject_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_transf_exec_reject_moa.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_add_quantity_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_builtin_nft_add_quantity.scen.json",
        world(),
    );
}

#[test]
fn forwarder_builtin_nft_burn_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_builtin_nft_burn.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_create_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_builtin_nft_create.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_local_burn_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_builtin_nft_local_burn.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_local_mint_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_builtin_nft_local_mint.scen.json", world());
}

#[test]
fn forwarder_call_async_accept_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_async_accept_moa.scen.json", world());
}

#[test]
fn forwarder_call_async_accept_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_async_accept_dct.scen.json", world());
}

#[test]
fn forwarder_call_async_accept_nft_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_async_accept_nft.scen.json", world());
}

#[test]
fn forwarder_call_async_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_async_multi_transfer.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_async_retrieve_moa_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_async_retrieve_moa.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_async_retrieve_dct_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_async_retrieve_dct.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_async_retrieve_nft_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_async_retrieve_nft.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_moa.scen.json", world());
}

#[test]
fn forwarder_call_sync_accept_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_dct.scen.json", world());
}

#[test]
fn forwarder_call_sync_accept_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_sync_accept_multi_transfer.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_nft_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_nft.scen.json", world());
}

#[test]
fn forwarder_call_sync_accept_then_read_moa_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_sync_accept_then_read_moa.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_then_read_dct_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_sync_accept_then_read_dct.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_then_read_nft_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_sync_accept_then_read_nft.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_retrieve_moa_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_sync_retrieve_moa.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_retrieve_dct_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_sync_retrieve_dct.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_retrieve_nft_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_call_sync_retrieve_nft.scen.json", world());
}

#[test]
fn forwarder_call_transf_exec_accept_moa_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_moa.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_moa_twice_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_moa_twice.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_dct_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_dct.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_dct_twice_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_dct_twice.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_multi_transfer.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_nft_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_nft.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_return_values_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_return_values.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_sft_twice_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_accept_sft_twice.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_reject_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_reject_multi_transfer.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_reject_nft_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_transf_exec_reject_nft.scen.json",
        world(),
    );
}

#[test]
fn forwarder_contract_change_owner_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_contract_change_owner.scen.json", world());
}

#[test]
fn forwarder_contract_deploy_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_contract_deploy.scen.json", world());
}

#[test]
fn forwarder_contract_upgrade_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_contract_upgrade.scen.json", world());
}

#[test]
fn forwarder_get_dct_local_roles_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_get_dct_local_roles.scen.json", world());
}

#[test]
fn forwarder_get_dct_token_data_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_get_dct_token_data.scen.json", world());
}

#[test]
fn forwarder_nft_add_uri_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_add_uri.scen.json", world());
}

#[test]
fn forwarder_nft_create_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_create.scen.json", world());
}

#[test]
fn forwarder_nft_create_and_send_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_create_and_send.scen.json", world());
}

#[test]
fn forwarder_nft_current_nonce_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_current_nonce.scen.json", world());
}

#[test]
fn forwarder_nft_decode_complex_attributes_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_nft_decode_complex_attributes.scen.json",
        world(),
    );
}

#[test]
fn forwarder_nft_transfer_async_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_transfer_async.scen.json", world());
}

#[test]
fn forwarder_nft_transfer_exec_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_transfer_exec.scen.json", world());
}

#[test]
fn forwarder_nft_update_attributes_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_nft_update_attributes.scen.json", world());
}

#[test]
fn forwarder_no_endpoint_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_no_endpoint.scen.json", world());
}

#[test]
fn forwarder_retrieve_funds_with_accept_func_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_retrieve_funds_with_accept_func.scen.json",
        world(),
    );
}

#[test]
fn forwarder_send_dct_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_send_dct_multi_transfer.scen.json",
        world(),
    );
}

// #[test]
// fn forwarder_send_twice_moa_rs() {
//     dharitri_wasm_debug::denali_rs("denali/forwarder_send_twice_moa.scen.json", world());
// }

// #[test]
// fn forwarder_send_twice_dct_rs() {
//     dharitri_wasm_debug::denali_rs("denali/forwarder_send_twice_dct.scen.json", world());
// }

#[test]
fn forwarder_sync_echo_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_sync_echo.scen.json", world());
}

#[test]
fn forwarder_tranfer_dct_with_fees_rs() {
    dharitri_wasm_debug::denali_rs("denali/forwarder_tranfer_dct_with_fees.scen.json", world());
}

#[test]
fn forwarder_validate_token_identifier_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_validate_token_identifier.scen.json",
        world(),
    );
}

#[test]
fn promises_multi_transfer_rs() {
    dharitri_wasm_debug::denali_rs("denali-promises/promises_multi_transfer.scen.json", world());
}

// #[test]
// fn promises_single_transfer_rs() {
//     dharitri_wasm_debug::denali_rs("denali-promises/promises_single_transfer.scen.json", world());
// }

#[test]
fn proxy_test_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/proxy_test_init.scen.json", world());
}

#[test]
fn proxy_test_message_othershard_rs() {
    dharitri_wasm_debug::denali_rs("denali/proxy_test_message_otherShard.scen.json", world());
}

#[test]
fn proxy_test_message_othershard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_message_otherShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_message_sameshard_rs() {
    dharitri_wasm_debug::denali_rs("denali/proxy_test_message_sameShard.scen.json", world());
}

#[test]
fn proxy_test_message_sameshard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_message_sameShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_payment_othershard_rs() {
    dharitri_wasm_debug::denali_rs("denali/proxy_test_payment_otherShard.scen.json", world());
}

#[test]
fn proxy_test_payment_othershard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_payment_otherShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_payment_sameshard_rs() {
    dharitri_wasm_debug::denali_rs("denali/proxy_test_payment_sameShard.scen.json", world());
}

#[test]
fn proxy_test_payment_sameshard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_payment_sameShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_upgrade_rs() {
    dharitri_wasm_debug::denali_rs("denali/proxy_test_upgrade.scen.json", world());
}

#[test]
fn recursive_caller_moa_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/recursive_caller_moa_1.scen.json", world());
}

#[test]
fn recursive_caller_dct_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/recursive_caller_dct_1.scen.json", world());
}

#[test]
fn send_moa_rs() {
    dharitri_wasm_debug::denali_rs("denali/send_moa.scen.json", world());
}

#[test]
fn send_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/send_dct.scen.json", world());
}
