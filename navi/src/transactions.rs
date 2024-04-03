use std::str::FromStr;
use clap::Arg;
use log::info;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::base_types::MoveObjectType_::Coin;
use sui_types::clock::Clock;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, CallArg, ObjectArg};
use sui_types::transaction::ObjectArg::SharedObject;
use sui_types::TypeTag;
use misc_utils::utils::ptb_get_length;
use crate::constants::{INCENTIVE_V1, INCENTIVE_V2, NAVI_FLASHLOAN_CONFIG, NAVI_FLASHLOAN_INITIAL_SHARED_VERSION, NAVI_INITIAL_PACKAGE, NAVI_ORACLE, NAVI_PACKAGE, NAVI_PACKAGE_LATEST, NAVI_STORAGE};

pub fn get_storage_arg() -> CallArg {
    return CallArg::Object(ObjectArg::SharedObject {
        id: ObjectID::from_str(NAVI_STORAGE).unwrap(),
        initial_shared_version: SequenceNumber::from_u64(8202844),
        mutable: true
    })
}

pub fn get_oracle_arg() -> CallArg {
    return CallArg::Object(ObjectArg::SharedObject {
        id: ObjectID::from_str(NAVI_ORACLE).unwrap(),
        initial_shared_version: SequenceNumber::from_u64(8202835),
        mutable: false
    })
}

pub fn get_incentive_v1_arg() -> CallArg {
    CallArg::Object(ObjectArg::SharedObject {
        id: ObjectID::from_str(INCENTIVE_V1).unwrap(),
        initial_shared_version: SequenceNumber::from_u64(8202844),
        mutable: true
    })
}
pub fn get_incentive_v2_arg() -> CallArg {
    CallArg::Object(ObjectArg::SharedObject {
        id: ObjectID::from_str(INCENTIVE_V2).unwrap(),
        initial_shared_version: SequenceNumber::from_u64(38232222),
        mutable: true
    })
}

// public withdraw<Ty0>(Arg0: &Clock, Arg1: &PriceOracle, Arg2: &mut Storage, Arg3: &mut Pool<Ty0>, Arg4: u8, Arg5: u64, Arg6: &mut Incentive, Arg7: &mut Incentive, Arg8: &mut TxContext): Balance<Ty0> {
pub fn incentive_v2_withdraw(mut tb: ProgrammableTransactionBuilder,
                                         coin_tag: TypeTag,
                                         // address: SuiAddress,
                                         pool_id: ObjectID,
                                         pool_initial_shared_version: SequenceNumber,
                                         reserve_idx: u8,
                                         amount: u64,
) -> ProgrammableTransactionBuilder {
    let clock_arg = CallArg::CLOCK_IMM;
    let oracle_arg = get_oracle_arg();
    let storage_arg = get_storage_arg();
    let pool_arg = CallArg::Object(ObjectArg::SharedObject {
        id: pool_id,
        initial_shared_version: pool_initial_shared_version,
        mutable: true
    });
    let incentive_v1_arg = get_incentive_v1_arg();
    let incentive_arg = get_incentive_v2_arg();
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "incentive_v2".parse().unwrap(),
        "withdraw".parse().unwrap(), // or use entry_borrow
        vec![coin_tag.clone()],
        vec![
            clock_arg,
            oracle_arg,
            storage_arg,
            pool_arg,
            CallArg::Pure(vec![reserve_idx]),
            CallArg::Pure(amount.to_le_bytes().to_vec()),
            incentive_v1_arg,
            incentive_arg,
        ],
    );
    return tb;
}

// public borrow<Ty0>(Arg0: &Clock, Arg1: &PriceOracle, Arg2: &mut Storage, Arg3: &mut Pool<Ty0>, Arg4: u8, Arg5: u64, Arg6: &mut Incentive, Arg7: &mut TxContext): Balance<Ty0> {
// after inscruction there needs to be coin created ... and trasnfered;
pub fn incentive_v2_borrow_with_transfer(mut tb: ProgrammableTransactionBuilder,
    coin_tag: TypeTag,
    address: SuiAddress,
    pool_id: ObjectID,
    pool_initial_shared_version: SequenceNumber,
    reserve_idx: u8,
    amount: u64,
) -> ProgrammableTransactionBuilder{
    let clock_arg = CallArg::CLOCK_IMM;
    let oracle_arg = get_oracle_arg();
    let storage_arg = get_storage_arg();
    let pool_arg = CallArg::Object(ObjectArg::SharedObject {
        id: pool_id,
        initial_shared_version: pool_initial_shared_version,
        mutable: true
    });
    let incentive_arg = get_incentive_v2_arg();
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
    "incentive_v2".parse().unwrap(),
        "borrow".parse().unwrap(), // or use entry_borrow
        vec![coin_tag.clone()],
        vec![
            clock_arg,
            oracle_arg,
            storage_arg,
            pool_arg,
            CallArg::Pure(vec![reserve_idx]),
            CallArg::Pure(amount.to_le_bytes().to_vec()),
            incentive_arg,
        ],
    );
    let (mut length, mut tb) = ptb_get_length(tb);
    //let commands = tb.finish().commands.len().clone();
    tb.programmable_move_call(
        "0x0000000000000000000000000000000000000000000000000000000000000002".parse().unwrap(),
        "coin".parse().unwrap(),
        "from_balance".parse().unwrap(),
        vec![coin_tag.clone()],
        vec![Argument::Result((length - 1) as u16)
        ],
    );
    let address_arg = tb.input(CallArg::Pure(address.to_vec())).unwrap();
    let correct_tag = format!("0x2::coin::Coin<{coin_tag}>");
    // let coin = Coin(coin_tag);
    tb.programmable_move_call(
        "0x0000000000000000000000000000000000000000000000000000000000000002".parse().unwrap(),
        "transfer".parse().unwrap(),
        "public_transfer".parse().unwrap(),
        vec![correct_tag.parse().unwrap()], //coin_tag.clone()],
        vec![Argument::Result(length as u16),
            address_arg,
        ],
    );
    return tb;
}


pub fn incentive_v2_borrow(mut tb: ProgrammableTransactionBuilder,
                                         coin_tag: TypeTag,
                                         pool_id: ObjectID,
                                         pool_initial_shared_version: SequenceNumber,
                                         reserve_idx: u8,
                                         amount: u64,
) -> ProgrammableTransactionBuilder{
    let clock_arg = CallArg::CLOCK_IMM;
    let oracle_arg = get_oracle_arg();
    let storage_arg = get_storage_arg();
    let pool_arg = CallArg::Object(ObjectArg::SharedObject {
        id: pool_id,
        initial_shared_version: pool_initial_shared_version,
        mutable: true
    });
    let incentive_arg = get_incentive_v2_arg();
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "incentive_v2".parse().unwrap(),
        "borrow".parse().unwrap(), // or use entry_borrow
        vec![coin_tag.clone()],
        vec![
            clock_arg,
            oracle_arg,
            storage_arg,
            pool_arg,
            CallArg::Pure(vec![reserve_idx]),
            CallArg::Pure(amount.to_le_bytes().to_vec()),
            incentive_arg,
        ],
    );
    let (mut length, mut tb) = ptb_get_length(tb);
    //let commands = tb.finish().commands.len().clone();
    tb.programmable_move_call(
        "0x0000000000000000000000000000000000000000000000000000000000000002".parse().unwrap(),
        "coin".parse().unwrap(),
        "from_balance".parse().unwrap(),
        vec![coin_tag.clone()],
        vec![Argument::Result((length - 1) as u16)
        ],
    );
    return tb;
}

pub fn incentive_v2_borrow_as_balance(mut tb: ProgrammableTransactionBuilder,
                           coin_tag: TypeTag,
                           pool_id: ObjectID,
                           pool_initial_shared_version: SequenceNumber,
                           reserve_idx: u8,
                           amount: u64,
) -> ProgrammableTransactionBuilder{
    let clock_arg = CallArg::CLOCK_IMM;
    let oracle_arg = get_oracle_arg();
    let storage_arg = get_storage_arg();
    let pool_arg = CallArg::Object(ObjectArg::SharedObject {
        id: pool_id,
        initial_shared_version: pool_initial_shared_version,
        mutable: true
    });
    let incentive_arg = get_incentive_v2_arg();
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "incentive_v2".parse().unwrap(),
        "borrow".parse().unwrap(), // or use entry_borrow
        vec![coin_tag.clone()],
        vec![
            clock_arg,
            oracle_arg,
            storage_arg,
            pool_arg,
            CallArg::Pure(vec![reserve_idx]),
            CallArg::Pure(amount.to_le_bytes().to_vec()),
            incentive_arg,
        ],
    );
    return tb;
}


// entry public entry_repay<Ty0>(Arg0: &Clock, Arg1: &PriceOracle, Arg2: &mut Storage, Arg3: &mut Pool<Ty0>, Arg4: u8, Arg5: Coin<Ty0>, Arg6: u64, Arg7: &mut Incentive, Arg8: &mut TxContext) {
pub fn incentive_v2_entry_repay(
    mut tb: ProgrammableTransactionBuilder,
    coin_tag: TypeTag,
    pool_id: ObjectID,
    pool_initial_shared_version: SequenceNumber,
    reserve_idx: u8,
    amount_arg: Argument,
    coin_arg: Argument,
) -> ProgrammableTransactionBuilder {
    let clock_arg = tb.input(CallArg::CLOCK_IMM).unwrap();
    let oracle_arg = tb.input(get_oracle_arg()).unwrap();
    let storage_arg = tb.input(get_storage_arg()).unwrap();
    let pool_arg = tb.input(CallArg::Object(ObjectArg::SharedObject {
        id: pool_id,
        initial_shared_version: pool_initial_shared_version,
        mutable: true
    })).unwrap();
    let incentive_arg = tb.input(get_incentive_v2_arg()).unwrap();
    let ridx = tb.input(CallArg::Pure(vec![reserve_idx])).unwrap();
    tb.programmable_move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "incentive_v2".parse().unwrap(),
        "entry_repay".parse().unwrap(), // or use entry_borrow
        vec![coin_tag.clone()],
        vec![
            clock_arg,
            oracle_arg,
            storage_arg,
            pool_arg,
            ridx,
            coin_arg,
            amount_arg,
            incentive_arg,
        ],
    );
    return tb;
}

pub fn incentive_v2_entry_deposit(
    mut tb: ProgrammableTransactionBuilder,
    coin_tag: TypeTag,
    pool_id: ObjectID,
    pool_initial_shared_version: SequenceNumber,
    reserve_idx: u8,
    amount_arg: Argument,
    coin_arg: Argument,
) -> ProgrammableTransactionBuilder {
    let clock_arg = tb.input(CallArg::CLOCK_IMM).unwrap();
    let storage_arg = tb.input(get_storage_arg()).unwrap();
    let pool_arg = tb.input(CallArg::Object(ObjectArg::SharedObject {
        id: pool_id,
        initial_shared_version: pool_initial_shared_version,
        mutable: true
    })).unwrap();
    let incentive_v1_arg = tb.input(get_incentive_v1_arg()).unwrap();
    let incentive_arg = tb.input(get_incentive_v2_arg()).unwrap();
    let ridx = tb.input(CallArg::Pure(vec![reserve_idx])).unwrap();
    tb.programmable_move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "incentive_v2".parse().unwrap(),
        "entry_deposit".parse().unwrap(), // or use entry_borrow
        vec![coin_tag.clone()],
        vec![
            clock_arg,
            storage_arg,
            pool_arg,
            ridx,
            coin_arg,
            amount_arg,
            incentive_v1_arg,
            incentive_arg,
        ],
    );
    return tb;
}
// public liquidation<Ty0, Ty1>(Arg0: &Clock, Arg1: &PriceOracle, Arg2: &mut Storage, Arg3: u8, Arg4: &mut Pool<Ty0>, Arg5: Balance<Ty0>, Arg6: u8, Arg7: &mut Pool<Ty1>, Arg8: address, Arg9: &mut Incentive, Arg10: &mut Incentive, Arg11: &mut TxContext): Balance<Ty1> * Balance<Ty0>
// before coin -> balance 0x2::coin::into_balance public fun into_balance<T>(coin: Coin<T>): Balance<T>
// after public fun from_balance<T>(balance: Balance<T>, ctx: &mut TxContext): Coin<T> {
pub fn incentive_v2_liquidation(mut tb: ProgrammableTransactionBuilder,
  liquor_address: SuiAddress,
    debt_asset: TypeTag,
    coll_asset: TypeTag,
    debt_reserve_idx: u8,
    debt_pool_id: ObjectID,
    debt_pool_isv: SequenceNumber,
                                coll_reserve_idx: u8,
    coll_pool_id: ObjectID,
    coll_pool_isv: SequenceNumber,
    balance_arg: Argument,
) -> ProgrammableTransactionBuilder {
    let clock_arg = tb.input(CallArg::CLOCK_IMM).unwrap();
    let oracle_arg = tb.input(get_oracle_arg()).unwrap();
    let storage_arg = tb.input(get_storage_arg()).unwrap();
    let incentive_arg_v1 = tb.input(get_incentive_v1_arg()).unwrap();
    let incentive_arg = tb.input(get_incentive_v2_arg()).unwrap();
    let address_arg = tb.input(CallArg::Pure(liquor_address.to_vec())).unwrap();
    let debt_reserve_arg = tb.input(  CallArg::Pure(vec![debt_reserve_idx])).unwrap();
    let coll_reserve_arg = tb.input(CallArg::Pure(vec![coll_reserve_idx])).unwrap();
    let debt_pool_arg = tb.input(CallArg::Object(ObjectArg::SharedObject {
        id: debt_pool_id,
        initial_shared_version: debt_pool_isv,
        mutable: true,
    })).unwrap();
    let coll_pool_arg = tb.input(CallArg::Object(ObjectArg::SharedObject {
        id: coll_pool_id,
        initial_shared_version: coll_pool_isv,
        mutable: true,
    })).unwrap();
    let args = vec![clock_arg,
                    oracle_arg,
                    storage_arg,
                    debt_reserve_arg,
                    debt_pool_arg,
                    balance_arg,
                    coll_reserve_arg,
                    coll_pool_arg,
                    address_arg,
                    incentive_arg_v1,
                    incentive_arg,
    ];
    tb.programmable_move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "incentive_v2".parse().unwrap(),
        "liquidation".parse().unwrap(),
        vec![debt_asset.clone(), coll_asset.clone()],
        args,
    );
    return tb
}

// public get_user_assets(Arg0: &Storage, Arg1: address): vector<u8> * vector<u8>
pub fn storage_get_user_assets(mut tb: ProgrammableTransactionBuilder, address: SuiAddress) -> ProgrammableTransactionBuilder {
    let storage = get_storage_arg();
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "storage".parse().unwrap(),
        "get_user_assets".parse().unwrap(),
        vec![],
        vec![
            storage,
            CallArg::Pure(address.to_vec()),
        ]
    );
    return tb;
}

// public get_user_balance(Arg0: &mut Storage, Arg1: u8, Arg2: address): u256 * u256
pub fn storage_get_user_balance(mut tb: ProgrammableTransactionBuilder, asset_index: u8, address: SuiAddress) -> ProgrammableTransactionBuilder {
    let storage = get_storage_arg();
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "storage".parse().unwrap(),
        "get_user_balance".parse().unwrap(),
        vec![],
        vec![
            storage,
            CallArg::Pure(vec![asset_index]),
            CallArg::Pure(address.to_vec()),
        ]
    );
    return tb;
}

// public user_health_factor(Arg0: &Clock, Arg1: &mut Storage, Arg2: &PriceOracle, Arg3: address): u256
pub fn logic_user_health_factor(mut tb: ProgrammableTransactionBuilder, address:SuiAddress) -> ProgrammableTransactionBuilder{
    let clock_arg = CallArg::CLOCK_IMM;
    let oracle_arg = get_oracle_arg();
    let storage_arg = get_storage_arg();
    tb.move_call(
        NAVI_INITIAL_PACKAGE.parse().unwrap(),
        "logic".parse().unwrap(),
        "user_health_factor".parse().unwrap(),
        vec![],
        vec![
            clock_arg, storage_arg, oracle_arg, CallArg::Pure(address.to_vec())
        ],
    );
    return tb;
}

// public user_collateral_balance(Arg0: &mut Storage, Arg1: u8, Arg2: address): u256 {
pub fn logic_user_collateral_balance(mut tb: ProgrammableTransactionBuilder,
                                     reserve_idx: u8,
                                     address:SuiAddress) -> ProgrammableTransactionBuilder{
    let storage_arg = get_storage_arg();
    tb.move_call(
        NAVI_INITIAL_PACKAGE.parse().unwrap(),
        "logic".parse().unwrap(),
        "user_collateral_balance".parse().unwrap(),
        vec![],
        vec![
            storage_arg,
            CallArg::Pure(vec![reserve_idx]),
            CallArg::Pure(address.to_vec())
        ],
    );
    return tb;
}

// public user_collateral_value(Arg0: &Clock, Arg1: &PriceOracle, Arg2: &mut Storage, Arg3: u8, Arg4: address): u256 {
// public user_loan_value(Arg0: &Clock, Arg1: &PriceOracle, Arg2: &mut Storage, Arg3: u8, Arg4: address): u256 {
// public user_loan_balance(Arg0: &mut Storage, Arg1: u8, Arg2: address): u256 {

// lending public flash_loan_with_ctx<Ty0>(Arg0: &Config, Arg1: &mut Pool<Ty0>, Arg2: u64, Arg3: &mut TxContext): Balance<Ty0> * Receipt<Ty0> {
pub fn lending_flash_loan_with_ctx(mut tb: ProgrammableTransactionBuilder, asset: TypeTag,
   pool_id: ObjectID,
    pool_isv: SequenceNumber,
    amount: u64,
) -> ProgrammableTransactionBuilder{
    // let config_arg = get_
    let config_arg = CallArg::Object(SharedObject {
        id: NAVI_FLASHLOAN_CONFIG.parse().unwrap(),
        initial_shared_version: SequenceNumber::from_u64(NAVI_FLASHLOAN_INITIAL_SHARED_VERSION),
        mutable: false,
    });
    let pool_arg = CallArg::Object(SharedObject {
        id: pool_id,
        initial_shared_version: pool_isv,
        mutable: true,
    });
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "lending".parse().unwrap(),
        "flash_loan_with_ctx".parse().unwrap(),
        vec![asset],
        vec![
            config_arg,
            pool_arg,
            CallArg::Pure(amount.to_le_bytes().to_vec()),
        ],
    );
    return tb;
}
// public flash_repay_with_ctx<Ty0>(Arg0: &Clock, Arg1: &mut Storage, Arg2: &mut Pool<Ty0>, Arg3: Receipt<Ty0>, Arg4: Balance<Ty0>, Arg5: &mut TxContext): Balance<Ty0> {
pub fn lending_flash_repay_with_ctx(mut tb: ProgrammableTransactionBuilder,
                                    asset: TypeTag,
                                   pool_id: ObjectID,
                                   pool_isv: SequenceNumber,
    receipt: Argument,
    balance: Argument,
) -> ProgrammableTransactionBuilder{
    // let config_arg = get_
    let clock_arg = tb.input( CallArg::CLOCK_IMM).unwrap();
     let storage = tb.input(get_storage_arg()).unwrap();
    let pool_arg = tb.input(CallArg::Object(SharedObject {
        id: pool_id,
        initial_shared_version: pool_isv,
        mutable: true,
    })).unwrap();
    tb.programmable_move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "lending".parse().unwrap(),
        "flash_repay_with_ctx".parse().unwrap(),
        vec![asset],
        vec![
            clock_arg,
            storage,
            pool_arg,
            receipt,
            balance,
        ],
    );
    return tb;
}

// entry public claim_reward<Ty0>(Arg0: &Clock, Arg1: &mut Incentive, Arg2: &mut IncentiveFundsPool<Ty0>, Arg3: &mut Storage, Arg4: u8, Arg5: u8, Arg6: &mut TxContext) {
pub fn incentive_v2_claim_reward(mut tb: ProgrammableTransactionBuilder, asset: TypeTag,
    asset_id: u8,
    option: u8,
) -> ProgrammableTransactionBuilder {
    tb.move_call(
        NAVI_PACKAGE.parse().unwrap(),
        "lending".parse().unwrap(),
        "claim_reward".parse().unwrap(),
        vec![asset],
        vec![
            CallArg::CLOCK_IMM,
            get_incentive_v2_arg(),
            get_storage_arg(),
            CallArg::Pure(vec![asset_id]),
            CallArg::Pure(vec![option]),
        ],
    );
    return tb;
}