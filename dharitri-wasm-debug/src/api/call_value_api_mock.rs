use crate::{num_bigint, tx_mock::TxPanic, DebugApi};
use dharitri_wasm::{
    api::{CallValueApi, CallValueApiImpl},
    err_msg,
    types::DctTokenType,
};
use num_traits::Zero;

impl DebugApi {
    fn fail_if_more_than_one_dct_transfer(&self) {
        if self.dct_num_transfers() > 1 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::TOO_MANY_DCT_TRANSFERS.to_string(),
            });
        }
    }
}

impl CallValueApi for DebugApi {
    type CallValueApiImpl = DebugApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        DebugApi::new_from_static()
    }
}

impl CallValueApiImpl for DebugApi {
    fn check_not_payable(&self) {
        if self.input_ref().moa_value > num_bigint::BigUint::zero() {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_MOA.to_string(),
            });
        }
        if self.dct_num_transfers() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_DCT.to_string(),
            });
        }
    }

    #[inline]
    fn load_moa_value(&self, dest: Self::BigIntHandle) {
        self.set_big_uint(dest, self.input_ref().received_moa().clone())
    }

    #[inline]
    fn load_single_dct_value(&self, dest: Self::BigIntHandle) {
        self.fail_if_more_than_one_dct_transfer();
        if let Some(dct_value) = self.input_ref().received_dct().get(0) {
            self.set_big_uint(dest, dct_value.value.clone());
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn token(&self) -> Option<Self::ManagedBufferHandle> {
        self.fail_if_more_than_one_dct_transfer();

        if self.dct_num_transfers() > 0 {
            Some(self.token_by_index(0))
        } else {
            None
        }
    }

    #[inline]
    fn dct_token_nonce(&self) -> u64 {
        self.fail_if_more_than_one_dct_transfer();
        self.dct_token_nonce_by_index(0)
    }

    #[inline]
    fn dct_token_type(&self) -> DctTokenType {
        self.fail_if_more_than_one_dct_transfer();
        self.dct_token_type_by_index(0)
    }

    #[inline]
    fn dct_num_transfers(&self) -> usize {
        self.input_ref().received_dct().len()
    }

    #[inline]
    fn dct_value_by_index(&self, index: usize) -> Self::BigIntHandle {
        if let Some(dct_value) = self.input_ref().received_dct().get(index) {
            self.insert_new_big_uint(dct_value.value.clone())
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn token_by_index(&self, index: usize) -> Self::ManagedBufferHandle {
        if let Some(dct_value) = self.input_ref().received_dct().get(index) {
            self.insert_new_managed_buffer(dct_value.token_identifier.clone())
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn dct_token_nonce_by_index(&self, index: usize) -> u64 {
        if let Some(dct_value) = self.input_ref().received_dct().get(index) {
            dct_value.nonce
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn dct_token_type_by_index(&self, index: usize) -> DctTokenType {
        if self.dct_token_nonce_by_index(index) == 0 {
            DctTokenType::Fungible
        } else {
            DctTokenType::NonFungible
        }
    }
}
