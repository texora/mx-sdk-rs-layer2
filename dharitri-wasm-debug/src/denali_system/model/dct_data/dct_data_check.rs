use crate::denali_system::model::{CheckValue, U64Value};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckDctDataRaw,
};

use super::CheckDctInstances;

#[derive(Debug, Default)]
pub struct CheckDctData {
    pub instances: CheckDctInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckDctDataRaw> for CheckDctData {
    fn interpret_from(from: CheckDctDataRaw, context: &InterpreterContext) -> Self {
        CheckDctData {
            instances: CheckDctInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}

impl IntoRaw<CheckDctDataRaw> for CheckDctData {
    fn into_raw(self) -> CheckDctDataRaw {
        CheckDctDataRaw {
            instances: self.instances.into_raw(),
            last_nonce: self.last_nonce.into_raw(),
            roles: Vec::new(),
            frozen: self.frozen.into_raw(),
        }
    }
}
