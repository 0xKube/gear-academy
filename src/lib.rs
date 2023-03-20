#![no_std]

use gstd::{
    debug,
    exec::{self, block_timestamp},
    msg,
    prelude::*,
    ActorId,
};
static mut GREETING: Option<String> = None;

#[derive(Encode, Decode, TypeInfo)]
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
    let greeting = String::from_utf8(msg::load_bytes().expect("Can't load init message"))
        .expect("Invalid message");
    debug!("Program was initialized with message {:?}", greeting);
    unsafe { GREETING = Some(greeting) };
}

#[no_mangle]
extern "C" fn handle() {
    let inquery: TmgAction = msg::load().expect("Error in handling msg");
    debug!("Program was initialized with message {:?}", inquery);

    let character = Tamagotchi {
        name: "Valera".to_owned(),
        date_of_birth: (block_timestamp()),
    };

    match inquery {
        TmgAction::Age => {
            msg::reply(TmgEvent::Age(character.date_of_birth), 0)
                .expect("Error in sending Hello message to account");
        }
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(character.name), 0)
                .expect("Error in sending Hello message to account");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let greeting = unsafe { GREETING.as_ref().expect("The contract is not initialized") };
    msg::reply(greeting, 0).expect("Failed to share state");
}

#[no_mangle]
// It returns the Hash of metadata.
// .metahash is generating automatically while you are using build.rs
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
