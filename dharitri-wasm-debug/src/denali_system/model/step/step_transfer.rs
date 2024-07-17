use crate::denali_system::model::{
    AddressValue, BigUintValue, BytesValue, TxDCT, TxTransfer, TxValidatorReward, U64Value,
};

#[derive(Debug, Default)]
pub struct TransferStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub comment: Option<String>,
    pub tx: Box<TxTransfer>,
}

#[derive(Debug)]
pub struct ValidatorRewardStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub comment: Option<String>,
    pub tx: Box<TxValidatorReward>,
}

impl TransferStep {
    pub fn new() -> Self {
        // 50,000 is the gas limit for simple MOA transfers, so it is default for convenience
        // DCT transfers will need more
        Self::default().gas_limit("50,000")
    }

    pub fn from<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.from = AddressValue::from(address);
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.to = AddressValue::from(address);
        self
    }

    pub fn moa_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: From<A>,
    {
        if !self.tx.dct_value.is_empty() {
            panic!("Cannot transfer both MOA and DCT");
        }

        self.tx.moa_value = BigUintValue::from(amount);
        self
    }

    pub fn dct_transfer<T, N, A>(mut self, token_id: T, token_nonce: N, amount: A) -> Self
    where
        BytesValue: From<T>,
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        if self.tx.moa_value.value > 0u32.into() {
            panic!("Cannot transfer both MOA and DCT");
        }

        self.tx.dct_value.push(TxDCT {
            dct_token_identifier: BytesValue::from(token_id),
            nonce: U64Value::from(token_nonce),
            dct_value: BigUintValue::from(amount),
        });

        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: From<V>,
    {
        self.tx.gas_limit = U64Value::from(value);
        self
    }
}
