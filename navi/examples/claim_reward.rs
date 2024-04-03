use ethnum::u256;
use log::{info, LevelFilter};
use sui_sdk::{SuiClient, SuiClientBuilder};
use sui_types::base_types::SuiAddress;
use navi::incentive::{get_incentive, get_incentive_fund_pool_ids, Incentive};
use navi::navi::Navi;
use navi::oracle::{get_oracles, get_oracles_v2};
use navi::storage::get_storage_data;
use navi::user::get_user_balances;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let client = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await.unwrap();
    let incentive = get_incentive(&client).await;
    info!("{:#?}", incentive);
    let funds_pool_info_dfs = get_incentive_fund_pool_ids(&client, &incentive).await;
    info!("{:#?}", funds_pool_info_dfs);
}
