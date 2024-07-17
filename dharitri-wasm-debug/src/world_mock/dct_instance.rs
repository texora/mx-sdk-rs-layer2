use crate::num_bigint::BigUint;
use num_traits::Zero;

use super::DctInstanceMetadata;

/// Holds the data for a Dharitri standard digital token transaction
#[derive(Clone, Default, Debug)]
pub struct DctInstance {
    pub nonce: u64,
    pub balance: BigUint,
    pub metadata: DctInstanceMetadata,
}

impl DctInstance {
    pub fn default(nonce: u64) -> Self {
        DctInstance {
            nonce,
            balance: BigUint::zero(),
            metadata: DctInstanceMetadata::default(),
        }
    }

    pub fn fungible(balance: BigUint) -> Self {
        DctInstance {
            nonce: 0,
            balance,
            metadata: DctInstanceMetadata::default(),
        }
    }

    pub fn is_empty_dct(&self) -> bool {
        self.balance.is_zero()
    }
}
