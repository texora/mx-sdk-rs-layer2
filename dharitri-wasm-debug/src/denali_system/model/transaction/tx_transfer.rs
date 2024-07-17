use crate::denali_system::model::{AddressValue, BigUintValue, U64Value};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::TxTransferRaw,
};

use super::{tx_interpret_util::interpret_moa_value, TxCall, TxDCT};

#[derive(Debug, Default)]
pub struct TxTransfer {
    pub from: AddressValue,
    pub to: AddressValue,
    pub moa_value: BigUintValue,
    pub dct_value: Vec<TxDCT>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            moa_value: interpret_moa_value(from.value, from.moa_value, context),
            dct_value: from
                .dct_value
                .iter()
                .map(|dct_value| TxDCT::interpret_from(dct_value.clone(), context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit.unwrap_or_default(), context),
            gas_price: U64Value::interpret_from(from.gas_price.unwrap_or_default(), context),
        }
    }
}

impl IntoRaw<TxTransferRaw> for TxTransfer {
    fn into_raw(self) -> TxTransferRaw {
        TxTransferRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            moa_value: self.moa_value.into_raw_opt(),
            dct_value: self
                .dct_value
                .into_iter()
                .map(|dct_value| dct_value.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw_opt(),
            gas_price: self.gas_price.into_raw_opt(),
        }
    }
}

impl TxTransfer {
    /// Converts to a TxCall, with empty endpoint and arguments.
    pub fn to_tx_call(&self) -> TxCall {
        TxCall {
            from: self.from.clone(),
            to: self.to.clone(),
            moa_value: self.moa_value.clone(),
            dct_value: self.dct_value.clone(),
            function: String::new(),
            arguments: Vec::new(),
            gas_limit: self.gas_limit.clone(),
            gas_price: self.gas_price.clone(),
        }
    }
}
