use crowdfunding_dct::*;
use dharitri_wasm::types::MoaOrDctTokenIdentifier;
use dharitri_wasm_debug::{denali_system::model::*, *};

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dct");

    blockchain.register_contract(
        "file:output/crowdfunding-dct.wasm",
        crowdfunding_dct::ContractBuilder,
    );
    blockchain
}

#[test]
fn crowdfunding_denali_rust_test() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ctx = world.interpreter_context();

    let owner_addr = "address:owner";
    let first_user_addr = "address:user1";
    let second_user_addr = "address:user2";

    let deadline: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
    let cf_token_id_value = "CROWD-123456"; // when passing as argument
    let cf_token_id = "str:CROWD-123456"; // when specifying the token transfer
    let mut cf_sc = ContractInfo::<crowdfunding_dct::Proxy<DebugApi>>::new("sc:crowdfunding");

    // setup owner and crowdfunding SC
    world.denali_set_state(
        SetStateStep::new()
            .put_account(owner_addr, Account::new())
            .new_address(owner_addr, 0, &cf_sc),
    );
    let (_, ()) = cf_sc
        .init(
            2_000u32,
            deadline,
            MoaOrDctTokenIdentifier::dct(cf_token_id_value),
        )
        .into_blockchain_call()
        .from(owner_addr)
        .contract_code("file:output/crowdfunding-dct.wasm", &ctx)
        .gas_limit("5,000,000")
        .expect(TxExpect::ok().no_result())
        .execute(&mut world);

    // setup user accounts
    world
        .denali_set_state(SetStateStep::new().put_account(
            first_user_addr,
            Account::new().dct_balance(cf_token_id, 1_000u64),
        ))
        .denali_set_state(SetStateStep::new().put_account(
            second_user_addr,
            Account::new().dct_balance(cf_token_id, 1_000u64),
        ));

    // first user deposit
    world
        .denali_sc_call(
            ScCallStep::new()
                .from(first_user_addr)
                .to(&cf_sc)
                .dct_transfer(cf_token_id, 0u64, 1_000u64)
                .call(cf_sc.fund())
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(
                    first_user_addr,
                    CheckAccount::new().dct_balance(cf_token_id, 0u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().dct_balance(cf_token_id, 1_000u64),
                ),
        );

    // second user deposit
    world
        .denali_sc_call(
            ScCallStep::new()
                .from(second_user_addr)
                .to(&cf_sc)
                .dct_transfer(cf_token_id, 0u64, 500u64)
                .call(cf_sc.fund())
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(
                    second_user_addr,
                    CheckAccount::new().dct_balance(cf_token_id, 500u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().dct_balance(cf_token_id, 1_500u64),
                ),
        );

    // get status before
    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result(""))
        .execute(&mut world);
    assert_eq!(status, Status::FundingPeriod);

    // deadline passed
    world.denali_set_state(SetStateStep::new().block_timestamp(deadline));

    // get status after deadline
    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result("2"))
        .execute(&mut world);
    assert_eq!(status, Status::Failed);

    // test failed campaign

    // owner claim - failed campaign - nothing is transferred
    world
        .denali_sc_call(
            ScCallStep::new()
                .from(owner_addr)
                .to(&cf_sc)
                .call(cf_sc.claim())
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(
                    owner_addr,
                    CheckAccount::new().dct_balance(cf_token_id, 0u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().dct_balance(cf_token_id, 1_500u64),
                ),
        );

    // first user claim - failed campaign
    world
        .denali_sc_call(
            ScCallStep::new()
                .from(first_user_addr)
                .to(&cf_sc)
                .call(cf_sc.claim())
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(
                    first_user_addr,
                    CheckAccount::new().dct_balance(cf_token_id, 1_000u64),
                )
                .put_account(
                    &cf_sc,
                    CheckAccount::new().dct_balance(cf_token_id, 500u64),
                ),
        );

    // second user claim - failed campaign
    world
        .denali_sc_call(
            ScCallStep::new()
                .from(second_user_addr)
                .to(&cf_sc)
                .call(cf_sc.claim())
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(
                    second_user_addr,
                    CheckAccount::new().dct_balance(cf_token_id, 1_000u64),
                )
                .put_account(&cf_sc, CheckAccount::new().dct_balance(cf_token_id, 0u64)),
        );

    // test successful campaign

    world.denali_set_state(SetStateStep::new().block_timestamp(deadline / 2));

    // first user deposit
    world.denali_sc_call(
        ScCallStep::new()
            .from(first_user_addr)
            .to(&cf_sc)
            .dct_transfer(cf_token_id, 0u64, 1_000u64)
            .call(cf_sc.fund())
            .expect(TxExpect::ok().no_result()),
    );

    // second user deposit
    world.denali_sc_call(
        ScCallStep::new()
            .from(second_user_addr)
            .to(&cf_sc)
            .dct_transfer(cf_token_id, 0u64, 1_000u64)
            .call(cf_sc.fund())
            .expect(TxExpect::ok().no_result()),
    );

    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result(""))
        .execute(&mut world);
    assert_eq!(status, Status::FundingPeriod);

    world.denali_set_state(SetStateStep::new().block_timestamp(deadline));

    let status: Status = cf_sc
        .status()
        .into_vm_query()
        .expect(TxExpect::ok().result("1"))
        .execute(&mut world);
    assert_eq!(status, Status::Successful);

    // first user try claim - successful campaign
    world.denali_sc_call(
        ScCallStep::new()
            .from(first_user_addr)
            .to(&cf_sc)
            .call(cf_sc.claim())
            .expect(TxExpect::err(
                4,
                "str:only owner can claim successful funding",
            )),
    );

    // owner claim successful campaign
    world
        .denali_sc_call(
            ScCallStep::new()
                .from(owner_addr)
                .to(&cf_sc)
                .call(cf_sc.claim())
                .expect(TxExpect::ok().no_result()),
        )
        .denali_check_state(
            CheckStateStep::new()
                .put_account(
                    owner_addr,
                    CheckAccount::new().dct_balance(cf_token_id, 2_000u64),
                )
                .put_account(cf_sc, CheckAccount::new().dct_balance(cf_token_id, 0u64)),
        );

    world.write_denali_trace("denali-gen/crowdfunding_rust.scen.json");
}
