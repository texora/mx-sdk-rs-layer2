use crate::denali_system::model::BigUintValue;
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ValueSubTree,
};

pub fn interpret_moa_value(
    opt_legacy_value: Option<ValueSubTree>,
    opt_moa_value: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BigUintValue {
    let mut moa_value = BigUintValue::default();
    if let Some(parsed_legacy_value) = opt_legacy_value {
        moa_value = BigUintValue::interpret_from(parsed_legacy_value, context);
    }
    if let Some(parsed_moa_value) = opt_moa_value {
        moa_value = BigUintValue::interpret_from(parsed_moa_value, context);
    }
    moa_value
}
