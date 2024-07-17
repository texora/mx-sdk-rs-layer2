use super::VmApiImpl;
use dharitri_wasm::{
    api::{CallValueApi, CallValueApiImpl, StaticVarApiImpl},
    types::{DctTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetMultiDCTCallValue(resultHandle: i32);

    fn getNumDCTTransfers() -> i32;

    // single DCT transfer
    fn bigIntGetDCTCallValue(dest: i32);
    fn getDCTTokenName(resultOffset: *const u8) -> i32;
    fn getDCTTokenNonce() -> i64;
    fn getDCTTokenType() -> i32;

    // DCT by index
    fn bigIntGetDCTCallValueByIndex(dest: i32, index: i32);
    fn getDCTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getDCTTokenNonceByIndex(index: i32) -> i64;
    fn getDCTTokenTypeByIndex(index: i32) -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn load_moa_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn load_all_dct_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiDCTCallValue(dest_handle);
        }
    }

    fn dct_num_transfers(&self) -> usize {
        unsafe { getNumDCTTransfers() as usize }
    }

    fn load_single_dct_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetDCTCallValue(dest);
        }
    }

    fn token(&self) -> Option<Self::ManagedBufferHandle> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                None
            } else {
                Some(
                    TokenIdentifier::<Self>::from_dct_bytes(&name_buffer[..name_len as usize])
                        .get_raw_handle(),
                )
            }
        }
    }

    fn dct_token_nonce(&self) -> u64 {
        unsafe { getDCTTokenNonce() as u64 }
    }

    fn dct_token_type(&self) -> DctTokenType {
        unsafe { (getDCTTokenType() as u8).into() }
    }

    fn dct_value_by_index(&self, index: usize) -> Self::BigIntHandle {
        unsafe {
            let value_handle = self.next_handle();
            bigIntGetDCTCallValueByIndex(value_handle, index as i32);
            value_handle
        }
    }

    fn token_by_index(&self, index: usize) -> Self::ManagedBufferHandle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);

            TokenIdentifier::<Self>::from_dct_bytes(&name_buffer[..name_len as usize])
                .get_raw_handle()
        }
    }

    fn dct_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getDCTTokenNonceByIndex(index as i32) as u64 }
    }

    fn dct_token_type_by_index(&self, index: usize) -> DctTokenType {
        unsafe { (getDCTTokenTypeByIndex(index as i32) as u8).into() }
    }
}
