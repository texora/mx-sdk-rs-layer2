#![no_std]
#![allow(clippy::type_complexity)]

dharitri_wasm::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Andes functionality for accepting MOA and DCT payments.
#[dharitri_wasm::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value(
        &self,
    ) -> MultiValue2<BigUint, ManagedVec<Self::Api, DctTokenPayment<Self::Api>>> {
        (
            self.call_value().moa_value(),
            self.call_value().all_dct_transfers(),
        )
            .into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedVec<DctTokenPayment<Self::Api>>,
    ) -> ManagedVec<DctTokenPayment<Self::Api>> {
        payments
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_3(&self) -> MultiValue3<DctTokenPayment, DctTokenPayment, DctTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_dct();
        (payment_a, payment_b, payment_c).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment_amount] payment: BigUint,
        #[payment_token] token: MoaOrDctTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let token = self.call_value().moa_or_single_dct().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: MoaOrDctTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let payment = self.call_value().moa_or_single_dct();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let payment = self.call_value().moa_or_single_dct();
        (payment.amount, payment.token_identifier).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_1(
        &self,
        #[payment_token] token: MoaOrDctTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let payment = self.call_value().moa_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let token = self.call_value().moa_or_single_dct().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_3(
        &self,
        #[payment_token] token: MoaOrDctTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let payment = self.call_value().moa_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_4(&self) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let payment = self.call_value().moa_value();
        let token = self.call_value().moa_or_single_dct().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: MoaOrDctTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().single_dct().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: MoaOrDctTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDctTokenIdentifier> {
        let payment = self.call_value().single_dct();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_dct().amount;
        let token = self.call_value().single_dct().token_identifier;
        (payment, token).into()
    }
}
