use std::collections::BTreeMap;
use std::str::FromStr;
use ethnum::u256;
use log::{debug, info};
use sui_sdk::SuiClient;
use sui_types::base_types::SuiAddress;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::TransactionKind;
use sui_types::transaction::TransactionKind::ProgrammableTransaction;
use crate::oracle::{get_oracles_v2, Oracle};
use crate::storage::{get_storage_and_reserves, Reserve};
use crate::transactions::storage_get_user_balance;


pub async fn get_user_balances(client: &SuiClient, reserves_size: u8, address: SuiAddress) -> Option<(BTreeMap<u8, u64>, BTreeMap<u8, u64>)> {
    let mut tb = ProgrammableTransactionBuilder::new();
    for i in 0..reserves_size {
        tb = storage_get_user_balance(tb, i, address);
    }
    let response = client.read_api().dev_inspect_transaction_block(address,
                                                                       TransactionKind::ProgrammableTransaction(tb.finish()),
                                                                       None,
                                                                       None,
                                                                       None,
    ).await.unwrap();
    let unwrapped = response.results.unwrap();
    let mut coll_balances = BTreeMap::new();
    let mut debt_balances = BTreeMap::new();
    for (idx, item) in unwrapped.iter().enumerate(){
        let balance = u256::from_le_bytes(item.return_values.clone().first().unwrap().0.clone().try_into().unwrap());
        let debt_balance = u256::from_le_bytes(item.return_values.clone().last().unwrap().0.clone().try_into().unwrap());
        if balance > 0 {
            coll_balances.insert(idx as u8, balance.as_u64());
        }
        if debt_balance > 0 {
            debt_balances.insert(idx as u8, debt_balance.as_u64());
        }
        debug!("{} {}", balance, debt_balance);
    }
    return Some((coll_balances, debt_balances))
}