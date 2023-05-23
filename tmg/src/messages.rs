use crate::{exec, msg, ActorId, AttributeId, BTreeSet, StoreAction, StoreEvent};
use gstd::prelude::*;

pub async fn get_attributes(tmg_store_id: &ActorId) -> BTreeSet<AttributeId> {
    let reply: StoreEvent = msg::send_for_reply_as(
        *tmg_store_id,
        StoreAction::GetAttributes {
            tamagotchi_id: exec::program_id(),
        },
        0,
    )
    .expect("Error in sending a message `StoreAction::GetAttributes")
    .await
    .expect("Unable to decode `StoreEvent`");
    if let StoreEvent::Attributes { attributes } = reply {
        attributes
    } else {
        panic!("Wrong received message");
    }
}

pub async fn buy_attribute(
    store_id: &ActorId,
    attribute_id: AttributeId,
) -> Result<(), StoreEvent> {
    let reply = msg::send_for_reply_as::<_, StoreEvent>(
        *store_id,
        StoreAction::BuyAttribute { attribute_id },
        0,
    )
    .expect("Error in sending a message `StoreAction::BuyAttribute`")
    .await;
    match reply {
        Ok(StoreEvent::AttributeSold { success }) => {
            if success {
                Ok(())
            } else {
                Err(StoreEvent::ErrorDuringPurchase)
            }
        }
        Ok(StoreEvent::CompletePrevTx(attribute_id)) => {
            Err(StoreEvent::CompletePrevTx(attribute_id))
        }
        _ => Err(StoreEvent::ErrorDuringPurchase),
    }
}

//#[derive(Encode, Decode)]
//pub enum StoreEvent {
//  ErrorDuringPurchase,
//CompletePrevTx(AttributeId),
//}
