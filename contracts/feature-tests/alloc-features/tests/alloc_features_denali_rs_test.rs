use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/alloc-features");

    blockchain.register_contract(
        "file:output/alloc-features.wasm",
        alloc_features::ContractBuilder,
    );

    blockchain
}
#[test]
fn boxed_bytes_zeros_rs() {
    dharitri_wasm_debug::denali_rs("denali/boxed_bytes_zeros.scen.json", world());
}

// #[test]
// fn crypto_elliptic_curves_legacy_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_elliptic_curves_legacy.scen.json", world());
// }

#[test]
fn crypto_keccak256_legacy_alloc_rs() {
    dharitri_wasm_debug::denali_rs("denali/crypto_keccak256_legacy_alloc.scen.json", world());
}

// #[test]
// fn crypto_ripemd160_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_ripemd160.scen.json", world());
// }

#[test]
fn crypto_sha256_legacy_alloc_rs() {
    dharitri_wasm_debug::denali_rs("denali/crypto_sha256_legacy_alloc.scen.json", world());
}

// #[test]
// fn crypto_verify_bls_legacy_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_verify_bls_legacy.scen.json", world());
// }

#[test]
fn crypto_verify_ed25519_legacy_rs() {
    dharitri_wasm_debug::denali_rs("denali/crypto_verify_ed25519_legacy.scen.json", world());
}

// #[test]
// fn crypto_verify_secp256k1_legacy_rs() {
//     dharitri_wasm_debug::denali_rs("denali/crypto_verify_secp256k1_legacy.scen.json", world());
// }

#[test]
fn echo_async_result_empty_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_async_result_empty.scen.json", world());
}

#[test]
fn echo_big_int_nested_alloc_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_big_int_nested_alloc.scen.json", world());
}

#[test]
fn echo_boxed_bytes_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_boxed_bytes.scen.json", world());
}

#[test]
fn echo_multi_value_tuples_alloc_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_multi_value_tuples_alloc.scen.json", world());
}

#[test]
fn echo_ser_ex_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_ser_ex_1.scen.json", world());
}

#[test]
fn echo_slice_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_slice_u8.scen.json", world());
}

#[test]
fn echo_str_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_str.scen.json", world());
}

#[test]
fn echo_str_box_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_str_box.scen.json", world());
}

#[test]
fn echo_string_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_string.scen.json", world());
}

#[test]
fn echo_varargs_u32_alloc_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_varargs_u32_alloc.scen.json", world());
}

#[test]
fn echo_vec_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/echo_vec_u8.scen.json", world());
}

#[test]
fn events_legacy_rs() {
    dharitri_wasm_debug::denali_rs("denali/events_legacy.scen.json", world());
}

#[test]
fn managed_buffer_concat_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_concat_2.scen.json", world());
}

#[test]
fn managed_buffer_load_slice_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_load_slice.scen.json", world());
}

#[test]
fn managed_buffer_overwrite_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_overwrite.scen.json", world());
}

#[test]
fn managed_buffer_set_slice_rs() {
    dharitri_wasm_debug::denali_rs("denali/managed_buffer_set_slice.scen.json", world());
}

#[test]
fn only_owner_legacy_rs() {
    dharitri_wasm_debug::denali_rs("denali/only_owner_legacy.scen.json", world());
}

#[test]
fn sc_result_rs() {
    dharitri_wasm_debug::denali_rs("denali/sc_result.scen.json", world());
}

#[test]
fn storage_address_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_address.scen.json", world());
}

#[test]
fn storage_opt_address_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_opt_address.scen.json", world());
}

#[test]
fn storage_vec_u8_rs() {
    dharitri_wasm_debug::denali_rs("denali/storage_vec_u8.scen.json", world());
}
