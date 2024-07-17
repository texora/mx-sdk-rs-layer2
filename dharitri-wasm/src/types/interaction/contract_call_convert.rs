use core::marker::PhantomData;

use crate::{
    api::{
        CallTypeApi, DCT_MULTI_TRANSFER_FUNC_NAME, DCT_NFT_TRANSFER_FUNC_NAME,
        DCT_TRANSFER_FUNC_NAME,
    },
    contract_base::BlockchainWrapper,
    types::{BigUint, DctTokenPayment, ManagedVec},
};

use super::{
    contract_call_no_payment::ContractCallNoPayment, ContractCallWithMoa, ManagedArgBuffer,
};

impl<SA, OriginalResult> ContractCallWithMoa<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// If this is an DCT call, it converts it to a regular call to DCTTransfer.
    /// Async calls require this step, but not `transfer_dct_execute`.
    pub fn convert_to_dct_transfer_call(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> Self {
        match payments.len() {
            0 => self,
            1 => self.convert_to_single_transfer_dct_call(payments.get(0)),
            _ => self.convert_to_multi_transfer_dct_call(payments),
        }
    }

    pub(super) fn convert_to_single_transfer_dct_call(
        self,
        payment: DctTokenPayment<SA>,
    ) -> Self {
        if payment.token_nonce == 0 {
            // fungible DCT
            let mut new_arg_buffer = ManagedArgBuffer::new();
            new_arg_buffer.push_arg(&payment.token_identifier);
            new_arg_buffer.push_arg(&payment.amount);
            if !self.basic.endpoint_name.is_empty() {
                new_arg_buffer.push_arg(&self.basic.endpoint_name);
            }

            ContractCallWithMoa {
                basic: ContractCallNoPayment {
                    _phantom: PhantomData,
                    to: self.basic.to,
                    endpoint_name: DCT_TRANSFER_FUNC_NAME.into(),
                    arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                    explicit_gas_limit: self.basic.explicit_gas_limit,
                    _return_type: PhantomData,
                },
                moa_payment: BigUint::zero(),
            }
        } else {
            // NFT
            // `DCTNFTTransfer` takes 4 arguments:
            // arg0 - token identifier
            // arg1 - nonce
            // arg2 - quantity to transfer
            // arg3 - destination address
            let mut new_arg_buffer = ManagedArgBuffer::new();
            new_arg_buffer.push_arg(&payment.token_identifier);
            new_arg_buffer.push_arg(payment.token_nonce);
            new_arg_buffer.push_arg(&payment.amount);
            new_arg_buffer.push_arg(&self.basic.to);
            if !self.basic.endpoint_name.is_empty() {
                new_arg_buffer.push_arg(&self.basic.endpoint_name);
            }

            // nft transfer is sent to self, sender = receiver
            let recipient_addr = BlockchainWrapper::<SA>::new().get_sc_address();

            ContractCallWithMoa {
                basic: ContractCallNoPayment {
                    _phantom: PhantomData,
                    to: recipient_addr,
                    endpoint_name: DCT_NFT_TRANSFER_FUNC_NAME.into(),
                    arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                    explicit_gas_limit: self.basic.explicit_gas_limit,
                    _return_type: PhantomData,
                },
                moa_payment: BigUint::zero(),
            }
        }
    }

    fn convert_to_multi_transfer_dct_call(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> Self {
        let mut new_arg_buffer = ManagedArgBuffer::new();
        new_arg_buffer.push_arg(self.basic.to);
        new_arg_buffer.push_arg(payments.len());

        for payment in payments.into_iter() {
            new_arg_buffer.push_arg(payment.token_identifier);
            new_arg_buffer.push_arg(payment.token_nonce);
            new_arg_buffer.push_arg(payment.amount);
        }
        if !self.basic.endpoint_name.is_empty() {
            new_arg_buffer.push_arg(self.basic.endpoint_name);
        }

        // multi transfer is sent to self, sender = receiver
        let recipient_addr = BlockchainWrapper::<SA>::new().get_sc_address();

        ContractCallWithMoa {
            basic: ContractCallNoPayment {
                _phantom: PhantomData,
                to: recipient_addr,
                endpoint_name: DCT_MULTI_TRANSFER_FUNC_NAME.into(),
                arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                explicit_gas_limit: self.basic.explicit_gas_limit,
                _return_type: PhantomData,
            },
            moa_payment: BigUint::zero(),
        }
    }
}
