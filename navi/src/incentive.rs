use bcs::from_bytes;
use ethnum::u256;
use log::info;
use serde_derive::{Deserialize, Serialize};
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::SuiClient;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::collection_types::Table;
use sui_types::id::ID;
use crate::constants::INCENTIVE_V2;
use crate::oracle::match_sui_raw_data;


pub async fn get_incentive(client: &SuiClient) -> Incentive {
    let incentive_result = client.read_api().get_move_object_bcs(INCENTIVE_V2.parse().unwrap()).await.unwrap();
    let incentive: Incentive = from_bytes(&incentive_result).unwrap();
    return incentive;
}

pub async fn get_incentive_fund_pool_ids(client: &SuiClient, incentive: &Incentive) -> Vec<ObjectID> {
    let result = client.read_api().get_dynamic_fields(incentive.funds.id, None, None).await.unwrap();
    let ids: Vec<ObjectID> = result.data.iter().map(|d_o| d_o.name.value.as_str().unwrap().parse::<ObjectID>().unwrap()).collect();
    return ids;
    // // this pools are used ...
    // info!("{:#?}", pool_ids);
    // let ids: Vec<ObjectID> = result.data.iter().map(|d_o| d_o.object_id.clone()).collect();
    // info!("{:#?}", ids);
    // let result = client.read_api().multi_get_object_with_options(ids, SuiObjectDataOptions{
    //     show_type: false,
    //     show_owner: false,
    //     show_previous_transaction: false,
    //     show_display: false,
    //     show_content: false,
    //     show_bcs: true,
    //     show_storage_rebate: false,
    // }).await.unwrap();
    // let incentive_funds_pool_infos:Vec<IncentiveFundsPoolInfoDF> = result.iter().map(|x| from_bytes::<IncentiveFundsPoolInfoDF>(&match_sui_raw_data(x.clone().data.unwrap().bcs.unwrap()).unwrap()).unwrap()).collect();
    // return incentive_funds_pool_infos;
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TypeName {
    pub name: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Incentive {
    pub id: ID,
    pub version: u64,
    pub pool_objs: Vec<SuiAddress>,
    pub inactive_objs: Vec<SuiAddress>,
    pub pools: Table,
    pub funds: Table,
    /*
    pools: Table<address, IncentivePool>,
	funds: Table<address, IncentiveFundsPoolInfo>
     */
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct IncentiveFundsPoolInfoDF {
    pub id: ID,
    pub name: SuiAddress,
    pub value: IncentiveFundsPoolInfo,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct IncentiveFundsPoolInfo {
    pub id: ID,
    pub oracle_id: u8,
    pub coin_type: TypeName,
}

// #[derive(Deserialize,Debug,Clone)]
// struct IncentivePoolInfo {
//     pool_id: SuiAddress,
//     funds: SuiAddress,
//     phase: u64,
//     start_at: u64,
//     end_at: u64,
//     closed_at: u64,
//     total_supply: u64,
//     asset_id: u8,
//     option: u8,
//     factor: u256,
//     distributed: u64,
//     available: u256,
//     total: u256
// }
//
// #[derive(Deserialize,Debug,Clone)]
// struct IncentiveAPYInfo  {
//     asset_id: u8,
//     apy: u256,
//     coin_types: Vec<String>
// }
//
// #[derive(Deserialize,Debug,Clone)]
// struct IncentivePoolInfoByPhase {
//     phase: u64,
//     pools:  Vec<IncentivePoolInfo>
// }