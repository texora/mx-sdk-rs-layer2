use crate::denali_system::model::{
    AddressKey, BytesValue, CheckAccounts, CheckDct, CheckDctData, CheckDctInstance,
    CheckDctInstances, CheckDctMap, CheckStateStep, CheckStorage, CheckValue, Checkable, Step,
};
use num_traits::Zero;

use crate::{
    bytes_to_string, verbose_hex, verbose_hex_list,
    world_mock::{AccountDct, BlockchainMock, DctData, DctInstance, DctInstances},
};

impl BlockchainMock {
    pub fn denali_check_state(&mut self, check_state_step: CheckStateStep) -> &mut Self {
        execute(self, &check_state_step.accounts);
        self.denali_trace
            .steps
            .push(Step::CheckState(check_state_step));
        self
    }

    pub fn denali_dump_state(&mut self) -> &mut Self {
        self.print_accounts();
        self
    }
}

fn execute(state: &BlockchainMock, accounts: &CheckAccounts) {
    for (expected_address, expected_account) in accounts.accounts.iter() {
        if let Some(account) = state.accounts.get(&expected_address.value) {
            assert!(
                expected_account.nonce.check(account.nonce),
                "bad account nonce. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.nonce,
                account.nonce
            );

            assert!(
                expected_account.balance.check(&account.moa_balance),
                "bad account balance. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.balance,
                account.moa_balance
            );

            assert!(
                expected_account.username.check(&account.username),
                "bad account username. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.username,
                std::str::from_utf8(account.username.as_slice()).unwrap()
            );
            let default_value = &Vec::new();
            let actual_code = account.contract_path.as_ref().unwrap_or(default_value);
            assert!(
                expected_account.code.check(actual_code),
                "bad account code. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.code,
                std::str::from_utf8(actual_code.as_slice()).unwrap()
            );

            assert!(
                expected_account
                    .developer_rewards
                    .check(&account.developer_rewards),
                "bad account developerRewards. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.developer_rewards,
                account.developer_rewards
            );

            if let CheckStorage::Equal(eq) = &expected_account.storage {
                let default_value = &Vec::new();
                for (expected_key, expected_value) in eq.storages.iter() {
                    let actual_value = account
                        .storage
                        .get(&expected_key.value)
                        .unwrap_or(default_value);
                    assert!(
                        expected_value.check(actual_value),
                        "bad storage value. Address: {}. Key: {}. Want: {}. Have: {}",
                        expected_address,
                        expected_key,
                        expected_value,
                        verbose_hex(actual_value)
                    );
                }

                let default_check_value = CheckValue::Equal(BytesValue::empty());
                for (actual_key, actual_value) in account.storage.iter() {
                    let expected_value = eq
                        .storages
                        .get(&actual_key.clone().into())
                        .unwrap_or(&default_check_value);
                    if expected_value.to_string() == default_check_value.to_string()
                        && !eq.other_storages_allowed
                    {
                        assert!(
                            expected_value.check(actual_value),
                            "bad storage value. Address: {}. Key: {}. Want: {}. Have: {}",
                            expected_address,
                            verbose_hex(actual_key),
                            expected_value,
                            verbose_hex(actual_value)
                        );
                    }
                }
            }
            check_account_dct(expected_address, &expected_account.dct, &account.dct);
        } else {
            assert!(
                accounts.other_accounts_allowed,
                "Expected account not found"
            );
        }
    }
}

pub fn check_account_dct(address: &AddressKey, expected: &CheckDctMap, actual: &AccountDct) {
    match expected {
        CheckDctMap::Star => {},
        CheckDctMap::Equal(contents) => {
            for (key, expected_value) in contents.contents.iter() {
                let actual_value = actual.get_by_identifier_or_default(key.value.as_slice());
                match expected_value {
                    CheckDct::Short(expected_balance) => {
                        if expected_balance.value.is_zero() {
                            assert!(
                                actual_value.is_empty(),
                                "No balance expected for DCT token address: {}. token name: {}. nonce: {}.",
                                address,
                                bytes_to_string(key.value.as_slice()),
                                0
                            );
                        } else {
                            assert!(
                                actual_value.instances.len() == 1,
                                "One DCT instance expected, with nonce 0 for address: {}. token name: {}.",
                                address,
                                bytes_to_string(key.value.as_slice()),
                            );
                            let single_instance = actual_value
                                .instances
                                .get_by_nonce(0)
                                .unwrap_or_else(|| panic!("Expected fungible DCT with none 0"));
                            assert_eq!(
                                single_instance.balance,
                                expected_balance.value,
                                "Unexpected fungible token balance for address: {}. token name: {}.",
                                address,
                                bytes_to_string(key.value.as_slice()),
                            );
                        }
                    },
                    CheckDct::Full(expected_dct) => {
                        check_dct_data(
                            address,
                            bytes_to_string(key.value.as_slice()),
                            expected_dct,
                            &actual_value,
                        );
                    },
                }
            }

            if !contents.other_dcts_allowed || contents.contents.iter().len() == 0 {
                for (token_identifier, actual_value) in actual.iter() {
                    if contents.contains_token(token_identifier) {
                        continue;
                    }
                    check_dct_data(
                        address,
                        bytes_to_string(token_identifier),
                        &CheckDctData::default(),
                        actual_value,
                    );
                }
            }
        },
        CheckDctMap::Unspecified => {
            for (token_identifier, actual_value) in actual.iter() {
                check_dct_data(
                    address,
                    bytes_to_string(token_identifier),
                    &CheckDctData::default(),
                    actual_value,
                );
            }
        },
    }
}

pub fn check_dct_data(
    address: &AddressKey,
    token: String,
    expected: &CheckDctData,
    actual: &DctData,
) {
    let mut errors: Vec<String> = vec!["".to_string()];
    check_token_instances(
        address,
        token.clone(),
        &expected.instances,
        &actual.instances,
        &mut errors,
    );
    if !expected.last_nonce.check(actual.last_nonce) {
        errors.push(format!(
            "bad last nonce. Address: {}. Token Name: {}. Want: {}. Have: {}\n",
            address, token, expected.last_nonce, &actual.last_nonce
        ));
    }

    if !expected.frozen.check(u64::from(actual.frozen)) {
        errors.push(format!(
            "bad last nonce. Address: {}. Token Name: {}. Want: {}. Have: {}\n",
            address, token, expected.frozen, &actual.frozen
        ));
    }

    errors.push("".to_string());
    assert!(errors.len() == 2, "{}", errors.join("\n"));
}

pub fn check_token_instances(
    address: &AddressKey,
    token: String,
    expected: &CheckDctInstances,
    actual: &DctInstances,
    errors: &mut Vec<String>,
) {
    match expected {
        CheckDctInstances::Equal(eq) => {
            for expected_value in eq.iter() {
                let actual_value = actual.get_by_nonce_or_default(expected_value.nonce.value);
                check_token_instance(address, &token, expected_value, &actual_value, errors);
            }

            let default_expected_value = CheckDctInstance::default();
            for (actual_key, actual_value) in actual.get_instances().iter() {
                if !expected.contains_nonce(*actual_key) {
                    check_token_instance(
                        address,
                        &token,
                        &default_expected_value,
                        actual_value,
                        errors,
                    );
                }
            }
        },
        CheckDctInstances::Star => {
            // nothing to be done for *
        },
    }
}

pub fn check_token_instance(
    address: &AddressKey,
    token: &str,
    expected_value: &CheckDctInstance,
    actual_value: &DctInstance,
    errors: &mut Vec<String>,
) {
    if !expected_value.balance.check(&actual_value.balance) {
        errors.push(format!(
            "bad dct balance. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address,
            token,
            expected_value.nonce.value,
            expected_value.balance,
            &actual_value.balance,
        ))
    }

    if !expected_value.balance.check(&actual_value.balance) {
        errors.push(format!(
            "bad dct balance. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address,
            token,
            expected_value.nonce.value,
            expected_value.balance,
            &actual_value.balance,
        ))
    }
    let actual_creator = if let Some(creator) = &actual_value.metadata.creator {
        creator.as_ref()
    } else {
        &[]
    };
    if !expected_value.creator.check(actual_creator) {
        errors.push(format!(
            "bad dct creator. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address,
            token,
            expected_value.nonce.value,
            expected_value.creator,
            verbose_hex(actual_creator),
        ))
    }

    let actual_royalties = actual_value.metadata.royalties;
    if !expected_value.royalties.check(actual_royalties) {
        errors.push(format!(
            "bad dct royalties. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address, token, expected_value.nonce.value, expected_value.royalties, actual_royalties
        ))
    }

    let actual_hash = actual_value.metadata.hash.clone().unwrap_or_default();
    if !expected_value.hash.check(&actual_hash) {
        errors.push(format!(
            "bad dct hash. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address,
            token,
            expected_value.nonce.value,
            expected_value.hash,
            verbose_hex(&actual_hash),
        ))
    }

    let actual_uri = actual_value.metadata.uri.as_slice();
    if !expected_value.uri.check(actual_uri) {
        errors.push(format!(
            "bad dct uri. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address,
            token,
            expected_value.nonce.value,
            expected_value.uri.pretty_str(),
            verbose_hex_list(actual_uri),
        ))
    }

    if !expected_value
        .attributes
        .check(&actual_value.metadata.attributes)
    {
        errors.push(format!(
            "bad dct attributes. Address: {}. Token {}. Nonce {}. Want: {}. Have: {}",
            address,
            token,
            expected_value.nonce.value,
            expected_value.attributes,
            verbose_hex(&actual_value.metadata.attributes),
        ))
    }
}
