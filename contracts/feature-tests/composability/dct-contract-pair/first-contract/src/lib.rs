#![no_std]

dharitri_wasm::imports!();

const DCT_TRANSFER_STRING: &[u8] = b"DCTTransfer";
const SECOND_CONTRACT_ACCEPT_DCT_PAYMENT: &[u8] = b"acceptDctPayment";
const SECOND_CONTRACT_REJECT_DCT_PAYMENT: &[u8] = b"rejectDctPayment";

#[dharitri_wasm::contract]
pub trait FirstContract {
    #[init]
    fn init(
        &self,
        dct_token_identifier: TokenIdentifier,
        second_contract_address: ManagedAddress,
    ) {
        self.set_contract_dct_token_identifier(&dct_token_identifier);
        self.set_second_contract_address(&second_contract_address);
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFull)]
    fn transfer_to_second_contract_full(&self) {
        let (actual_token_identifier, dct_value) = self.call_value().single_fungible_dct();
        let expected_token_identifier = self.get_contract_dct_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dct token"
        );

        self.call_dct_second_contract(
            &expected_token_identifier,
            &dct_value,
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_DCT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractHalf)]
    fn transfer_to_second_contract_half(&self) {
        let (actual_token_identifier, dct_value) = self.call_value().single_fungible_dct();
        let expected_token_identifier = self.get_contract_dct_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dct token"
        );

        self.call_dct_second_contract(
            &expected_token_identifier,
            &(dct_value / 2u32),
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_DCT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejected)]
    fn transfer_to_second_contract_rejected(&self) {
        let (actual_token_identifier, dct_value) = self.call_value().single_fungible_dct();
        let expected_token_identifier = self.get_contract_dct_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dct token"
        );

        self.call_dct_second_contract(
            &expected_token_identifier,
            &dct_value,
            &self.get_second_contract_address(),
            &ManagedBuffer::from(SECOND_CONTRACT_REJECT_DCT_PAYMENT),
            &ManagedVec::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejectedWithTransferAndExecute)]
    fn transfer_to_second_contract_rejected_with_transfer_and_execute(&self) {
        let (actual_token_identifier, dct_value) = self.call_value().single_fungible_dct();
        let second_contract_address = self.get_second_contract_address();
        let expected_token_identifier = self.get_contract_dct_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dct token"
        );

        let _ = self.send_raw().transfer_dct_execute(
            &second_contract_address,
            &expected_token_identifier,
            &dct_value,
            self.blockchain().get_gas_left(),
            &ManagedBuffer::from(SECOND_CONTRACT_REJECT_DCT_PAYMENT),
            &ManagedArgBuffer::new(),
        );
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFullWithTransferAndExecute)]
    fn transfer_to_second_contract_full_with_transfer_and_execute(&self) {
        let (actual_token_identifier, dct_value) = self.call_value().single_fungible_dct();
        let second_contract_address = self.get_second_contract_address();
        let expected_token_identifier = self.get_contract_dct_token_identifier();

        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dct token"
        );

        let _ = self.send_raw().transfer_dct_execute(
            &second_contract_address,
            &expected_token_identifier,
            &dct_value,
            self.blockchain().get_gas_left(),
            &ManagedBuffer::from(SECOND_CONTRACT_ACCEPT_DCT_PAYMENT),
            &ManagedArgBuffer::new(),
        );
    }

    fn call_dct_second_contract(
        &self,
        dct_token_identifier: &TokenIdentifier,
        amount: &BigUint,
        to: &ManagedAddress,
        func_name: &ManagedBuffer,
        args: &ManagedVec<Self::Api, ManagedBuffer>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(dct_token_identifier);
        arg_buffer.push_arg(amount);
        arg_buffer.push_arg(func_name);
        for arg in args.into_iter() {
            arg_buffer.push_arg_raw(arg);
        }

        self.send_raw().async_call_raw(
            to,
            &BigUint::zero(),
            &ManagedBuffer::from(DCT_TRANSFER_STRING),
            &arg_buffer,
        );
    }

    // storage

    #[storage_set("dctTokenName")]
    fn set_contract_dct_token_identifier(&self, dct_token_identifier: &TokenIdentifier);

    #[view(getdctTokenName)]
    #[storage_get("dctTokenName")]
    fn get_contract_dct_token_identifier(&self) -> TokenIdentifier;

    #[storage_set("secondContractAddress")]
    fn set_second_contract_address(&self, address: &ManagedAddress);

    #[view(getSecondContractAddress)]
    #[storage_get("secondContractAddress")]
    fn get_second_contract_address(&self) -> ManagedAddress;
}
