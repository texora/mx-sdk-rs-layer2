use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckDctInstancesRaw,
};

use super::CheckDctInstance;

#[derive(Debug)]
pub enum CheckDctInstances {
    Star,
    Equal(Vec<CheckDctInstance>),
}

impl CheckDctInstances {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDctInstances::Star)
    }

    pub fn contains_nonce(&self, nonce: u64) -> bool {
        match &self {
            CheckDctInstances::Equal(eq) => {
                for expected_value in eq.iter() {
                    if expected_value.nonce.value == nonce {
                        return true;
                    }
                }
            },
            CheckDctInstances::Star => {},
        }
        false
    }
}

impl Default for CheckDctInstances {
    fn default() -> Self {
        CheckDctInstances::Equal(Vec::new())
    }
}

impl InterpretableFrom<CheckDctInstancesRaw> for CheckDctInstances {
    fn interpret_from(from: CheckDctInstancesRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDctInstancesRaw::Unspecified => CheckDctInstances::Star,
            CheckDctInstancesRaw::Star => CheckDctInstances::Star,
            CheckDctInstancesRaw::Equal(m) => CheckDctInstances::Equal(
                m.into_iter()
                    .map(|v| CheckDctInstance::interpret_from(v, context))
                    .collect(),
            ),
        }
    }
}

impl IntoRaw<CheckDctInstancesRaw> for CheckDctInstances {
    fn into_raw(self) -> CheckDctInstancesRaw {
        match self {
            CheckDctInstances::Equal(eq) => {
                CheckDctInstancesRaw::Equal(eq.into_iter().map(|cei| cei.into_raw()).collect())
            },
            CheckDctInstances::Star => CheckDctInstancesRaw::Star,
        }
    }
}
