use adder::*;
use dharitri_wasm::storage::mappers::SingleValue;
use dharitri_wasm_debug::{denali_system::model::*, num_bigint::BigUint, *};

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_denali_constructed_raw() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();
    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<DebugApi>>::new("sc:adder");

    world
        .denali_set_state(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:adder"),
        )
        .denali_sc_deploy(
            ScDeployStep::new()
                .from(owner_address)
                .contract_code("file:output/adder.wasm", &ic)
                .call(adder_contract.init(5u32))
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .denali_sc_query(
            ScQueryStep::new()
                .to(&adder_contract)
                .call_expect(adder_contract.sum(), SingleValue::from(BigUint::from(5u32))),
        )
        .denali_sc_call(
            ScCallStep::new()
                .from(owner_address)
                .to(&adder_contract)
                .call(adder_contract.add(3u32))
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(
                    &adder_contract,
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        )
        .write_denali_trace("trace1.scen.json");
}
