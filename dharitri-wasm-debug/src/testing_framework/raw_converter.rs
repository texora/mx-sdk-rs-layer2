use std::collections::BTreeMap;

use crate::{
    num_bigint,
    world_mock::{AccountData, BlockInfo, DctData},
};
use dharitri_wasm::types::heap::Address;
use denali::serde_raw::{
    AccountRaw, BlockInfoRaw, CheckAccountRaw, CheckAccountsRaw, CheckBytesValueRaw,
    CheckDctDataRaw, CheckDctInstanceRaw, CheckDctInstancesRaw, CheckDctMapContentsRaw,
    CheckDctMapRaw, CheckDctRaw, CheckLogsRaw, CheckStorageDetailsRaw, CheckStorageRaw,
    CheckValueListRaw, DctFullRaw, DctInstanceRaw, DctRaw, TxCallRaw, TxDCTRaw, TxExpectRaw,
    TxQueryRaw, ValueSubTree,
};
use num_traits::Zero;

use super::{ScCallDenali, ScQueryDenali, TxExpectDenali};

pub(crate) const STAR_STR: &str = "*";

pub(crate) fn account_as_raw(acc: &AccountData) -> AccountRaw {
    let balance_raw = Some(rust_biguint_as_raw(&acc.moa_balance));
    let developer_rewards_raw = Some(rust_biguint_as_raw(&acc.developer_rewards));
    let code_raw = acc
        .contract_path
        .clone()
        .map(|c| ValueSubTree::Str(String::from_utf8(c).unwrap()));

    let mut all_dct_raw = BTreeMap::new();
    for (token_id, dct_data) in acc.dct.iter() {
        let token_id_raw = bytes_to_denali_string_or_hex(token_id);
        let dct_raw = dct_data_as_raw(dct_data);

        let _ = all_dct_raw.insert(token_id_raw, dct_raw);
    }

    let mut storage_raw = BTreeMap::new();
    for (key, value) in acc.storage.iter() {
        let key_raw = bytes_to_denali_string_or_hex(key);
        let value_raw = bytes_as_raw(value);

        let _ = storage_raw.insert(key_raw, value_raw);
    }

    AccountRaw {
        balance: balance_raw,
        code: code_raw,
        comment: None,
        dct: all_dct_raw,
        nonce: Some(u64_as_raw(acc.nonce)),
        owner: acc.contract_owner.as_ref().map(address_as_raw),
        storage: storage_raw,
        username: None, // TODO: Add if needed
        developer_rewards: developer_rewards_raw,
    }
}

pub(crate) fn dct_data_as_raw(dct: &DctData) -> DctRaw {
    let last_nonce_raw = if dct.last_nonce == 0 {
        None
    } else {
        Some(u64_as_raw(dct.last_nonce))
    };

    let roles = dct.get_roles();
    let mut roles_raw = Vec::with_capacity(roles.len());
    for role in roles {
        roles_raw.push(String::from_utf8(role).unwrap());
    }

    let mut instances_raw = Vec::new();
    for inst in dct.instances.get_instances().values() {
        let inst_raw = DctInstanceRaw {
            attributes: Some(bytes_as_raw(&inst.metadata.attributes)),
            balance: Some(rust_biguint_as_raw(&inst.balance)),
            creator: inst.metadata.creator.as_ref().map(address_as_raw),
            hash: inst.metadata.hash.as_ref().map(|h| bytes_as_raw(h)),
            nonce: Some(u64_as_raw(inst.nonce)),
            royalties: Some(u64_as_raw(inst.metadata.royalties)),
            uri: inst.metadata.uri.iter().map(|u| bytes_as_raw(u)).collect(),
        };

        instances_raw.push(inst_raw);
    }

    DctRaw::Full(DctFullRaw {
        frozen: None,
        instances: instances_raw,
        last_nonce: last_nonce_raw,
        roles: roles_raw,
        token_identifier: None,
    })
}

pub(crate) fn block_info_as_raw(block_info: &BlockInfo) -> BlockInfoRaw {
    BlockInfoRaw {
        block_epoch: Some(u64_as_raw(block_info.block_epoch)),
        block_nonce: Some(u64_as_raw(block_info.block_nonce)),
        block_round: Some(u64_as_raw(block_info.block_round)),
        block_timestamp: Some(u64_as_raw(block_info.block_timestamp)),
        block_random_seed: Some(bytes_as_raw(&block_info.block_random_seed[..])),
    }
}

pub(crate) fn tx_call_as_raw(tx_call: &ScCallDenali) -> TxCallRaw {
    let mut all_dct_raw = Vec::with_capacity(tx_call.dct.len());
    for dct in tx_call.dct.iter() {
        let dct_raw = TxDCTRaw {
            token_identifier: Some(ValueSubTree::Str(bytes_to_denali_string_or_hex(
                &dct.token_identifier,
            ))),
            nonce: Some(u64_as_raw(dct.nonce)),
            value: rust_biguint_as_raw(&dct.value),
        };

        all_dct_raw.push(dct_raw);
    }

    let mut arguments_raw = Vec::with_capacity(tx_call.arguments.len());
    for arg in tx_call.arguments.iter() {
        let arg_raw = bytes_as_raw(arg);
        arguments_raw.push(arg_raw);
    }

    TxCallRaw {
        from: address_as_raw(&tx_call.from),
        to: address_as_raw(&tx_call.to),
        value: None, // this is the old "value" field, which is now "moa_value". Only kept for backwards compatibility
        moa_value: rust_biguint_as_opt_raw(&tx_call.moa_value),
        dct_value: all_dct_raw,
        function: tx_call.function.clone(),
        arguments: arguments_raw,
        gas_limit: u64_as_raw(tx_call.gas_limit),
        gas_price: u64_as_raw(tx_call.gas_price),
    }
}

pub(crate) fn tx_query_as_raw(tx_query: &ScQueryDenali) -> TxQueryRaw {
    let mut arguments_raw = Vec::with_capacity(tx_query.arguments.len());
    for arg in tx_query.arguments.iter() {
        let arg_raw = bytes_as_raw(arg);
        arguments_raw.push(arg_raw);
    }

    TxQueryRaw {
        to: address_as_raw(&tx_query.to),
        function: tx_query.function.clone(),
        arguments: arguments_raw,
    }
}

pub(crate) fn tx_expect_as_raw(tx_expect: &TxExpectDenali) -> TxExpectRaw {
    let mut out_values_raw = Vec::with_capacity(tx_expect.out.len());
    for out_val in tx_expect.out.iter() {
        let out_raw = if out_val.len() == 1 && out_val[0] == b'*' {
            CheckBytesValueRaw::Star
        } else {
            CheckBytesValueRaw::Equal(bytes_as_raw(out_val))
        };

        out_values_raw.push(out_raw);
    }

    let msg_raw = if tx_expect.message == STAR_STR {
        CheckBytesValueRaw::Star
    } else {
        let denali_formatted_str = "str:".to_owned() + &tx_expect.message;
        CheckBytesValueRaw::Equal(ValueSubTree::Str(denali_formatted_str))
    };

    TxExpectRaw {
        out: CheckValueListRaw::CheckList(out_values_raw),
        status: CheckBytesValueRaw::Equal(u64_as_raw(tx_expect.status)),
        message: msg_raw,
        logs: CheckLogsRaw::Star,
        gas: CheckBytesValueRaw::Star,
        refund: CheckBytesValueRaw::Star,
    }
}

pub(crate) fn account_as_check_state_raw(acc: &AccountData) -> CheckAccountsRaw {
    let mut all_check_dct_raw = BTreeMap::new();
    for (token_id, dct_data) in acc.dct.iter() {
        let dct_data_raw = match dct_data_as_raw(dct_data) {
            DctRaw::Short(_) => unreachable!(), // this can't happen, dct_data_as_raw always returns the full format
            DctRaw::Full(full_raw) => full_raw,
        };
        let last_nonce_check = opt_raw_value_to_check_raw(&dct_data_raw.last_nonce);

        let mut dct_instances_check_raw = Vec::new();
        for inst_raw in dct_data_raw.instances.iter() {
            let inst_check_raw = CheckDctInstanceRaw {
                attributes: opt_raw_value_to_check_raw(&inst_raw.attributes),
                balance: opt_raw_value_to_check_raw(&inst_raw.balance),
                creator: opt_raw_value_to_check_raw(&inst_raw.creator),
                hash: opt_raw_value_to_check_raw(&inst_raw.hash),
                nonce: inst_raw
                    .nonce
                    .clone()
                    .unwrap_or_else(|| ValueSubTree::Str("0".to_owned())),
                royalties: opt_raw_value_to_check_raw(&inst_raw.royalties),
                uri: CheckValueListRaw::CheckList(
                    inst_raw
                        .uri
                        .iter()
                        .map(|v| CheckBytesValueRaw::Equal(v.clone()))
                        .collect(),
                ),
            };

            dct_instances_check_raw.push(inst_check_raw);
        }

        let mut roles_as_str = Vec::new();
        for role in dct_data.roles.get() {
            let role_str = String::from_utf8(role).unwrap();
            roles_as_str.push(role_str);
        }

        let dct_check_raw = CheckDctDataRaw {
            frozen: CheckBytesValueRaw::Unspecified,
            last_nonce: last_nonce_check,
            instances: CheckDctInstancesRaw::Equal(dct_instances_check_raw),
            roles: roles_as_str,
        };

        let token_id_str = bytes_to_denali_string_or_hex(token_id);
        all_check_dct_raw.insert(token_id_str, CheckDctRaw::Full(dct_check_raw));
    }

    let mut raw_storage = BTreeMap::new();
    for (key, value) in acc.storage.iter() {
        let key_as_str = bytes_to_denali_string_or_hex(key);
        let check_val_raw = CheckBytesValueRaw::Equal(bytes_as_raw(value));

        raw_storage.insert(key_as_str, check_val_raw);
    }

    let check_storage_raw = CheckStorageDetailsRaw {
        other_storages_allowed: false,
        storages: raw_storage,
    };
    let check_acc_raw = CheckAccountRaw {
        nonce: CheckBytesValueRaw::Equal(u64_as_raw(acc.nonce)),
        balance: CheckBytesValueRaw::Equal(rust_biguint_as_raw(&acc.moa_balance)),
        dct: CheckDctMapRaw::Equal(CheckDctMapContentsRaw {
            other_dcts_allowed: false,
            contents: all_check_dct_raw,
        }),
        owner: CheckBytesValueRaw::Star, // TODO: Add owner check?
        developer_rewards: CheckBytesValueRaw::Equal(rust_biguint_as_raw(&acc.developer_rewards)),
        storage: CheckStorageRaw::Equal(check_storage_raw),
        code: CheckBytesValueRaw::Star,
        async_call_data: CheckBytesValueRaw::Unspecified,
        comment: None,
        username: CheckBytesValueRaw::Unspecified,
    };

    let mut all_accounts_check_raw = BTreeMap::new();
    all_accounts_check_raw.insert(
        bytes_to_hex(acc.address.as_bytes()),
        Box::new(check_acc_raw),
    );

    CheckAccountsRaw {
        other_accounts_allowed: true, // so we only check the current account
        accounts: all_accounts_check_raw,
    }
}

pub(crate) fn opt_raw_value_to_check_raw(raw_value: &Option<ValueSubTree>) -> CheckBytesValueRaw {
    match raw_value {
        Some(val) => CheckBytesValueRaw::Equal(val.clone()),
        None => CheckBytesValueRaw::Unspecified,
    }
}

pub(crate) fn bytes_to_denali_string_or_hex(bytes: &[u8]) -> String {
    let conversion_result = String::from_utf8(bytes.to_vec());
    match conversion_result {
        core::result::Result::Ok(bytes_as_str) => format!("str:{bytes_as_str}"),
        core::result::Result::Err(_) => bytes_to_hex(bytes),
    }
}

pub(crate) fn rust_biguint_as_raw(big_uint: &num_bigint::BigUint) -> ValueSubTree {
    ValueSubTree::Str(big_uint.to_string())
}

pub(crate) fn rust_biguint_as_opt_raw(big_uint: &num_bigint::BigUint) -> Option<ValueSubTree> {
    if big_uint > &num_bigint::BigUint::zero() {
        Some(rust_biguint_as_raw(big_uint))
    } else {
        None
    }
}

pub(crate) fn address_as_raw(address: &Address) -> ValueSubTree {
    bytes_as_raw(address.as_bytes())
}

pub(crate) fn u64_as_raw(value: u64) -> ValueSubTree {
    ValueSubTree::Str(value.to_string())
}

pub(crate) fn bytes_as_raw(bytes: &[u8]) -> ValueSubTree {
    ValueSubTree::Str(bytes_to_hex(bytes))
}

pub(crate) fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}
