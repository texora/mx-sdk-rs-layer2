#[test]
fn buy_all_tickets_different_accounts_go() {
    dharitri_wasm_debug::denali_go("denali/buy-all-tickets-different-accounts.scen.json");
}

#[test]
fn buy_more_tickets_than_allowed_go() {
    dharitri_wasm_debug::denali_go("denali/buy-more-tickets-than-allowed.scen.json");
}

#[test]
fn buy_ticket_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket.scen.json");
}

#[test]
fn buy_ticket_after_deadline_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-after-deadline.scen.json");
}

#[test]
fn buy_ticket_after_determined_winner_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-after-determined-winner.scen.json");
}

#[test]
fn buy_ticket_after_sold_out_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-after-sold-out.scen.json");
}

#[test]
fn buy_ticket_all_options_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-all-options.scen.json");
}

#[test]
fn buy_ticket_another_account_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-another-account.scen.json");
}

#[test]
fn buy_ticket_not_on_whitelist_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-not-on-whitelist.scen.json");
}

#[test]
fn buy_ticket_same_account_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-same-account.scen.json");
}

#[test]
fn buy_ticket_second_lottery_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-second-lottery.scen.json");
}

#[test]
fn buy_ticket_wrong_fee_go() {
    dharitri_wasm_debug::denali_go("denali/buy-ticket-wrong-fee.scen.json");
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc1_go() {
    dharitri_wasm_debug::denali_go(
        "denali/determine-winner-different-ticket-holders-winner-acc1.scen.json",
    );
}

#[test]
fn determine_winner_early_go() {
    dharitri_wasm_debug::denali_go("denali/determine-winner-early.scen.json");
}

#[test]
fn determine_winner_same_ticket_holder_go() {
    dharitri_wasm_debug::denali_go("denali/determine-winner-same-ticket-holder.scen.json");
}

// #[test]
// fn determine_winner_split_prize_pool_go() {
// 	dharitri_wasm_debug::denali_go("denali/determine-winner-split-prize-pool.scen.json");
// }

#[test]
fn lottery_init_go() {
    dharitri_wasm_debug::denali_go("denali/lottery-init.scen.json");
}

#[test]
fn start_after_announced_winner_go() {
    dharitri_wasm_debug::denali_go("denali/start-after-announced-winner.scen.json");
}

#[test]
fn start_all_options_bigger_whitelist_go() {
    dharitri_wasm_debug::denali_go("denali/start-all-options-bigger-whitelist.scen.json");
}

#[test]
fn start_alternative_function_name_go() {
    dharitri_wasm_debug::denali_go("denali/start-alternative-function-name.scen.json");
}

#[test]
fn start_fixed_deadline_go() {
    dharitri_wasm_debug::denali_go("denali/start-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_go() {
    dharitri_wasm_debug::denali_go("denali/start-limited-tickets.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_go() {
    dharitri_wasm_debug::denali_go("denali/start-limited-tickets-and-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_go() {
    dharitri_wasm_debug::denali_go(
        "denali/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json",
    );
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_go() {
    dharitri_wasm_debug::denali_go(
        "denali/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
    );
}

#[test]
fn start_second_lottery_go() {
    dharitri_wasm_debug::denali_go("denali/start-second-lottery.scen.json");
}

#[test]
fn start_with_all_options_go() {
    dharitri_wasm_debug::denali_go("denali/start-with-all-options.scen.json");
}

#[test]
fn start_with_no_options_go() {
    dharitri_wasm_debug::denali_go("denali/start-with-no-options.scen.json");
}
