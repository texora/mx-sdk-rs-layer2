#![allow(unused_variables)] // for now

use crate::world_mock::BlockchainMock;

use crate::denali_system::model::Step;
use std::path::Path;

/// Runs denali test using the Rust infrastructure and the debug mode.
/// Uses a contract map to replace the references to the wasm bytecode
/// with the contracts running in debug mode.
pub fn denali_rs<P: AsRef<Path>>(relative_path: P, mut world: BlockchainMock) {
    let mut absolute_path = world.current_dir.clone();
    absolute_path.push(relative_path);
    parse_execute_denali_steps(absolute_path.as_ref(), &mut world);
}

fn parse_execute_denali_steps(steps_path: &Path, state: &mut BlockchainMock) {
    let scenario = crate::denali_system::parse_scenario(steps_path);

    for step in scenario.steps.into_iter() {
        match step {
            Step::ExternalSteps(external_steps_step) => {
                let parent_path = steps_path.parent().unwrap();
                let new_path = parent_path.join(external_steps_step.path);
                parse_execute_denali_steps(new_path.as_path(), state);
            },
            Step::SetState(set_state_step) => {
                state.denali_set_state(set_state_step);
            },
            Step::ScCall(sc_call_step) => {
                state.denali_sc_call(sc_call_step);
            },
            Step::ScQuery(sc_query_step) => {
                state.denali_sc_query(sc_query_step);
            },
            Step::ScDeploy(sc_deploy_step) => {
                state.denali_sc_deploy(sc_deploy_step);
            },
            Step::Transfer(transfer_step) => {
                state.denali_transfer(transfer_step);
            },
            Step::ValidatorReward(validator_reward_step) => {
                state.denali_validator_reward(validator_reward_step);
            },
            Step::CheckState(check_state_step) => {
                state.denali_check_state(check_state_step);
            },
            Step::DumpState(_) => {
                state.denali_dump_state();
            },
        }
    }
}
