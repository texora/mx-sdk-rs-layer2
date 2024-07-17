use super::{CheckDctData, CheckDctInstance, CheckDctInstances};
use crate::denali_system::model::{BigUintValue, CheckValue, U64Value};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::{CheckDctRaw, ValueSubTree},
};
use num_bigint::BigUint;

#[derive(Debug)]
pub enum CheckDct {
    Short(BigUintValue),
    Full(CheckDctData),
}

impl CheckDct {
    pub fn convert_to_short_if_possible(&mut self) {
        if let CheckDct::Full(dct_check) = self {
            let has_single_fungible_instance =
                if let CheckDctInstances::Equal(check_instance) = &dct_check.instances {
                    check_instance.len() == 1 && check_instance[0].is_simple_fungible()
                } else {
                    false
                };

            if has_single_fungible_instance
                && dct_check.frozen.is_star()
                && dct_check.last_nonce.is_star()
            {
                let balance =
                    if let CheckDctInstances::Equal(check_instances) = &dct_check.instances {
                        match &check_instances[0].balance {
                            CheckValue::Star => BigUintValue {
                                original: ValueSubTree::Str("*".to_string()),
                                value: BigUint::from(0u32),
                            },
                            CheckValue::Equal(val) => val.clone(),
                        }
                    } else {
                        unreachable!();
                    };

                *self = CheckDct::Short(balance);
            }
        }
    }

    pub fn convert_to_full(&mut self) {
        if let CheckDct::Short(prev_balance_check) = self {
            let new_instances_check = vec![CheckDctInstance {
                balance: CheckValue::Equal(prev_balance_check.clone()),
                ..Default::default()
            }];

            let new_dct_check = CheckDctData {
                instances: CheckDctInstances::Equal(new_instances_check),
                ..Default::default()
            };
            *self = CheckDct::Full(new_dct_check);
        }
    }

    pub fn add_balance_check<N, V>(&mut self, nonce_expr: N, balance_expr: V)
    where
        U64Value: InterpretableFrom<N>,
        BigUintValue: InterpretableFrom<V>,
    {
        let ctx = InterpreterContext::default();
        let nonce = U64Value::interpret_from(nonce_expr, &ctx);
        let balance = BigUintValue::interpret_from(balance_expr, &ctx);

        self.convert_to_full();

        if let CheckDct::Full(prev_dct_check) = self {
            match &mut prev_dct_check.instances {
                CheckDctInstances::Star => {
                    let new_instances_check = vec![CheckDctInstance {
                        nonce,
                        balance: CheckValue::Equal(balance),
                        ..Default::default()
                    }];

                    prev_dct_check.instances = CheckDctInstances::Equal(new_instances_check);
                },
                CheckDctInstances::Equal(dct_instance_check) => {
                    if let Some(i) = dct_instance_check
                        .iter()
                        .position(|item| item.nonce.value == nonce.value)
                    {
                        dct_instance_check[i].balance = CheckValue::Equal(balance);
                    } else {
                        dct_instance_check.push(CheckDctInstance {
                            nonce,
                            balance: CheckValue::Equal(balance),
                            ..Default::default()
                        });
                    }
                },
            }
        }
    }
}

impl InterpretableFrom<CheckDctRaw> for CheckDct {
    fn interpret_from(from: CheckDctRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDctRaw::Full(m) => CheckDct::Full(CheckDctData::interpret_from(m, context)),
            CheckDctRaw::Short(v) => CheckDct::Short(BigUintValue::interpret_from(v, context)),
        }
    }
}

impl IntoRaw<CheckDctRaw> for CheckDct {
    fn into_raw(mut self) -> CheckDctRaw {
        self.convert_to_short_if_possible();

        match self {
            CheckDct::Full(m) => CheckDctRaw::Full(m.into_raw()),
            CheckDct::Short(v) => CheckDctRaw::Short(v.into_raw()),
        }
    }
}
