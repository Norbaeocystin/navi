use log::{info, LevelFilter};
use sui_sdk::rpc_types::EventFilter;
use sui_sdk::SuiClientBuilder;

#[tokio::main]
async fn main(){
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let client = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await.unwrap();
    // fetch last liquidation call events
    let results = client.event_api().query_events(
            EventFilter::MoveEventType(
                "0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca::lending::LiquidationCallEvent".parse().unwrap()
            ),
            None,
            Some(50),
            true,
        ).await.unwrap();
    for result in results.data {
        info!("{:?}", result);
    }
    // last flashloan events
    let results = client.event_api().query_events(
        EventFilter::MoveEventType(
            "0x06007a2d0ddd3ef4844c6d19c83f71475d6d3ac2d139188d6b62c052e6965edd::flash_loan::FlashLoan".parse().unwrap()
        ),
        None,
        Some(50),
        true,
    ).await.unwrap();
    for result in results.data {
        info!("{:?}", result);
    }
}