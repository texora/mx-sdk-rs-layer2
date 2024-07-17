use crate::num_bigint::BigUint;
use num_traits::Zero;
use std::{
    collections::BTreeMap,
    fmt::{self, Write},
};

use crate::verbose_hex_list;

use super::{DctInstance, DctInstanceMetadata};

#[derive(Clone, Debug, Default)]
pub struct DctInstances(BTreeMap<u64, DctInstance>);

impl DctInstances {
    pub fn new() -> Self {
        DctInstances(BTreeMap::new())
    }

    pub fn new_from_hash(hash: BTreeMap<u64, DctInstance>) -> Self {
        DctInstances(hash)
    }

    pub fn add(&mut self, nonce: u64, value: BigUint) {
        if self.0.contains_key(&nonce) {
            let dct_balance = self.0.get_mut(&nonce).unwrap();
            dct_balance.balance += value;
        } else {
            let mut instance = DctInstance::default(nonce);
            instance.balance = value;
            self.push_instance(instance)
        }
    }

    pub fn push_instance(&mut self, instance: DctInstance) {
        self.0.insert(instance.nonce, instance);
    }

    pub fn increase_balance(
        &mut self,
        nonce: u64,
        value: &BigUint,
        metadata: DctInstanceMetadata,
    ) {
        let instance = self.0.entry(nonce).or_insert_with(|| DctInstance {
            nonce,
            balance: BigUint::zero(),
            metadata,
        });
        instance.balance += value;
    }

    pub fn set_balance(&mut self, nonce: u64, value: &BigUint, metadata: DctInstanceMetadata) {
        let instance = self.0.entry(nonce).or_insert_with(|| DctInstance {
            nonce,
            balance: BigUint::zero(),
            metadata,
        });
        instance.balance = value.clone();
    }

    pub fn get_by_nonce(&self, nonce: u64) -> Option<&DctInstance> {
        self.0.get(&nonce)
    }

    pub fn get_by_nonce_or_default(&self, nonce: u64) -> DctInstance {
        if let Some(instance) = self.0.get(&nonce) {
            instance.clone()
        } else {
            DctInstance::default(nonce)
        }
    }

    pub fn get_mut_by_nonce(&mut self, nonce: u64) -> Option<&mut DctInstance> {
        self.0.get_mut(&nonce)
    }

    pub fn get_instances(&self) -> &BTreeMap<u64, DctInstance> {
        &self.0
    }

    pub fn is_empty_dct(&self) -> bool {
        self.0.values().all(DctInstance::is_empty_dct)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for DctInstances {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut instance_buf = String::new();
        for (_, value) in self.0.iter() {
            let creator_encoded = if let Some(creator) = &value.metadata.creator {
                hex::encode(creator)
            } else {
                "".to_string()
            };
            write!(
                instance_buf,
                "{{
                    nonce: {},
                    balance: {},
                    creator: {},
                    royalties: {},
                    hash: {},
                    uri: [{} ],
                    attributes: {}
                }}",
                value.nonce,
                value.balance,
                creator_encoded,
                value.metadata.royalties,
                hex::encode(
                    value
                        .metadata
                        .hash
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .as_slice()
                ),
                verbose_hex_list(value.metadata.uri.as_slice()),
                hex::encode(value.metadata.attributes.as_slice())
            )?;
        }
        Ok(())
    }
}
