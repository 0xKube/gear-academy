#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

pub type AttributeId = u32;
pub type TransactionId = u64;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<String, ()>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Play,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    Fed,
    Happy,
    Slept,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    ApproveTokens { account: ActorId, amount: u128 },
    ApprovalError,
    SetFTokenContract,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub owner: ActorId,
    pub name: String,
    pub date_of_birth: u64,
    pub fed: u64,
    pub happy: u64,
    pub rested: u64,
    pub last_sleep: u64,
    pub last_feed: u64,
    pub last_play: u64,
    pub approved_account: Option<ActorId>,
    pub ft_contract_id: Option<ActorId>,
    pub ft_transaction_id: TransactionId,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
}
