use crate::{
    tx_execution::{
        builtin_function_mocks::builtin_func_trait::BuiltinFunctionDctTransferInfo,
        default_execution,
    },
    tx_mock::{
        BlockchainUpdate, TxCache, TxFunctionName, TxInput, TxLog, TxResult, TxTokenTransfer,
    },
};
use dharitri_wasm::{dharitri_codec::TopDecode, types::heap::Address};
use num_bigint::BigUint;
use num_traits::Zero;

pub(super) struct ParsedTransferBuiltinFunCall {
    pub destination: Address,
    pub raw_dct_transfers: Vec<RawDctTransfer>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
}

pub(super) struct RawDctTransfer {
    pub token_identifier: Vec<u8>,
    pub nonce_bytes: Vec<u8>,
    pub value_bytes: Vec<u8>,
}

pub(super) fn process_raw_dct_transfer(raw_dct_transfer: RawDctTransfer) -> TxTokenTransfer {
    TxTokenTransfer {
        token_identifier: raw_dct_transfer.token_identifier,
        nonce: u64::top_decode(raw_dct_transfer.nonce_bytes.as_slice()).unwrap(),
        value: BigUint::from_bytes_be(raw_dct_transfer.value_bytes.as_slice()),
    }
}

fn process_raw_dct_transfers(raw_dct_transfers: Vec<RawDctTransfer>) -> Vec<TxTokenTransfer> {
    raw_dct_transfers
        .into_iter()
        .map(process_raw_dct_transfer)
        .collect()
}

pub(super) fn extract_transfer_info(
    parsed_tx: ParsedTransferBuiltinFunCall,
) -> BuiltinFunctionDctTransferInfo {
    BuiltinFunctionDctTransferInfo {
        real_recipient: parsed_tx.destination,
        transfers: process_raw_dct_transfers(parsed_tx.raw_dct_transfers),
    }
}

pub(super) fn execute_transfer_builtin_func(
    parsed_tx: ParsedTransferBuiltinFunCall,
    builtin_function_name: &str,
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    let mut builtin_logs = Vec::new();
    for raw_dct_transfer in &parsed_tx.raw_dct_transfers {
        builtin_logs.push(TxLog {
            address: tx_input.from.clone(),
            endpoint: builtin_function_name.into(),
            topics: vec![
                raw_dct_transfer.token_identifier.clone(),
                raw_dct_transfer.nonce_bytes.clone(),
                raw_dct_transfer.value_bytes.clone(),
                parsed_tx.destination.to_vec(),
            ],
            data: vec![],
        });
    }

    let exec_input = TxInput {
        from: tx_input.from,
        to: parsed_tx.destination,
        moa_value: BigUint::zero(),
        dct_values: process_raw_dct_transfers(parsed_tx.raw_dct_transfers),
        func_name: parsed_tx.func_name,
        args: parsed_tx.args,
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
        ..Default::default()
    };

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends dct log
    tx_result.result_logs = [builtin_logs.as_slice(), tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}
