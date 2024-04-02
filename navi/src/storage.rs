use std::collections::BTreeMap;
use std::str::FromStr;
use serde_derive::{Deserialize};
use sui_sdk::rpc_types::{SuiObjectDataOptions, SuiRawData};
use sui_sdk::SuiClient;
use sui_types::base_types::{ObjectID, SuiAddress};
use crate::constants::NAVI_STORAGE;
use bcs::from_bytes;
use ethnum::u256;
use sui_types::collection_types::Table;
use sui_types::id::ID;
#[derive(Deserialize,Debug)]
pub struct TokenBalance {
    pub user_state: Table,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub total_supply: u256
}

#[derive(Deserialize,Debug)]
pub struct Storage {
    pub id: ID,
    pub version: u64,
    pub paused: bool,
    pub reserves: Table,
    pub reserves_count: u8,
    pub users: Vec<SuiAddress>,
    // user_info: Table<address, UserInfo>
    pub user_info: Table,
}

#[derive(Deserialize,Debug)]
struct BorrowRateFactors  {
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub base_rate: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub multiplier: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub jump_rate_multiplier: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub reserve_factor: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub optimal_utilization:u256
}
#[derive(Deserialize,Debug)]
pub struct LiquidationFactors {
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub ratio: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub bonus: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub threshold: u256,
}

#[derive(Deserialize,Debug)]
pub struct Reserve {
    pub id: ID,
    pub name: u8,
    pub value: ReserveData,
}

// u256 - here is stored as [u8, 32]
#[derive(Deserialize,Debug)]
pub struct ReserveData  {
    pub id: u8,
    pub oracle_id: u8,
    pub coin_type: String,
    pub is_isolated: bool,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub supply_cap_ceiling: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub borrow_cap_ceiling: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub current_supply_rate: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub current_borrow_rate: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub current_supply_index: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub current_borrow_index: u256,
    pub supply_balance: TokenBalance,
    pub borrow_balance: TokenBalance,
    pub last_update_timestamp: u64,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub ltv: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub treasury_factor: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub treasury_balance: u256,
    pub borrow_rate_factors: BorrowRateFactors,
    pub liquidation_factors: LiquidationFactors,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub reserve_field_a: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub reserve_field_b: u256,
    #[serde(deserialize_with = "misc_utils::serde::u256_from_le_bytes")]
    pub reserve_field_c: u256
}

pub async fn get_storage_data(client: &SuiClient) -> Storage {
   let result = client
       .read_api()
       .get_move_object_bcs(ObjectID::from_str(NAVI_STORAGE).unwrap()).await.unwrap();
    let storage:Storage =  from_bytes(&*result).unwrap();
    return storage;
}

pub async fn get_storage_and_reserves(client: &SuiClient) -> (Storage, Vec<Reserve>){
    let result = client
        .read_api()
        .get_move_object_bcs(ObjectID::from_str(NAVI_STORAGE).unwrap()).await.unwrap();
    let storage:Storage =  from_bytes(&*result).unwrap();
    let dynamic_fields = client.read_api().get_dynamic_fields(storage.reserves.id, None, None).await.unwrap();
    let reserve_ids: Vec<ObjectID> = dynamic_fields.data.iter().map(|x| x.object_id).collect();
    let results = client.read_api().multi_get_object_with_options(reserve_ids, SuiObjectDataOptions{
        show_type: false,
        show_owner: false,
        show_previous_transaction: false,
        show_display: false,
        show_content: false,
        show_bcs: true,
        show_storage_rebate: false,
    }).await.unwrap();
    let mut reserves: Vec<Reserve> = results.iter().map(|x| from_bytes::<Reserve>(&*match_sui_raw_data(x.data.clone().unwrap().bcs.unwrap()).unwrap()).unwrap()).collect();
    reserves.sort_by(|a,b| a.name.cmp(&b.name));
    return (storage, reserves)
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

// you need to aply interest to collaterals to get correct numbers
pub fn update_collaterals(reserves: &Vec<Reserve>, mut colls: BTreeMap<u8, u64>) -> BTreeMap<u8, u64>{
    for reserve in reserves.iter() {
        let coll = colls.get(&reserve.name).unwrap_or(&0);
        if coll == &0 {
            continue;
        }
        let supply_index = reserve.value.current_supply_index.checked_div(u256::new(10_u128.pow(19))).unwrap().as_u128();
        let coll_norm = (*coll as u128 * supply_index)/10_u128.pow(9);
        colls.insert(reserve.name.clone(), coll_norm as u64);
    }
    return colls;
}

// you need to aply interest on debts to get correct numbers
pub fn update_debts(reserves: &Vec<Reserve>, mut debts: BTreeMap<u8, u64>) -> BTreeMap<u8, u64>{
    for reserve in reserves.iter() {
        let debt = debts.get(&reserve.name).unwrap_or(&0);
        if debt == &0 {
            continue;
        }
        let borrow_index = reserve.value.current_borrow_index.checked_div(u256::new(10_u128.pow(19))).unwrap().as_u128();
        let debt_norm = (*debt as u128 * borrow_index)/10_u128.pow(9);
        debts.insert(reserve.name.clone(), debt_norm as u64);
    }
    return debts;
}
