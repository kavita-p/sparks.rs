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
    pub dice: Vec<u32>,
}

mod custom_interpreter;
mod forged_interpreter;
mod pbta_interpreter;
mod sparked_interpreter;
