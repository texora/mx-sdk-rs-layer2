use crate::{display_util::*, num_bigint::BigUint};
use alloc::vec::Vec;
use dharitri_wasm::types::heap::{Address, H256};
use num_traits::Zero;
use std::fmt;

use super::TxFunctionName;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: Address,
    pub to: Address,
    pub moa_value: BigUint,
    pub dct_values: Vec<TxTokenTransfer>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
    pub promise_callback_closure_data: Vec<u8>,
    pub callback_payments: CallbackPayments,
}

impl Default for TxInput {
    fn default() -> Self {
        TxInput {
            from: Address::zero(),
            to: Address::zero(),
            moa_value: BigUint::zero(),
            dct_values: Vec::new(),
            func_name: TxFunctionName::EMPTY,
            args: Vec::new(),
            gas_limit: 0,
            gas_price: 0,
            tx_hash: H256::zero(),
            promise_callback_closure_data: Vec::new(),
            callback_payments: Default::default(),
        }
    }
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, dct_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            self.func_name.as_str(),
            self.args,
            self.moa_value,
            self.dct_values,
            address_hex(&self.from),
            address_hex(&self.to))
    }
}

impl TxInput {
    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }

    pub fn func_name_from_arg_index(&self, arg_index: usize) -> TxFunctionName {
        if let Some(arg) = self.args.get(arg_index) {
            arg.into()
        } else {
            TxFunctionName::EMPTY
        }
    }
}

/// Models DCT transfers between accounts.
#[derive(Clone, Debug)]
pub struct TxTokenTransfer {
    pub token_identifier: Vec<u8>,
    pub nonce: u64,
    pub value: BigUint,
}

/// Signals to the callback that funds have been returned to it, without performing any transfer.
#[derive(Default, Clone, Debug)]
pub struct CallbackPayments {
    pub moa_value: BigUint,
    pub dct_values: Vec<TxTokenTransfer>,
}

impl TxInput {
    /// The received MOA can come either from the original caller, or from an async call, during callback.
    pub fn received_moa(&self) -> &BigUint {
        if !self.callback_payments.moa_value.is_zero() {
            &self.callback_payments.moa_value
        } else {
            &self.moa_value
        }
    }

    /// The received DCT tokens can come either from the original caller, or from an async call, during callback.
    pub fn received_dct(&self) -> &[TxTokenTransfer] {
        if !self.callback_payments.dct_values.is_empty() {
            self.callback_payments.dct_values.as_slice()
        } else {
            self.dct_values.as_slice()
        }
    }
}
