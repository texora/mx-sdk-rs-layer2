use dharitri_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, MoaOrDctTokenIdentifier, MoaOrDctTokenPayment, ManagedAddress, ManagedBuffer,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithMoa};

/// Holds data for calling another contract, with a single payment, either MOA or a single DCT token.
///
/// Gets created when chaining method `with_moa_or_single_dct_transfer`.
#[must_use]
pub struct ContractCallWithMoaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) payment: MoaOrDctTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallWithMoaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_normalized_moa(self) -> ContractCallWithMoa<SA, OriginalResult> {
        ContractCallWithMoa {
            basic: self.basic,
            moa_payment: self.payment.amount,
        }
    }

    fn into_normalized_dct(self) -> ContractCallWithMoa<SA, OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_single_transfer_dct_call(self.payment.unwrap_dct())
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMoaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithMoa<SA, Self::OriginalResult> {
        if self.payment.token_identifier.is_moa() {
            self.into_normalized_moa()
        } else {
            // Because we know that there can be at most one DCT payment,
            // there is no need to call the full `convert_to_dct_transfer_call`.
            self.into_normalized_dct()
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        if self.payment.token_identifier.is_moa() {
            self.basic.transfer_execute_moa(self.payment.amount);
        } else {
            self.basic
                .transfer_execute_single_dct(self.payment.unwrap_dct());
        }
    }
}

impl<SA, OriginalResult> ContractCallWithMoaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_moa_or_single_dct_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        token_identifier: MoaOrDctTokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithMoaOrSingleDct {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment: MoaOrDctTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
