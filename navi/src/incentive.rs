use serde_derive::{Deserialize, Serialize};
use sui_types::base_types::SuiAddress;
use sui_types::collection_types::Table;
use sui_types::id::ID;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TypeName {
    pub name: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Incentive {
    id: ID,
    version: u64,
    pool_objs: Vec<SuiAddress>,
    inactive_objs: Vec<SuiAddress>,
    pools: Table,
    funds: Table,
    /*
    pools: Table<address, IncentivePool>,
	funds: Table<address, IncentiveFundsPoolInfo>
     */
}

#[derive(Serialize,Deserialize,Debug,Clone)]
struct IncentiveFundsPoolInfo {
    id: ID,
    oracle_id: u8,
    coin_type: TypeName,
}