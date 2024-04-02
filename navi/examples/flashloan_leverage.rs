use log::{info, LevelFilter};
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::SuiAddress;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, TransactionKind};
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
    let BORROW = 100_000_000_000;
    let FEE = 60_000_000;
    let navi = Navi::new(&client, sui_address).await;
    let mut tb = ProgrammableTransactionBuilder::new();
    let asset = "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI";
    info!("pools: {:#?}", navi.pools);
    let (pool_id, isv) = navi.pools.get(&"0x2::sui::SUI".to_string()).unwrap();
    let mut ct = asset.to_string();
    ct = ct.replace("0x","");
    let reserve_idx = navi.reserves.iter().find(|x| x.value.coin_type == ct || x.value.coin_type.contains(&ct)  || ct.contains(&x.value.coin_type)).unwrap().name;
    // 0
    tb = lending_flash_loan_with_ctx(tb,asset.parse().unwrap(), pool_id.clone(), isv.clone(), BORROW);
    // 1
    tb.programmable_move_call(
        "0x0000000000000000000000000000000000000000000000000000000000000002".parse().unwrap(),
        "coin".parse().unwrap(),
        "from_balance".parse().unwrap(),
        vec![asset.parse().unwrap()],
        vec![Argument::NestedResult(0,0)],
    );
    // 2
    tb = coin_value(tb, asset.parse().unwrap(), Argument::Result(1));
    // 3
    tb = navi.deposit(asset.parse().unwrap(), tb, Argument::Result(1), Argument::Result(2) );
    // 4
    tb = navi.borrow_as_balance(asset.parse().unwrap(), BORROW + FEE, tb);
    // 5
    tb = lending_flash_repay_with_ctx(tb, asset.parse().unwrap(), pool_id.clone(), isv.clone(), Argument::NestedResult(0, 1), Argument::Result(4));
    // 6
    tb.programmable_move_call(
        "0x0000000000000000000000000000000000000000000000000000000000000002".parse().unwrap(),
        "balance".parse().unwrap(),
        "destroy_zero".parse().unwrap(),
        vec![asset.parse().unwrap()],
        vec![Argument::Result(5)],
    );

    let ptb = tb.finish();
    let result = client.read_api().dev_inspect_transaction_block(sui_address, TransactionKind::ProgrammableTransaction(ptb.clone()), None, None, None).await.unwrap();
    info!("{:#?}", result);
}