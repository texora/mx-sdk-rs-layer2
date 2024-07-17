dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_moa(&self) -> MoaOrDctTokenIdentifier {
        MoaOrDctTokenIdentifier::moa()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, token_id: MoaOrDctTokenIdentifier) -> bool {
        token_id.is_valid()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_dct_identifier()
    }
}
