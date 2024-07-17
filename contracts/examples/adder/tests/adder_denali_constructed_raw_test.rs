use dharitri_wasm_debug::{denali_system::model::*, *};

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_denali_constructed_raw() {
    let mut world = world();
    let ic = world.interpreter_context();
    world
        .denali_set_state(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:adder"),
        )
        .denali_sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .contract_code("file:output/adder.wasm", &ic)
                .argument("5")
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .denali_sc_query(
            ScQueryStep::new()
                .to("sc:adder")
                .function("getSum")
                .expect(TxExpect::ok().result("5")),
        )
        .denali_sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:adder")
                .function("add")
                .argument("3")
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:adder",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}
