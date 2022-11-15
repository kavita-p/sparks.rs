#[derive(PartialEq, Eq, Debug)]
pub enum RollStatus {
    Crit,
    FullSuccess,
    MixedSuccess,
    Failure,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ForgedType {
    Action,
    Resist,
    Fortune,
    Clear,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Reply {
    pub title: String,
    pub description: String,
    pub status: RollStatus,
    pub dice: String,
}

pub mod custom;
pub mod forged;
pub mod pbta;
pub mod sparked;
