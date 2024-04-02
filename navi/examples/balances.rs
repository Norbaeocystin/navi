use log::{info, LevelFilter};
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::SuiAddress;
use navi::storage::get_storage_data;
use navi::user::get_user_balances;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let client = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await.unwrap();
   let storage_data = get_storage_data(&client).await;
    let address = SuiAddress::ZERO;
    let balances = get_user_balances(&client, storage_data.reserves_count, address).await;
    info!("balances: {:#?}", balances);
}