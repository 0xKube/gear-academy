#![no_std]
use gmeta::metawasm;
use gstd::{exec, prelude::*};
use tmg_io::Tamagotchi;

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

#[metawasm]
pub mod metafns {
    pub type State = Tamagotchi;

    pub fn current_state(state: State) -> TmgCurrentState {
        let fed = state.fed.saturating_sub(
            HUNGER_PER_BLOCK * ((exec::block_timestamp() - state.last_feed) / 1_000),
        );
        let happy = state.happy.saturating_sub(
            BOREDOM_PER_BLOCK * ((exec::block_timestamp() - state.last_play) / 1_000),
        );
        let rested = state.rested.saturating_sub(
            ENERGY_PER_BLOCK * ((exec::block_timestamp() - state.last_sleep) / 1_000),
        );
        let current_state = TmgCurrentState { fed, happy, rested };
        current_state
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub struct TmgCurrentState {
    fed: u64,
    happy: u64,
    rested: u64,
}
