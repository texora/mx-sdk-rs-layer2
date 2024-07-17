use dharitri_wasm::types::Address;

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer};

pub trait BuiltinFunction {
    fn name(&self) -> &str;

    fn extract_dct_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo {
        BuiltinFunctionDctTransferInfo::empty(tx_input)
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate);
}

/// Contains a builtin function call DCT transfers (if any) and the real recipient of the transfer
/// (can be different from the "to" field.)
pub struct BuiltinFunctionDctTransferInfo {
    pub real_recipient: Address,
    pub transfers: Vec<TxTokenTransfer>,
}

impl BuiltinFunctionDctTransferInfo {
    pub fn empty(tx_input: &TxInput) -> Self {
        BuiltinFunctionDctTransferInfo {
            real_recipient: tx_input.to.clone(),
            transfers: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transfers.is_empty()
    }
}
