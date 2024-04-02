use std::collections::BTreeMap;
use std::str::FromStr;
use bcs::from_bytes;
use log::{debug, info};
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::SuiClient;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::Argument;
use sui_types::TypeTag;
use crate::pools::get_pool_create;
use crate::storage::{get_storage_data, match_sui_raw_data, Reserve};
use crate::transactions::{incentive_v2_borrow, incentive_v2_borrow_as_balance, incentive_v2_borrow_with_transfer, incentive_v2_entry_deposit, incentive_v2_entry_repay};

pub struct Navi {
    pub pools: BTreeMap<String, (ObjectID,SequenceNumber)>,
    pub reserves: Vec<Reserve>,
    pub address: SuiAddress,
}

impl Navi {
    pub async fn new(client: &SuiClient, address: SuiAddress) -> Navi {
        let storage = get_storage_data(&client).await;
        debug!("{:?}", storage);
        let dynamic_fields = client.read_api().get_dynamic_fields(storage.reserves.id, None, None).await.unwrap();
        let reserve_ids: Vec<ObjectID> = dynamic_fields.data.iter().map(|x| x.object_id).collect();
        debug!("reserve ids: {:?}", reserve_ids);
        let results = client.read_api().multi_get_object_with_options(reserve_ids, SuiObjectDataOptions{
            show_type: false,
            show_owner: false,
            show_previous_transaction: false,
            show_display: false,
            show_content: false,
            show_bcs: true,
            show_storage_rebate: false,
        }).await.unwrap();
        let reserves: Vec<Reserve> = results.iter().map(|x| from_bytes::<Reserve>(&*match_sui_raw_data(x.data.clone().unwrap().bcs.unwrap()).unwrap()).unwrap()).collect();
        debug!("{:?}", reserves);
        let pools = get_pool_create(&client).await;
        return Navi{ pools: pools, reserves:reserves, address:address }
    }

    pub fn borrow_with_transfer(&self, mut coin_tag: TypeTag, amount: u64, mut tb: ProgrammableTransactionBuilder) -> ProgrammableTransactionBuilder {
        let (pool_id, isv) = self.pools.get(&coin_tag.to_string()).unwrap();
        let mut ct = coin_tag.to_string();
        if coin_tag.to_string().starts_with("0x2::") {
            ct = ct.replace("0x2","0x0000000000000000000000000000000000000000000000000000000000000002");
        }
        ct = ct.replace("0x","");
        let tags:Vec<String> = self.reserves.iter().map(|x| x.value.coin_type.clone()).collect();
        debug!("{:?} {}", tags, ct);
        let reserve_idx = self.reserves.iter().find(|x| x.value.coin_type == ct).unwrap().name;
        let tb = incentive_v2_borrow_with_transfer(tb,
                                                   coin_tag,
                                                   self.address,
                                                   pool_id.clone(),
                                                   isv.clone(),
                                                   reserve_idx,
                                                   amount,
        );
        return tb;
    }

    pub fn borrow(&self, mut coin_tag: TypeTag, amount: u64, mut tb: ProgrammableTransactionBuilder) -> ProgrammableTransactionBuilder {
        let (pool_id, isv) = self.pools.get(&coin_tag.to_string()).unwrap();
        let mut ct = coin_tag.to_string();
        if coin_tag.to_string().starts_with("0x2::") {
            ct = ct.replace("0x2","0x0000000000000000000000000000000000000000000000000000000000000002");
        }
        // replace was replacing 0x also 0x0 in cetus
        ct = ct.replace("0x", "");
        let tags:Vec<String> = self.reserves.iter().map(|x| x.value.coin_type.clone()).collect();
        // info!("{:?} {}", tags, ct);
        let reserve_idx = self.reserves.iter().find(|x| x.value.coin_type == ct || x.value.coin_type.contains(&ct)  || ct.contains(&x.value.coin_type)).unwrap().name;
        let tb = incentive_v2_borrow(tb,
                                                   coin_tag,
                                                   pool_id.clone(),
                                                   isv.clone(),
                                                   reserve_idx,
                                                   amount,
        );
        return tb;
    }

    pub fn borrow_as_balance(&self, mut coin_tag: TypeTag, amount: u64, mut tb: ProgrammableTransactionBuilder) -> ProgrammableTransactionBuilder {
        let (pool_id, isv) = self.pools.get(&coin_tag.to_string()).unwrap();
        let mut ct = coin_tag.to_string();
        if coin_tag.to_string().starts_with("0x2::") {
            ct = ct.replace("0x2","0x0000000000000000000000000000000000000000000000000000000000000002");
        }
        ct = ct.replace("0x","");
        let tags:Vec<String> = self.reserves.iter().map(|x| x.value.coin_type.clone()).collect();
        debug!("{:?} {}", tags, ct);
        let reserve_idx = self.reserves.iter().find(|x| x.value.coin_type == ct || x.value.coin_type.contains(&ct)  || ct.contains(&x.value.coin_type)).unwrap().name;
        let tb = incentive_v2_borrow_as_balance(tb,
                                     coin_tag,
                                     pool_id.clone(),
                                     isv.clone(),
                                     reserve_idx,
                                     amount,
        );
        return tb;
    }

    pub fn repay(&self, mut coin_tag: TypeTag, mut tb: ProgrammableTransactionBuilder,
                 coin_arg: Argument,
                 amount_arg: Argument,
    ) -> ProgrammableTransactionBuilder {
        let (pool_id, isv) = self.pools.get(&coin_tag.to_string()).unwrap();
        let mut ct = coin_tag.to_string();
        if coin_tag.to_string().starts_with("0x2::") {
            ct = ct.replace("0x2","0x0000000000000000000000000000000000000000000000000000000000000002");
        }
        ct = ct.replace("0x","");
        let tags:Vec<String> = self.reserves.iter().map(|x| x.value.coin_type.clone()).collect();
        debug!("{:?} {}", tags, ct);
        let reserve_idx = self.reserves.iter().find(|x| x.value.coin_type == ct || x.value.coin_type.contains(&ct)  || ct.contains(&x.value.coin_type)).unwrap().name;
        let tb = incentive_v2_entry_repay(tb,
                                     coin_tag,
                                     pool_id.clone(),
                                     isv.clone(),
                                          reserve_idx,
                                          amount_arg,
                                          coin_arg,
        );
        return tb;
    }

    pub fn deposit(&self, mut coin_tag: TypeTag, mut tb: ProgrammableTransactionBuilder,
                 coin_arg: Argument,
                 amount_arg: Argument,
    ) -> ProgrammableTransactionBuilder {
        let (pool_id, isv) = self.pools.get(&coin_tag.to_string()).unwrap();
        let mut ct = coin_tag.to_string();
        if coin_tag.to_string().starts_with("0x2::") {
            ct = ct.replace("0x2","0x0000000000000000000000000000000000000000000000000000000000000002");
        }
        ct = ct.replace("0x","");
        let tags:Vec<String> = self.reserves.iter().map(|x| x.value.coin_type.clone()).collect();
        debug!("{:?} {}", tags, ct);
        let reserve_idx = self.reserves.iter().find(|x| x.value.coin_type == ct || x.value.coin_type.contains(&ct)  || ct.contains(&x.value.coin_type)).unwrap().name;
        let tb = incentive_v2_entry_deposit(tb,
                                          coin_tag,
                                          pool_id.clone(),
                                          isv.clone(),
                                          reserve_idx,
                                          amount_arg,
                                          coin_arg,
        );
        return tb;
    }
}