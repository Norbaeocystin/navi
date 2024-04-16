use log::{info, LevelFilter};
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::SuiAddress;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, CallArg, Command, TransactionKind};
use misc_utils::transaction::TransactionWrapper;
use misc_utils::utils::coin_value;
use navi::navi::Navi;
use navi::transactions::{lending_flash_loan_with_ctx, lending_flash_repay_with_ctx};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let client = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await.unwrap();
    // T0D0 change to your address
    let sui_address = SuiAddress::ZERO;
    let navi = Navi::new(&client, sui_address).await;
    let mut tb = ProgrammableTransactionBuilder::new();
    let asset = "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI";
    info!("pools: {:#?}", navi.pools);
    let (pool_id, isv) = navi.pools.get(&"0x2::sui::SUI".to_string()).unwrap();
    let mut ct = asset.to_string();
    ct = ct.replace("0x","");
    // 0
    let value = tb.input(CallArg::Pure(1_000_000_000_u64.to_le_bytes().to_vec())).unwrap();
    tb.command(Command::SplitCoins(Argument::GasCoin, vec![value]));
    tb = navi.deposit(asset.parse().unwrap(), tb, Argument::NestedResult(0,0), value);
    let ptb = tb.finish();
    // let txwrapper = TransactionWrapper::new(&client);
    // let result = txwrapper.process_ptx(ptb, None, None, None).await;
    let result = client.read_api().dev_inspect_transaction_block(sui_address, TransactionKind::ProgrammableTransaction(ptb.clone()), None, None, None).await.unwrap();
    info!("{:#?}", result);
}