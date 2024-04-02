use std::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};
use sui_sdk::rpc_types::{EventFilter, ObjectChange, SuiTransactionBlockResponseOptions};
use sui_sdk::SuiClient;
use sui_types::balance::Balance;
use sui_types::base_types::{ObjectID, SequenceNumber};
use sui_types::id::ID;
use crate::constants::NAVI_INITIAL_PACKAGE;

#[derive(Serialize,Deserialize,Debug)]
pub struct Pool {
    pub id: ID,
    pub balance: Balance,
    pub treasury_balance: Balance,
    pub decimal: u8
}

pub async fn get_pool_create(client: &SuiClient) -> BTreeMap<String, (ObjectID, SequenceNumber )>{
    let query = format!("{NAVI_INITIAL_PACKAGE}::pool::PoolCreate");
    let response = client.event_api().query_events(
        EventFilter::MoveEventType(
        query.parse().unwrap())
        , None, None, false).await.unwrap();
    let digests = response.data.iter().map(|x| x.id.tx_digest).collect();
    let txs = client.read_api().multi_get_transactions_with_options(digests, SuiTransactionBlockResponseOptions{
        show_input: false,
        show_raw_input: false,
        show_effects: false,
        show_events: false,
        show_object_changes: true,
        show_balance_changes: false,
        show_raw_effects: false,
    }).await.unwrap();
    let mut coin_tag2pool = BTreeMap::new();
    for tx in txs.iter() {
        let object_changes  = tx.clone().object_changes.unwrap().clone();
        for change  in object_changes.iter() {
            match change {
                ObjectChange::Created { object_type, object_id, version, .. } => {
                    if object_type.name.as_str().contains("Pool") {
                        coin_tag2pool.insert(object_type.type_params.last().unwrap().to_string(), (object_id.clone(), version.clone()));
                    }
                }
                _ => {}
            }
        }
    }
    return coin_tag2pool;
}