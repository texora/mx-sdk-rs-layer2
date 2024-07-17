use crate::denali_system::model::{AddressValue, BigUintValue};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::TxValidatorRewardRaw,
};

use super::tx_interpret_util::interpret_moa_value;

#[derive(Debug)]
pub struct TxValidatorReward {
    pub to: AddressValue,
    pub moa_value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: AddressValue::interpret_from(from.to, context),
            moa_value: interpret_moa_value(from.value, from.moa_value, context),
        }
    }
}

impl IntoRaw<TxValidatorRewardRaw> for TxValidatorReward {
    fn into_raw(self) -> TxValidatorRewardRaw {
        TxValidatorRewardRaw {
            to: self.to.into_raw(),
            value: None,
            moa_value: Some(self.moa_value.into_raw()),
        }
    }
}
