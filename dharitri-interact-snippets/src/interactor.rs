use dharitri_sdk_moars::{
    blockchain::rpc::DharitriProxy,
    data::{address::Address as MoarsAddress, network_config::NetworkConfig},
    interactors::wallet::Wallet,
};
use dharitri_wasm_debug::{dharitri_wasm::types::Address, denali_system::model::AddressValue, HashMap};
use std::time::Duration;

use crate::Sender;

pub struct Interactor {
    pub proxy: DharitriProxy,
    pub network_config: NetworkConfig,
    pub sender_map: HashMap<Address, Sender>,

    pub(crate) waiting_time_ms: u64,
}

impl Interactor {
    pub async fn new(gateway_url: &str) -> Self {
        let proxy = DharitriProxy::new(gateway_url.to_string());
        let network_config = proxy.get_network_config().await.unwrap();
        Self {
            proxy,
            network_config,
            sender_map: HashMap::new(),
            waiting_time_ms: 0,
        }
    }

    pub fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let address = moars_address_to_h256(wallet.address());
        self.sender_map.insert(
            address.clone(),
            Sender {
                address: address.clone(),
                wallet,
                current_nonce: None,
            },
        );
        address
    }

    pub async fn sleep(&mut self, duration: Duration) {
        self.waiting_time_ms += duration.as_millis() as u64;
        tokio::time::sleep(duration).await;
    }
}

pub(crate) fn denali_to_moars_address(denali_address: &AddressValue) -> MoarsAddress {
    let bytes = denali_address.value.as_array();
    MoarsAddress::from_bytes(*bytes)
}

pub(crate) fn address_h256_to_moars(address: &Address) -> MoarsAddress {
    let bytes = address.as_array();
    MoarsAddress::from_bytes(*bytes)
}

pub(crate) fn moars_address_to_h256(moars_address: MoarsAddress) -> Address {
    moars_address.to_bytes().into()
}
