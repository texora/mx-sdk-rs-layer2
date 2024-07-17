use crate::{
    api::{CallValueApi, CallValueApiImpl},
    types::DctTokenType,
};

use super::UncallableApi;

impl CallValueApi for UncallableApi {
    type CallValueApiImpl = UncallableApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        unreachable!()
    }
}

impl CallValueApiImpl for UncallableApi {
    fn check_not_payable(&self) {
        unreachable!()
    }

    fn load_moa_value(&self, _dest: Self::BigIntHandle) {
        unreachable!()
    }

    fn load_single_dct_value(&self, _dest: Self::BigIntHandle) {
        unreachable!()
    }

    fn token(&self) -> Option<Self::ManagedBufferHandle> {
        unreachable!()
    }

    fn dct_token_nonce(&self) -> u64 {
        unreachable!()
    }

    fn dct_token_type(&self) -> DctTokenType {
        unreachable!()
    }

    fn dct_num_transfers(&self) -> usize {
        unreachable!()
    }

    fn dct_value_by_index(&self, _index: usize) -> Self::BigIntHandle {
        unreachable!()
    }

    fn token_by_index(&self, _index: usize) -> Self::ManagedBufferHandle {
        unreachable!()
    }

    fn dct_token_nonce_by_index(&self, _index: usize) -> u64 {
        unreachable!()
    }

    fn dct_token_type_by_index(&self, _index: usize) -> DctTokenType {
        unreachable!()
    }
}
