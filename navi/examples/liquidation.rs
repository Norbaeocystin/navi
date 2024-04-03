use ethnum::u256;
use log::{info, LevelFilter};
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::SuiAddress;
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
    let storage_data = get_storage_data(&client).await;
    // change to your address
    let liquidator_address = SuiAddress::ZERO;
    // change to address which you want to liquidate
    let liquour_address = SuiAddress::ZERO;
    let balances = get_user_balances(&client, storage_data.reserves_count, liquour_address).await;
    info!("balances: {:#?}", balances);
    let navi = Navi::new(&client, liquidator_address).await;
    let reserve_idx2oracle = get_oracles_v2(&client).await;
    let (collaterals, debts) = balances.unwrap();
    let decimals = 10_f64.powi(8);
    let mut total_collateral_value = 0.0;
    let mut max_collateral_value = 0.0;
    let mut max_collateral_amount = 0;
    let mut max_collateral_asset = "".to_string();
    let mut collateral_value = 0.0;
    let mut total_debt_value = 0.0;
    let mut max_debt_value = 0.0;
    let mut max_debt_amount = 0;
    let mut max_debt_asset = "".to_string();
    for reserve in navi.reserves.iter() {
        let coll = collaterals.get(&reserve.name).unwrap_or(&0);
        if coll == &0 {
            continue;
        }
        let supply_index = reserve.value.current_supply_index.checked_div(u256::new(10_u128.pow(19))).unwrap().as_u128();
        let threshold = (reserve.value.liquidation_factors.threshold.as_u128()/10_u128.pow(20)) as f64/ 10_f64.powi(7);
        let oracle = reserve_idx2oracle.get(&reserve.value.oracle_id).unwrap();
        let d = 10_u128.pow(oracle.decimal as u32);
        let price = oracle.get_price_as_f64();
        let mut coll_norm = (*coll as u128 * supply_index)/10_u128.pow(9);
        let coll_value = (coll_norm as f64/decimals) * price;
        coll_norm = (coll_norm * d)/10_u128.pow(8);
        collateral_value += coll_value * threshold;
        total_collateral_value += coll_value;
        // info!("coll: {} {} {} {} {}", threshold, coll_value, price, coll_norm, reserve.value.coin_type);
        if max_collateral_value < (coll_value * threshold){
            max_collateral_value = coll_value * threshold;
            max_collateral_amount = coll_norm as u64;
            let mut ct = reserve.value.coin_type.clone();
            if ct.starts_with("0x") {
                ct.insert_str(0, "0x");
            }
            max_collateral_asset = ct;
        }
    }
    for reserve in navi.reserves.iter() {
        let debt = debts.get(&reserve.name).unwrap_or(&0);
        if debt == &0 {
            continue;
        }
        let borrow_index = reserve.value.current_borrow_index.checked_div(u256::new(10_u128.pow(19))).unwrap().as_u128();
        let mut debt_norm = (*debt as u128 * borrow_index)/10_u128.pow(9);
        let oracle = reserve_idx2oracle.get(&reserve.value.oracle_id).unwrap();
        let d = 10_u128.pow(oracle.decimal as u32);
        let price = oracle.get_price_as_f64();
        let mut debt_value = ( debt_norm as f64/decimals) * price;
        debt_norm = (debt_norm * d)/10_u128.pow(8);
        total_debt_value += debt_value;
        debt_value += debt_value;
        // info!("{} {} {} {}", debt_value, price, debt_norm, reserve.value.coin_type);
        if max_debt_value < debt_value {
            let mut ct = reserve.value.coin_type.clone();
            if ct.starts_with("0x") {
                ct.insert_str(0, "0x");
            }
            max_debt_asset = ct;
            max_debt_value = debt_value;
            max_debt_amount = debt_norm as u64;
        }
    }

}