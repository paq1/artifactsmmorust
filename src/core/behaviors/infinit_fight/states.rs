#[derive(Clone, Debug)]
pub enum InfinitFightStates {
    Empty,
    GoingFight,
    EndFight,
    FullInventory,
    GoingBank,
    Deposit,
}
