#![no_std]

dharitri_wasm::imports!();

const ZERO_ASCII: u8 = b'0';
const DASH: u8 = b'-';
const RAND_CHARS_LEN: usize = 6;

#[dharitri_wasm::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[payable("MOA")]
    #[endpoint(issue)]
    fn issue_fungible(
        &self,
        _token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
        _num_decimals: usize,
        _token_properties: MultiValueEncoded<MultiValue2<ManagedBuffer, bool>>,
    ) -> TokenIdentifier {
        let new_token_id = self.create_new_token_id(token_ticker);
        require!(new_token_id.is_valid_dct_identifier(), "Invalid token ID");

        if initial_supply > 0 {
            let caller = self.blockchain().get_caller();

            self.send()
                .dct_local_mint(&new_token_id, 0, &initial_supply);
            self.send()
                .transfer_dct_via_async_call(caller, new_token_id, 0, initial_supply);
        }

        new_token_id
    }

    #[payable("MOA")]
    #[endpoint(issueNonFungible)]
    fn issue_non_fungible(
        &self,
        _token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        _token_properties: MultiValueEncoded<MultiValue2<ManagedBuffer, bool>>,
    ) -> TokenIdentifier {
        self.create_new_token_id(token_ticker)
    }

    #[payable("MOA")]
    #[endpoint(issueSemiFungible)]
    fn issue_semi_fungible(
        &self,
        _token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        _token_properties: MultiValueEncoded<MultiValue2<ManagedBuffer, bool>>,
    ) -> TokenIdentifier {
        self.create_new_token_id(token_ticker)
    }

    #[payable("MOA")]
    #[endpoint(registerMetaDCT)]
    fn issue_meta_dct(
        &self,
        _token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        _num_decimals: usize,
        _token_properties: MultiValueEncoded<MultiValue2<ManagedBuffer, bool>>,
    ) -> TokenIdentifier {
        self.create_new_token_id(token_ticker)
    }

    #[endpoint(setSpecialRole)]
    fn set_special_roles(
        &self,
        _token_id: TokenIdentifier,
        _address: ManagedAddress,
        _roles: MultiValueEncoded<DctLocalRole>,
    ) {
    }

    #[payable("MOA")]
    #[endpoint(registerAndSetAllRoles)]
    fn register_and_set_all_roles(
        &self,
        _token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        _token_type_name: ManagedBuffer,
        _num_decimals: usize,
    ) -> TokenIdentifier {
        self.create_new_token_id(token_ticker)
    }

    fn create_new_token_id(&self, token_ticker: ManagedBuffer) -> TokenIdentifier {
        let nr_issued_tokens = self.nr_issued_tokens().get();
        let mut rand_chars = [ZERO_ASCII; RAND_CHARS_LEN];
        for c in &mut rand_chars {
            *c += nr_issued_tokens;
        }

        self.nr_issued_tokens().update(|nr| *nr += 1);

        let mut token_id = token_ticker;
        token_id.append_bytes(&[DASH][..]);
        token_id.append_bytes(&rand_chars);

        token_id.into()
    }

    #[storage_mapper("nrIssuedTokens")]
    fn nr_issued_tokens(&self) -> SingleValueMapper<u8>;
}
