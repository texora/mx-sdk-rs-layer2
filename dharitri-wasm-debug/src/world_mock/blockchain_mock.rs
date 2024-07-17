use crate::{
    denali_system::model::Scenario,
    num_bigint::BigUint,
    tx_execution::{init_builtin_functions, BuiltinFunctionMap},
    tx_mock::BlockchainUpdate,
};
use dharitri_wasm::types::heap::Address;
use denali::{interpret_trait::InterpreterContext, value_interpreter::interpret_string};
use num_traits::Zero;
use std::{collections::HashMap, path::PathBuf, rc::Rc};

use super::{AccountData, BlockInfo, ContractMap};

const DHARITRI_REWARD_KEY: &[u8] = b"DHARITRIreward";

#[derive(Debug)]
pub struct BlockchainMock {
    pub accounts: HashMap<Address, AccountData>,
    pub builtin_functions: Rc<BuiltinFunctionMap>,
    pub addr_to_denali_string_map: HashMap<Address, String>,
    pub new_addresses: HashMap<(Address, u64), Address>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub contract_map: ContractMap,
    pub current_dir: PathBuf,
    pub denali_trace: Scenario,
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            builtin_functions: Rc::new(init_builtin_functions()),
            addr_to_denali_string_map: HashMap::new(),
            new_addresses: HashMap::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
            contract_map: ContractMap::default(),
            current_dir: std::env::current_dir().unwrap(),
            denali_trace: Scenario::default(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainMock {
    pub fn account_exists(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn contains_contract(&self, contract_path_expr: &str) -> bool {
        let contract_bytes = interpret_string(
            contract_path_expr,
            &InterpreterContext::new(self.current_dir.clone()),
        );

        self.contract_map.contains_contract(&contract_bytes)
    }

    pub fn commit_updates(&mut self, updates: BlockchainUpdate) {
        updates.apply(self);
    }

    pub fn increase_account_nonce(&mut self, address: &Address) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn subtract_tx_gas(&mut self, address: &Address, gas_limit: u64, gas_price: u64) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            account.moa_balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        account.moa_balance -= &gas_cost;
    }

    pub fn increase_validator_reward(&mut self, address: &Address, amount: &BigUint) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.moa_balance += amount;
        let mut storage_v_rew =
            if let Some(old_storage_value) = account.storage.get(DHARITRI_REWARD_KEY) {
                BigUint::from_bytes_be(old_storage_value)
            } else {
                BigUint::zero()
            };
        storage_v_rew += amount;
        account
            .storage
            .insert(DHARITRI_REWARD_KEY.to_vec(), storage_v_rew.to_bytes_be());
    }

    pub(crate) fn with_borrowed<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(Self) -> (R, Self),
    {
        let obj = std::mem::replace(self, Self::new());
        let (result, obj) = f(obj);
        *self = obj;
        result
    }
}
