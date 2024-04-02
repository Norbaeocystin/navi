use serde::{Deserialize, Serialize};
use sui_types::base_types::SuiAddress;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositEvent {
    pub reserve: u8,
    pub sender:SuiAddress,
    pub amount: u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolDeposit {
    pub sender:SuiAddress,
    pub amount: u64,
    pub pool: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolWithdraw {
    pub sender:SuiAddress,
    pub recipient: SuiAddress,
    pub amount: u64,
    pub pool: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawEvent {
    pub reserve: u8,
    pub sender:SuiAddress,
    pub to: SuiAddress,
    pub amount: u64
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowEvent {
    pub reserve: u8,
    pub sender: SuiAddress,
    pub amount: u64
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepayEvent {
    pub reserve: u8,
    pub sender: SuiAddress,
    pub amount: u64
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationCallEvent {
    pub reserve: u8,
    pub sender: SuiAddress,
    pub liquidate_user: SuiAddress,
    pub liquidate_amount: u64
}



#[derive(Debug, Serialize, Deserialize)]
struct SetCenterPrice {
    pub prices: Vec<u64>,
    pub timestamps: Vec<u64>
}

#[derive(Debug, Deserialize)]
struct Prices {
    pub prices: Vec<[u8;32]>
}

#[derive(Debug, Deserialize)]
struct Timestamps {
    pub timestamps: Vec<u64>
}