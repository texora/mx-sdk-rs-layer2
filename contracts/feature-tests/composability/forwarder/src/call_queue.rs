dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub enum QueuedCallType {
    Sync,
    LegacyAsync,
    TransferExecute,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct QueuedCall<M: ManagedTypeApi> {
    call_type: QueuedCallType,
    to: ManagedAddress<M>,
    payment_token: MoaOrDctTokenIdentifier<M>,
    payment_nonce: u64,
    payment_amount: BigUint<M>,
}

/// Testing multiple calls per transaction, cascading on.
///
/// TODO: write actual tests with these.
#[dharitri_wasm::module]
pub trait ForwarderQueuedCallModule {
    #[proxy]
    fn self_proxy(&self, to: ManagedAddress) -> crate::Proxy<Self::Api>;

    #[view]
    #[storage_mapper("queued_calls")]
    fn queued_calls(&self) -> LinkedListMapper<QueuedCall<Self::Api>>;

    #[endpoint]
    fn add_queued_call(
        &self,
        call_type: QueuedCallType,
        to: ManagedAddress,
        payment_token: MoaOrDctTokenIdentifier,
        payment_nonce: u64,
        payment_amount: BigUint,
    ) {
        self.queued_calls().push_back(QueuedCall {
            call_type,
            to,
            payment_token,
            payment_nonce,
            payment_amount,
        });
    }

    #[endpoint]
    #[payable("*")]
    fn forward_queued_calls(&self, max_call_depth: usize) {
        let dct_transfers_multi = self.call_value().all_dct_transfers();
        self.forward_queued_calls_event(
            max_call_depth,
            &self.call_value().moa_value(),
            &dct_transfers_multi.into_multi_value(),
        );

        if max_call_depth == 0 {
            return;
        }

        while let Some(node) = self.queued_calls().pop_front() {
            let call = node.into_value();
            let contract_call = self
                .self_proxy(call.to)
                .forward_queued_calls(max_call_depth - 1)
                .with_moa_or_single_dct_transfer((
                    call.payment_token,
                    call.payment_nonce,
                    call.payment_amount,
                ));
            match call.call_type {
                QueuedCallType::Sync => {
                    contract_call.execute_on_dest_context::<()>();
                },
                QueuedCallType::LegacyAsync => {
                    contract_call.async_call().call_and_exit();
                },
                QueuedCallType::TransferExecute => {
                    contract_call.transfer_execute();
                },
            }
        }
    }

    #[event("forward_queued_calls")]
    fn forward_queued_calls_event(
        &self,
        #[indexed] max_call_depth: usize,
        #[indexed] moa_value: &BigUint,
        #[indexed] multi_dct: &MultiValueEncoded<DctTokenPaymentMultiValue>,
    );
}
