#[derive(PartialEq, Eq, Debug)]
pub enum RollStatus {
    Crit,
    FullSuccess,
    MixedSuccess,
    Fail
}

#[derive(PartialEq, Eq, Debug)]
pub struct Reply {
    pub title: String,
    pub description: String,
    pub status: RollStatus, 
    pub dice: Vec<u32>
}


mod sparked_interpreter;

