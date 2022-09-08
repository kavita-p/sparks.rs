use sparksrs::Rolls;

pub struct Reply {
    pub title: String,
    pub description: String,
    pub status: String,
    pub dice: Vec<u32>
    
}

mod sparked_interpreter;

pub fn sparked_interpreter(roll: Rolls) -> Reply {
    Reply {
        title: String::from("Sample title"),
        description: String::from("Sample description"),
        status: String::from("Sample status"),
        dice: vec![1, 2, 3]
    }
}
