use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckDctMapRaw,
};

use super::CheckDctMapContents;

#[derive(Debug)]
pub enum CheckDctMap {
    Unspecified,
    Star,
    Equal(CheckDctMapContents),
}

impl Default for CheckDctMap {
    fn default() -> Self {
        CheckDctMap::Unspecified
    }
}

impl CheckDctMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDctMap::Star)
    }
}

impl InterpretableFrom<CheckDctMapRaw> for CheckDctMap {
    fn interpret_from(from: CheckDctMapRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDctMapRaw::Unspecified => CheckDctMap::Unspecified,
            CheckDctMapRaw::Star => CheckDctMap::Star,
            CheckDctMapRaw::Equal(m) => {
                CheckDctMap::Equal(CheckDctMapContents::interpret_from(m, context))
            },
        }
    }
}

impl IntoRaw<CheckDctMapRaw> for CheckDctMap {
    fn into_raw(self) -> CheckDctMapRaw {
        match self {
            CheckDctMap::Unspecified => CheckDctMapRaw::Unspecified,
            CheckDctMap::Star => CheckDctMapRaw::Star,
            CheckDctMap::Equal(value) => CheckDctMapRaw::Equal(value.into_raw()),
        }
    }
}
