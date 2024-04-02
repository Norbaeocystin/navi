use std::str::FromStr;
use log::debug;
use sui_sdk::rpc_types::{SuiObjectDataOptions, SuiRawData};
use sui_sdk::SuiClient;
use sui_types::base_types::{ObjectID, SequenceNumber};
use sui_types::object::Owner;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, CallArg, ObjectArg};
use sui_types::TypeTag;

pub fn coin_value(mut tb: ProgrammableTransactionBuilder, coin_tag: TypeTag, coin_arg: Argument) -> ProgrammableTransactionBuilder{
    tb.programmable_move_call(
        "0x0000000000000000000000000000000000000000000000000000000000000002".parse().unwrap(),
        "coin".parse().unwrap(),
        "value".parse().unwrap(),
        vec![
            coin_tag,
        ],
        vec![
            coin_arg
        ],
    );
    return tb;
}

pub fn match_sui_raw_data(sui_raw_data: SuiRawData) -> Option<Vec<u8>>{
    match sui_raw_data {
        SuiRawData::MoveObject(object) => {
            return Some(object.bcs_bytes);
        }
        SuiRawData::Package(_) => {}
    }
    return None;
}

pub async fn get_initial_shared_version(client: &SuiClient, id: ObjectID) -> SequenceNumber{
    return get_initial_shared_version_from_owner(client.read_api().get_object_with_options(id, SuiObjectDataOptions{
        show_type: false,
        show_owner: true,
        show_previous_transaction: false,
        show_display: false,
        show_content: false,
        show_bcs: false,
        show_storage_rebate: false,
    }).await.unwrap().data.unwrap().owner.unwrap()).unwrap()
}

pub fn get_initial_shared_version_from_owner(owner: Owner) -> Option<SequenceNumber>{
    match owner {
        Owner::Shared { initial_shared_version } => {
            debug!("initial shared version: {}", initial_shared_version);
            return Some(initial_shared_version);
        }
        _ => {}
    }
    return None;
}

pub fn ptb_get_length(mut tb: ProgrammableTransactionBuilder) -> (usize, ProgrammableTransactionBuilder) {
    let ptb = tb.finish();
    let length = ptb.commands.len();
    let mut tb = ProgrammableTransactionBuilder::new();
    for input in ptb.inputs {
        tb.input(input);
    }
    for command in ptb.commands {
        tb.command(command);
    }
    return (length, tb);
}

pub fn get_object_arg(id:ObjectID) -> CallArg {
    return CallArg::Object(ObjectArg::SharedObject {
        id: id,
        initial_shared_version: SequenceNumber::default(),
        mutable: false
    });
}

pub fn get_mut_object_arg(id:ObjectID) -> CallArg {
    return CallArg::Object(ObjectArg::SharedObject {
        id: id,
        initial_shared_version: SequenceNumber::default(),
        mutable: true
    });
}