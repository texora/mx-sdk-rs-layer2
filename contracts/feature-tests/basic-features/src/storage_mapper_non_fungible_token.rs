dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[dharitri_wasm::module]
pub trait NonFungibleTokenMapperFeatures:
    dharitri_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[payable("MOA")]
    #[endpoint]
    fn issue_and_set_all_roles_meta(&self, token_ticker: ManagedBuffer) {
        let payment = self.call_value().moa_value();
        self.non_fungible_token_mapper().issue_and_set_all_roles(
            DctTokenType::Meta,
            payment,
            ManagedBuffer::new(),
            token_ticker,
            0,
            None,
        );
    }

    #[endpoint]
    fn mapper_nft_set_token_id(&self, token_id: TokenIdentifier) {
        self.non_fungible_token_mapper().set_token_id(token_id);
    }

    #[endpoint]
    fn mapper_nft_create(
        &self,
        amount: BigUint,
        attributes: RgbColor,
    ) -> DctTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_create(amount, &attributes)
    }

    #[endpoint]
    fn mapper_nft_create_and_send(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        attributes: RgbColor,
    ) -> DctTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_create_and_send(&to, amount, &attributes)
    }

    #[endpoint]
    fn mapper_nft_add_quantity(
        &self,
        token_nonce: u64,
        amount: BigUint,
    ) -> DctTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_add_quantity(token_nonce, amount)
    }

    #[endpoint]
    fn mapper_nft_add_quantity_and_send(
        &self,
        to: ManagedAddress,
        token_nonce: u64,
        amount: BigUint,
    ) -> DctTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_add_quantity_and_send(&to, token_nonce, amount)
    }

    #[endpoint]
    fn mapper_nft_burn(&self, token_nonce: u64, amount: BigUint) {
        self.non_fungible_token_mapper()
            .nft_burn(token_nonce, &amount);
    }

    #[endpoint]
    fn mapper_nft_get_balance(&self, token_nonce: u64) -> BigUint {
        self.non_fungible_token_mapper().get_balance(token_nonce)
    }

    #[endpoint]
    fn mapper_get_token_attributes(&self, token_nonce: u64) -> RgbColor {
        self.non_fungible_token_mapper()
            .get_token_attributes(token_nonce)
    }

    #[view(getNonFungibleTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn non_fungible_token_mapper(&self) -> NonFungibleTokenMapper;
}
