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
    pub name: String,
    pub date_of_birth: u64,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
}
//The Tamagochi program should accept the following messages:
//- Name - the program answers the name of the Tamagochi;
// - Age - the program answers about the age of the Tamagochi.

#[no_mangle]
extern "C" fn init() {
    let tname = String::from_utf8(msg::load_bytes().expect("Can't load init message"))
        .expect("Invalid message");
    let character = Tamagotchi {
        name: tname,
        date_of_birth: (block_timestamp()),
    };
    unsafe { TAMAGOCHI = Some(character) };
}

#[no_mangle]
extern "C" fn handle() {
    let inquery: TmgAction = msg::load().expect("Error in handling msg");
    let character = unsafe { TAMAGOCHI.get_or_insert(Default::default()) };

    debug!("Program was initialized with message {:?}", inquery);

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
