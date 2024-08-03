#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InfinitFightStates {
    Empty,
    GoingFight,
    EndFight,
    FullInventory,
    GoingBank,
    Deposit,
}
