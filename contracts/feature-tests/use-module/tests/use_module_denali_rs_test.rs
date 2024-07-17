mod user_builtin {
    dharitri_wasm::imports!();

    #[dharitri_wasm::proxy]
    pub trait UserBuiltin {
        #[endpoint(SetUserName)]
        fn set_user_name(&self, name: &BoxedBytes) -> BigUint;
    }
}

mod dns_mock {
    dharitri_wasm::imports!();

    #[dharitri_wasm::contract]
    pub trait DnsMock {
        #[proxy]
        fn user_builtin_proxy(&self, to: ManagedAddress) -> super::user_builtin::Proxy<Self::Api>;

        #[payable("MOA")]
        #[endpoint]
        fn register(&self, name: BoxedBytes) {
            let _payment = self.call_value().moa_value();
            let address = self.blockchain().get_caller();
            self.user_builtin_proxy(address)
                .set_user_name(&name)
                .async_call()
                .call_and_exit()
        }
    }
}

use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract("file:output/use-module.wasm", use_module::ContractBuilder);

    blockchain.register_contract(
        "file:test-wasm/dharitri-wasm-sc-dns.wasm",
        dns_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn use_module_claim_developer_rewards_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/use_module_claim_developer_rewards.scen.json",
        world(),
    );
}

#[test]
fn use_module_dns_register_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_dns_register.scen.json", world());
}

#[test]
fn use_module_features_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_features.scen.json", world());
}

#[test]
fn use_module_internal_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_internal.scen.json", world());
}

#[test]
fn use_module_only_owner_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_only_owner.scen.json", world());
}

#[test]
fn use_module_only_admin_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_only_admin.scen.json", world());
}

#[test]
fn use_module_no_endpoint_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_no_endpoint.scen.json", world());
}

#[test]
fn use_module_pause_rs() {
    dharitri_wasm_debug::denali_rs("denali/use_module_pause.scen.json", world());
}

// Will not work in denali-rs, since there is no gas usage
// #[test]
// fn use_module_ongoing_operation_rs() {
//     dharitri_wasm_debug::denali_rs(
//         "denali/use_module_ongoing_operation_example.scen.json",
//         world(),
//     );
// }
