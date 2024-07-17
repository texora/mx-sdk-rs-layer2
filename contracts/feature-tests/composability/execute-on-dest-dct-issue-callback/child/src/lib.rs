#![no_std]

dharitri_wasm::imports!();

const MOA_DECIMALS: usize = 18;

#[dharitri_wasm::contract]
pub trait Child {
    #[init]
    fn init(&self) {}

    #[payable("MOA")]
    #[endpoint(issueWrappedMoa)]
    fn issue_wrapped_moa(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().moa_value();
        self.send()
            .dct_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: MOA_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: false,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().dct_issue_callback())
            .call_and_exit()
    }

    // callbacks

    #[callback]
    fn dct_issue_callback(&self, #[call_result] _result: IgnoreValue) {
        let (token_identifier, _amount) = self.call_value().single_fungible_dct();
        self.wrapped_moa_token_identifier().set(&token_identifier);
    }

    // storage

    #[view(getWrappedMoaTokenIdentifier)]
    #[storage_mapper("wrappedMoaTokenIdentifier")]
    fn wrapped_moa_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;
}
