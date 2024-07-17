use crate::{
    api::ManagedTypeApi,
    types::{BigUint, MoaOrDctTokenIdentifier},
};

use dharitri_codec::{
    dharitri_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as dharitri_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::DctTokenPayment;

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct MoaOrDctTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: MoaOrDctTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> MoaOrDctTokenPayment<M> {
    pub fn no_payment() -> Self {
        MoaOrDctTokenPayment {
            token_identifier: MoaOrDctTokenIdentifier::moa(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: MoaOrDctTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        MoaOrDctTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just DCT or terminate execution if the token is MOA.
    pub fn unwrap_dct(self) -> DctTokenPayment<M> {
        DctTokenPayment::new(
            self.token_identifier.unwrap_dct(),
            self.token_nonce,
            self.amount,
        )
    }

    pub fn into_tuple(self) -> (MoaOrDctTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(MoaOrDctTokenIdentifier<M>, u64, BigUint<M>)>
    for MoaOrDctTokenPayment<M>
{
    #[inline]
    fn from(value: (MoaOrDctTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<DctTokenPayment<M>> for MoaOrDctTokenPayment<M> {
    fn from(dct_payment: DctTokenPayment<M>) -> Self {
        MoaOrDctTokenPayment {
            token_identifier: MoaOrDctTokenIdentifier::dct(dct_payment.token_identifier),
            token_nonce: dct_payment.token_nonce,
            amount: dct_payment.amount,
        }
    }
}

impl<M> CodecFromSelf for MoaOrDctTokenPayment<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for MoaOrDctTokenPayment<M> where M: ManagedTypeApi {}
