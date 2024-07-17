#![no_std]
#![allow(clippy::type_complexity)]
#![allow(clippy::let_unit_value)]

pub mod call_async;
pub mod call_queue;
pub mod call_sync;
pub mod call_transf_exec;
pub mod contract_change_owner;
pub mod contract_deploy;
pub mod contract_upgrade;
pub mod dct;
pub mod nft;
pub mod roles;
pub mod sft;
pub mod storage;

dharitri_wasm::imports!();

/// Test contract for investigating contract calls.
#[dharitri_wasm::contract]
pub trait Forwarder:
    call_sync::ForwarderSyncCallModule
    + call_async::ForwarderAsyncCallModule
    + call_transf_exec::ForwarderTransferExecuteModule
    + call_queue::ForwarderQueuedCallModule
    + contract_change_owner::ChangeOwnerModule
    + contract_deploy::DeployContractModule
    + contract_upgrade::UpgradeContractModule
    + dct::ForwarderDctModule
    + sft::ForwarderSftModule
    + nft::ForwarderNftModule
    + roles::ForwarderRolesModule
    + storage::ForwarderStorageModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn send_moa(&self, to: &ManagedAddress, amount: &BigUint) {
        self.send().direct_moa(to, amount);
    }
}
