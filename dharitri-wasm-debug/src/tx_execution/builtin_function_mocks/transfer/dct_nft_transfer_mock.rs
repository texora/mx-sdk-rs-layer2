use crate::{
    tx_execution::builtin_function_mocks::builtin_func_trait::BuiltinFunctionDctTransferInfo,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};
use dharitri_wasm::{
    api::DCT_NFT_TRANSFER_FUNC_NAME, dharitri_codec::TopDecode, types::heap::Address,
};

use super::{
    super::builtin_func_trait::BuiltinFunction,
    transfer_common::{
        execute_transfer_builtin_func, extract_transfer_info, ParsedTransferBuiltinFunCall,
        RawDctTransfer,
    },
};

pub struct DCTNftTransfer;

impl BuiltinFunction for DCTNftTransfer {
    fn name(&self) -> &str {
        DCT_NFT_TRANSFER_FUNC_NAME
    }

    fn extract_dct_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionDctTransferInfo::empty(tx_input)
        }
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                execute_transfer_builtin_func(parsed_tx, self.name(), tx_input, tx_cache)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message.to_string());
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 4 {
        return Err("DCTNFTTransfer too few arguments");
    }
    if tx_input.to != tx_input.from {
        // TODO: not sure what the real error message would be, certainly not this
        return Err("DCTNFTTransfer expects that to == from");
    }

    let token_identifier = tx_input.args[0].clone();
    let nonce_bytes = tx_input.args[1].clone();
    let value_bytes = tx_input.args[2].clone();
    let destination = Address::top_decode(tx_input.args[3].as_slice()).unwrap();

    let func_name = tx_input.func_name_from_arg_index(4);
    let args = if tx_input.args.len() > 5 {
        tx_input.args[5..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination,
        raw_dct_transfers: vec![RawDctTransfer {
            token_identifier,
            nonce_bytes,
            value_bytes,
        }],
        func_name,
        args,
    })
}
