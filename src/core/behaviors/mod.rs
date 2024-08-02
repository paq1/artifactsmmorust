use crate::core::behaviors::go_deposit_bank::GoDepositBankBehavior;
use crate::core::behaviors::go_infinit_gathering::GoInfinitGateringBehavior;
use crate::core::errors::Error;

pub mod infinit_fight;
pub mod infinit_gathering;
pub mod go_deposit_bank;
pub mod go_infinit_gathering;


#[derive(Clone)]
pub enum Behavior {
    GoDepositBankBehavior(GoDepositBankBehavior),
    GoInfinitGateringBehavior(GoInfinitGateringBehavior),
}

impl Behavior {
    pub async fn next_behavior(&self) -> Result<Option<Behavior>, Error> {
        match self {
            Behavior::GoDepositBankBehavior(b) => b.next_behavior().await,
            Behavior::GoInfinitGateringBehavior(b) => b.next_behavior().await
        }
    }
}
