use log::{info, LevelFilter};
use sui_sdk::SuiClientBuilder;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, Command};
use sui_types::transaction::ObjectArg::ImmOrOwnedObject;
use sui_types::transaction::TransactionKind::ProgrammableTransaction;
use misc_utils::transaction::TransactionWrapper;

#[tokio::main]
async fn main(){
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let sui_client = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await.unwrap();
    let mut tb = ProgrammableTransactionBuilder::new();
    let address = SuiAddress::ZERO;
    // usdc wormhole
    let coin = "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf::coin::COIN";
    let mut result = sui_client.coin_read_api().get_coins(address, Some(coin.to_string()), None, None).await.unwrap();
    let first = result.data.pop().unwrap();
    let coin_arg = tb.input(CallArg::Object(ImmOrOwnedObject(first.object_ref()))).unwrap();
    let mut args = vec![];
    for item in result.data {
        let arg = tb.input(CallArg::Object(ImmOrOwnedObject(item.object_ref()))).unwrap();
        args.push(arg);
    }
    tb.command(Command::MergeCoins(coin_arg,args ));
    let txw = TransactionWrapper::new(&sui_client);
    let result = txw.process_ptx(tb.finish(), None, None, None,).await;
    info!("{:?}", result);
}