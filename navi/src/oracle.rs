use std::collections::BTreeMap;
use std::str::FromStr;
use serde_derive::{Deserialize};
use sui_sdk::SuiClient;
use sui_types::base_types::ObjectID;
use sui_types::collection_types::Table;
use sui_types::id::ID;
use crate::constants::NAVI_ORACLE;
use bcs::from_bytes;
use ethnum::{u256, U256};
use sui_sdk::rpc_types::{SuiObjectDataOptions, SuiRawData};


pub async fn get_oracles(client: &SuiClient) -> Vec<OracleDF>{
    let oracle_bcs = client.read_api().get_move_object_bcs(ObjectID::from_str(NAVI_ORACLE).unwrap()).await.unwrap();
    let oracle: NaviOracle = from_bytes(&oracle_bcs).unwrap();
    let object_id = oracle.price_oracles.id;
    let fields = client.read_api().get_dynamic_fields(object_id, None, None).await.unwrap();
    let mut ids = vec![];
    for df in fields.data.iter() {
        ids.push(df.object_id.clone())
    }
    let response = client.read_api().multi_get_object_with_options(ids, SuiObjectDataOptions{
        show_type: false,
        show_owner: false,
        show_previous_transaction: false,
        show_display: false,
        show_content: false,
        show_bcs: true,
        show_storage_rebate: false,
    }).await.unwrap();
    let mut results: Vec<OracleDF> = response.iter().map(|x|
        from_bytes::<OracleDF>(&*match_sui_raw_data(x.data.as_ref().unwrap().bcs.clone().unwrap()).unwrap()).unwrap()
    ).collect();
    results.sort_by(|a, b| a.name.cmp(&b.name));
    return results;
}

pub fn match_sui_raw_data(sui_raw_data: SuiRawData) -> Option<Vec<u8>>{
    match sui_raw_data {
        SuiRawData::MoveObject(object) => {
            return Some(object.bcs_bytes);
        }
        SuiRawData::Package(_) => {}
    }
    return None;
}

pub async fn get_oracles_v2(client: &SuiClient) -> BTreeMap<u8, Oracle>{
    let oracle_bcs = client.read_api().get_move_object_bcs(ObjectID::from_str(NAVI_ORACLE).unwrap()).await.unwrap();
    let oracle: NaviOracle = from_bytes(&oracle_bcs).unwrap();
    let object_id = oracle.price_oracles.id;
    let fields = client.read_api().get_dynamic_fields(object_id, None, None).await.unwrap();
    let mut ids = vec![];
    for df in fields.data.iter() {
        ids.push(df.object_id.clone())
    }
    let response = client.read_api().multi_get_object_with_options(ids, SuiObjectDataOptions{
        show_type: false,
        show_owner: false,
        show_previous_transaction: false,
        show_display: false,
        show_content: false,
        show_bcs: true,
        show_storage_rebate: false,
    }).await.unwrap();
    let mut results = BTreeMap::new();
    let mut oracles: Vec<OracleDF> = response.iter().map(|x|
        from_bytes::<OracleDF>(&*match_sui_raw_data(x.data.as_ref().unwrap().bcs.clone().unwrap()).unwrap()).unwrap()
    ).collect();
    for oracle in oracles.iter(){
        results.insert(oracle.name, oracle.value.clone());
    }
    return results;
}

#[derive(Deserialize,Debug)]
pub struct NaviOracle {
    id: ID,
    version: u64,
    update_interval: u64,
    price_oracles: Table,
}
#[derive(Deserialize,Debug)]
pub struct OracleDF {
    pub id: ID,
    pub name: u8,
    pub value: Oracle
}

#[derive(Deserialize,Debug,Clone)]
pub struct Oracle {
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub value: u256, // u256
    pub decimal: u8,
    pub timestamp: u64,
}

impl Oracle {
    pub fn get_price_as_f64(&self) -> f64{
        let price = self.value.as_u64() as f64;
        let decimals = 10_f64.powi(self.decimal as i32);
        return price/decimals
    }
}

impl OracleDF {
    pub fn get_price_as_f64(&self) -> f64{
        let price = self.value.value.as_u64() as f64;
        let decimals = 10_f64.powi(self.value.decimal as i32);
        return price/decimals
    }
}