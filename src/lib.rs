#![no_std]

use gstd::{
    debug,
    exec::{self, block_timestamp},
    msg,
    prelude::*,
    ActorId,
};
static mut TAMAGOCHI: Option<Tamagotchi> = None;

#[derive(Encode, Decode, TypeInfo, Default)]
pub struct Tamagotchi {
    pub owner: ActorId,
    pub name: String,
    pub date_of_birth: u64,
    pub fed: u32,
    pub happy: u32,
    pub rested: u32,
    pub last_sleep: u64,
    pub last_feed: u64,
    pub last_play: u64,
    pub approved_account: Option<ActorId>,
}

const HUNGER_PER_BLOCK: u32 = 1;
const ENERGY_PER_BLOCK: u32 = 2;
const BOREDOM_PER_BLOCK: u32 = 2;
const FILL_PER_SLEEP: u32 = 1000;
const FILL_PER_FEED: u32 = 1000;
const FILL_PER_ENTERTAINMENT: u32 = 1000;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
    Sleep,
    Feed,
    Play,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
}
//The Tamagochi program should accept the following messages:
//- Name - the program answers the name of the Tamagochi;
// - Age - the program answers about the age of the Tamagochi.

impl Tamagotchi {
    fn update_states(&mut self) {
        let current_block = block_timestamp();
        let blocks_since_last_sleep = current_block - self.last_sleep;
        let blocks_since_last_feed = current_block - self.last_feed;
        let blocks_since_last_play = current_block - self.last_play;

        let hunger_increase = blocks_since_last_feed * HUNGER_PER_BLOCK as u64;
        let energy_decrease = blocks_since_last_sleep * ENERGY_PER_BLOCK as u64;
        let boredom_increase = blocks_since_last_play * BOREDOM_PER_BLOCK as u64;

        self.fed = self.fed.saturating_sub(hunger_increase as u32);
        self.rested = self.rested.saturating_sub(energy_decrease as u32);
        self.happy = self.happy.saturating_sub(boredom_increase as u32);
    }
}

#[no_mangle]
extern "C" fn init() {
    let tname = String::from_utf8(msg::load_bytes().expect("Can't load init message"))
        .expect("Invalid message");
    let character = Tamagotchi {
        owner: msg::source(),
        name: tname,
        date_of_birth: block_timestamp(),
        fed: FILL_PER_FEED,
        happy: FILL_PER_ENTERTAINMENT,
        rested: FILL_PER_SLEEP,
        last_sleep: block_timestamp(),
        last_feed: block_timestamp(),
        last_play: block_timestamp(),
        approved_account: None,
    };
    unsafe { TAMAGOCHI = Some(character) };
}

#[no_mangle]
extern "C" fn handle() {
    let inquery: TmgAction = msg::load().expect("Error in handling msg");
    let character: &mut Tamagotchi =
        unsafe { TAMAGOCHI.as_mut().expect("The contract is not initialized") };

    debug!("Program was initialized with message {:?}", inquery);

    character.update_states();

    match inquery {
        TmgAction::Age => {
            msg::reply(
                TmgEvent::Age(block_timestamp() - character.date_of_birth),
                0,
            )
            .expect("Error in sending Hello message to account");
        }
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(character.name.to_string()), 0)
                .expect("Error in sending Hello message to account");
        }
        TmgAction::Sleep => {
            character.rested = (character.rested + FILL_PER_SLEEP).min(10000);
            character.last_sleep = block_timestamp();
        }
        TmgAction::Feed => {
            character.fed = (character.fed + FILL_PER_FEED).min(10000);
            character.last_feed = block_timestamp();
        }
        TmgAction::Play => {
            character.happy = (character.happy + FILL_PER_ENTERTAINMENT).min(10000);
            character.last_play = block_timestamp();
        }
        TmgAction::Transfer(new_owner) => {
            if character.owner == msg::source() || character.approved_account == Some(msg::source())
            {
                character.owner = new_owner;
                character.approved_account = None;
            } else {
                panic!("Only the owner or an approved account can transfer ownership");
            }
        }
        TmgAction::Approve(allowed_account) => {
            if character.owner == msg::source() {
                character.approved_account = Some(allowed_account);
            } else {
                panic!("Only the owner can approve an account");
            }
        }
        TmgAction::RevokeApproval => {
            if character.owner == msg::source() {
                character.approved_account = None;
            } else {
                panic!("Only the owner can revoke approval");
            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = unsafe { TAMAGOCHI.as_ref().expect("The contract is not initialized") };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}

#[no_mangle]
// It returns the Hash of metadata.
// .metahash is generating automatically while you are using build.rs
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
