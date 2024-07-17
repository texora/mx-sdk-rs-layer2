use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/erc-style-contracts/erc20");

    blockchain.register_contract("file:output/erc20.wasm", erc20::ContractBuilder);
    blockchain
}

#[test]
fn allowance_callercaller_rs() {
    dharitri_wasm_debug::denali_rs("denali/allowance_CallerCaller.scen.json", world());
}

#[test]
fn allowance_callerother_rs() {
    dharitri_wasm_debug::denali_rs("denali/allowance_CallerOther.scen.json", world());
}

#[test]
fn allowance_othercaller_rs() {
    dharitri_wasm_debug::denali_rs("denali/allowance_OtherCaller.scen.json", world());
}

#[test]
fn allowance_othereqother_rs() {
    dharitri_wasm_debug::denali_rs("denali/allowance_OtherEqOther.scen.json", world());
}

#[test]
fn allowance_otherneqother_rs() {
    dharitri_wasm_debug::denali_rs("denali/allowance_OtherNEqOther.scen.json", world());
}

#[test]
fn approve_caller_positive_rs() {
    dharitri_wasm_debug::denali_rs("denali/approve_Caller-Positive.scen.json", world());
}

#[test]
fn approve_caller_zero_rs() {
    dharitri_wasm_debug::denali_rs("denali/approve_Caller-Zero.scen.json", world());
}

#[test]
fn approve_other_positive_rs() {
    dharitri_wasm_debug::denali_rs("denali/approve_Other-Positive.scen.json", world());
}

#[test]
fn approve_other_zero_rs() {
    dharitri_wasm_debug::denali_rs("denali/approve_Other-Zero.scen.json", world());
}

#[test]
fn approve_switchcaller_rs() {
    dharitri_wasm_debug::denali_rs("denali/approve_SwitchCaller.scen.json", world());
}

#[test]
fn balanceof_caller_rs() {
    dharitri_wasm_debug::denali_rs("denali/balanceOf_Caller.scen.json", world());
}

#[test]
fn balanceof_noncaller_rs() {
    dharitri_wasm_debug::denali_rs("denali/balanceOf_NonCaller.scen.json", world());
}

#[test]
fn not_payable_rs() {
    dharitri_wasm_debug::denali_rs("denali/not_payable.scen.json", world());
}

#[test]
fn totalsupply_positive_rs() {
    dharitri_wasm_debug::denali_rs("denali/totalSupply_Positive.scen.json", world());
}

#[test]
fn totalsupply_zero_rs() {
    dharitri_wasm_debug::denali_rs("denali/totalSupply_Zero.scen.json", world());
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-BalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_nooverflow_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-NoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_alldistinct_stillnooverflow_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllDistinct-StillNoOverflow.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_allequal_allowancerelevant_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllEqual-AllowanceRelevant.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_allequal_entirebalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_AllEqual-EntireBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqfrom_entirebalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_CallerEqFrom-EntireBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqfrom_morethanbalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_CallerEqFrom-MoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqto_balanceneqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-BalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
        world(),
    );
}

#[test]
fn transferfrom_fromeqto_nooverflow_rs() {
    dharitri_wasm_debug::denali_rs("denali/transferFrom_FromEqTo-NoOverflow.scen.json", world());
}

#[test]
fn transfer_caller_allowanceirrelevant_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transfer_Caller-AllowanceIrrelevant.scen.json",
        world(),
    );
}

#[test]
fn transfer_caller_entirebalance_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Caller-EntireBalance.scen.json", world());
}

#[test]
fn transfer_caller_morethanbalance_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Caller-MoreThanBalance.scen.json", world());
}

#[test]
fn transfer_caller_nooverflow_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Caller-NoOverflow.scen.json", world());
}

#[test]
fn transfer_caller_positive_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Caller-Positive.scen.json", world());
}

#[test]
fn transfer_caller_stillnooverflow_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Caller-StillNoOverflow.scen.json", world());
}

#[test]
fn transfer_caller_zero_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Caller-Zero.scen.json", world());
}

#[test]
fn transfer_other_allowanceirrelevant_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/transfer_Other-AllowanceIrrelevant.scen.json",
        world(),
    );
}

#[test]
fn transfer_other_entirebalance_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Other-EntireBalance.scen.json", world());
}

#[test]
fn transfer_other_morethanbalance_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Other-MoreThanBalance.scen.json", world());
}

#[test]
fn transfer_other_nooverflow_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Other-NoOverflow.scen.json", world());
}

#[test]
fn transfer_other_positive_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Other-Positive.scen.json", world());
}

#[test]
fn transfer_other_stillnooverflow_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Other-StillNoOverflow.scen.json", world());
}

#[test]
fn transfer_other_zero_rs() {
    dharitri_wasm_debug::denali_rs("denali/transfer_Other-Zero.scen.json", world());
}
