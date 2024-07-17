#[macro_export]
macro_rules! rust_biguint {
    ($value:expr) => {{
        dharitri_wasm_debug::num_bigint::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_biguint {
    ($value:expr) => {{
        dharitri_wasm::types::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_buffer {
    ($value:expr) => {{
        dharitri_wasm::types::ManagedBuffer::new_from_bytes($value)
    }};
}

#[macro_export]
macro_rules! managed_address {
    ($address:expr) => {{
        dharitri_wasm::types::ManagedAddress::from_address($address)
    }};
}

#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        dharitri_wasm::types::TokenIdentifier::from_dct_bytes($bytes)
    }};
}

#[macro_export]
macro_rules! managed_token_id_wrapped {
    ($bytes:expr) => {{
        let ___dct_token_id___ = dharitri_wasm::types::TokenIdentifier::from_dct_bytes($bytes);
        dharitri_wasm::types::MoaOrDctTokenIdentifier::dct(___dct_token_id___)
    }};
}

#[macro_export]
macro_rules! managed_moa_token_id {
    () => {{
        dharitri_wasm::types::MoaOrDctTokenIdentifier::moa()
    }};
}

#[macro_export]
macro_rules! assert_sc_error {
    ($sc_result:expr, $expected_string:expr) => {{
        match $sc_result {
            dharitri_wasm::types::SCResult::Ok(t) => {
                panic!("Expected SCError, but got SCResult::Ok: {:?}", t)
            },
            dharitri_wasm::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                assert_eq!(as_str, $expected_string);
            },
        }
    }};
}

#[macro_export]
macro_rules! assert_values_eq {
    ($left:expr, $right:expr) => {{
        assert!(
            $left == $right,
            "Assert mismatch: \n Left: {:?} \n Right: {:?}",
            $left,
            $right
        )
    }};
}

#[macro_export]
macro_rules! unwrap_or_panic {
    ($sc_result:expr) => {{
        match $sc_result {
            dharitri_wasm::types::SCResult::Ok(t) => t,
            dharitri_wasm::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                panic!("{}", as_str);
            },
        }
    }};
}
