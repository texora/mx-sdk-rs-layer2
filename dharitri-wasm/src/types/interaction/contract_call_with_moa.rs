use dharitri_codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall};

/// Holds data for calling another contract, with MOA payment only.
///
/// Gets created when chaining method `with_moa_transfer`.
///
/// If the payment is zero, it bevahes exactly like `ContractCallNoPayment`.
///
/// It also represents the normalized form of any contract call, since DCT transfers
/// (the only payment not available here) get converted to builtin function calls in normalized form.
#[must_use]
pub struct ContractCallWithMoa<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub moa_payment: BigUint<SA>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMoa<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithMoa<SA, Self::OriginalResult> {
        // no DCT, no conversion needed
        self
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        self.basic.transfer_execute_moa(self.moa_payment);
    }
}

impl<SA, OriginalResult> ContractCallWithMoa<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_moa_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        moa_payment: BigUint<SA>,
    ) -> Self {
        ContractCallWithMoa {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            moa_payment,
        }
    }
}
