#[derive(Clone, Debug)]
pub enum InfinitGateringStates {
    Empty,
    GoingGathering,
    EndGathering,
    FullInventory,
    GoingBank,
    Deposit,
}
