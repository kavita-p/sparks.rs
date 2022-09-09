#[derive(PartialEq, Eq, Debug)]
pub enum RollStatus {
    Crit,
    FullSuccess,
    MixedSuccess,
    Failure
}

#[derive(PartialEq, Eq, Debug)]
pub struct Reply {
    pub title: String,
    pub description: String,
    pub status: RollStatus, 
    pub dice: Vec<u32>
}


mod sparked_interpreter;
mod pbta_interpreter;
mod forged_interpreter;
mod custom_interpreter;
