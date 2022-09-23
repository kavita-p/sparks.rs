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
    pub dice: Vec<u64>,
}

pub mod custom_interpreter;
pub mod forged_interpreter;
pub mod pbta_interpreter;
pub mod sparked_interpreter;
