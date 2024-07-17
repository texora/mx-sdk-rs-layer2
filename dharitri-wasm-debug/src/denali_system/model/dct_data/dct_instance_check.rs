use crate::denali_system::model::{BigUintValue, BytesValue, CheckValue, CheckValueList, U64Value};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckDctInstanceRaw,
};

#[derive(Debug, Default)]
pub struct CheckDctInstance {
    pub nonce: U64Value,
    pub balance: CheckValue<BigUintValue>,
    pub creator: CheckValue<BytesValue>,
    pub royalties: CheckValue<U64Value>,
    pub hash: CheckValue<BytesValue>,
    pub uri: CheckValueList,
    pub attributes: CheckValue<BytesValue>,
}

impl CheckDctInstance {
    pub fn is_simple_fungible(&self) -> bool {
        let is_uri_star = matches!(self.uri, CheckValue::Star);

        self.nonce.value == 0
            && self.creator.is_star()
            && self.royalties.is_star()
            && self.hash.is_star()
            && is_uri_star
            && self.attributes.is_star()
    }
}

impl InterpretableFrom<CheckDctInstanceRaw> for CheckDctInstance {
    fn interpret_from(from: CheckDctInstanceRaw, context: &InterpreterContext) -> Self {
        CheckDctInstance {
            nonce: U64Value::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            creator: CheckValue::<BytesValue>::interpret_from(from.creator, context),
            royalties: CheckValue::<U64Value>::interpret_from(from.royalties, context),
            hash: CheckValue::<BytesValue>::interpret_from(from.hash, context),
            uri: CheckValueList::interpret_from(from.uri, context),
            attributes: CheckValue::<BytesValue>::interpret_from(from.attributes, context),
        }
    }
}

impl IntoRaw<CheckDctInstanceRaw> for CheckDctInstance {
    fn into_raw(self) -> CheckDctInstanceRaw {
        CheckDctInstanceRaw {
            nonce: self.nonce.into_raw(),
            balance: self.balance.into_raw(),
            creator: self.creator.into_raw(),
            royalties: self.royalties.into_raw(),
            hash: self.hash.into_raw(),
            uri: self.uri.into_raw(),
            attributes: self.attributes.into_raw(),
        }
    }
}
