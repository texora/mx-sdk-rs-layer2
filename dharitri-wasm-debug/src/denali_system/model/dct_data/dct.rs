use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::{DctFullRaw, DctRaw},
};

use super::{DctInstance, DctObject};
use crate::denali_system::model::{BigUintValue, BytesValue, U64Value};

#[derive(Debug)]
pub enum Dct {
    Short(BigUintValue),
    Full(DctObject),
}

impl Dct {
    pub fn convert_to_short_if_possible(&mut self) {
        if let Dct::Full(dct_obj) = self {
            if dct_obj.is_short_form() {
                *self = Self::Short(dct_obj.instances[0].balance.clone().unwrap())
            }
        }
    }

    pub fn convert_to_full(&mut self) {
        if let Dct::Short(balance) = self {
            let mut new_dct_obj = DctObject::default();
            new_dct_obj.set_balance(0u64, balance.clone());

            *self = Self::Full(new_dct_obj);
        }
    }

    pub fn set_balance<N, A>(&mut self, token_nonce_expr: N, amount_expr: A)
    where
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        self.convert_to_full();

        if let Dct::Full(dct_obj) = self {
            dct_obj.set_balance(token_nonce_expr, amount_expr);
        }
    }

    pub fn get_mut_dct_object(&mut self) -> &mut DctObject {
        self.convert_to_full();

        if let Dct::Full(dct_obj) = self {
            return dct_obj;
        }

        unreachable!()
    }
}

impl InterpretableFrom<DctRaw> for Dct {
    fn interpret_from(from: DctRaw, context: &InterpreterContext) -> Self {
        match from {
            DctRaw::Short(short_dct) => {
                Dct::Short(BigUintValue::interpret_from(short_dct, context))
            },
            DctRaw::Full(full_dct) => Dct::Full(DctObject {
                token_identifier: full_dct
                    .token_identifier
                    .map(|b| BytesValue::interpret_from(b, context)),
                instances: full_dct
                    .instances
                    .into_iter()
                    .map(|instance| DctInstance::interpret_from(instance, context))
                    .collect(),
                last_nonce: full_dct
                    .last_nonce
                    .map(|b| U64Value::interpret_from(b, context)),
                roles: full_dct.roles,
                frozen: full_dct
                    .frozen
                    .map(|b| U64Value::interpret_from(b, context)),
            }),
        }
    }
}

impl IntoRaw<DctRaw> for Dct {
    fn into_raw(mut self) -> DctRaw {
        self.convert_to_short_if_possible();

        match self {
            Dct::Short(short) => DctRaw::Short(short.original),
            Dct::Full(eo) => DctRaw::Full(DctFullRaw {
                token_identifier: eo.token_identifier.map(|ti| ti.original),
                instances: eo
                    .instances
                    .into_iter()
                    .map(|inst| inst.into_raw())
                    .collect(),
                last_nonce: eo.last_nonce.map(|ti| ti.original),
                roles: eo.roles,
                frozen: eo.frozen.map(|ti| ti.original),
            }),
        }
    }
}
