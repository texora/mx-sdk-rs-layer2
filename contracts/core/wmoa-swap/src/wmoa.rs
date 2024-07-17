#![no_std]

dharitri_wasm::imports!();

#[dharitri_wasm::contract]
pub trait MoaDctSwap: dharitri_wasm_modules::pause::PauseModule {
    #[init]
    fn init(&self, wrapped_moa_token_id: TokenIdentifier) {
        self.wrapped_moa_token_id().set(&wrapped_moa_token_id);
    }

    // endpoints

    #[payable("MOA")]
    #[endpoint(wrapMoa)]
    fn wrap_moa(&self) -> DctTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().moa_value();
        require!(payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_moa_token_id = self.wrapped_moa_token_id().get();
        self.send()
            .dct_local_mint(&wrapped_moa_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_dct(&caller, &wrapped_moa_token_id, 0, &payment_amount);

        DctTokenPayment::new(wrapped_moa_token_id, 0, payment_amount)
    }

    #[payable("*")]
    #[endpoint(unwrapMoa)]
    fn unwrap_moa(&self) {
        self.require_not_paused();

        let (payment_token, payment_amount) = self.call_value().single_fungible_dct();
        let wrapped_moa_token_id = self.wrapped_moa_token_id().get();

        require!(payment_token == wrapped_moa_token_id, "Wrong dct token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        require!(
            payment_amount <= self.get_locked_moa_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .dct_local_burn(&wrapped_moa_token_id, 0, &payment_amount);

        // 1 wrapped MOA = 1 MOA, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_moa(&caller, &payment_amount);
    }

    #[view(getLockedMoaBalance)]
    fn get_locked_moa_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&MoaOrDctTokenIdentifier::moa(), 0)
    }

    #[view(getWrappedMoaTokenId)]
    #[storage_mapper("wrappedMoaTokenId")]
    fn wrapped_moa_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
