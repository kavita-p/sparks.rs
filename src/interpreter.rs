use core::fmt;
use std::fmt::Display;

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
pub enum WildType {
    Action,
    Attack,
    Defense,
    Acquisition,
    Creation,
    Recovery,
    Ratings,
    Watch,
    Weather,
}

impl Display for WildType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WildType::Action => write!(f, "Action"),
            WildType::Attack => write!(f, "Attack"),
            WildType::Defense => write!(f, "Defense"),
            WildType::Acquisition => write!(f, "Acquisition"),
            WildType::Creation => write!(f, "Creation"),
            WildType::Recovery => write!(f, "Recovery"),
            WildType::Ratings => write!(f, "Ratings"),
            WildType::Watch => write!(f, "Watch"),
            WildType::Weather => write!(f, "Weather-watching"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Reply {
    pub title: String,
    pub description: String,
    pub status: RollStatus,
    pub dice: String,
}

pub mod custom;
pub mod fitd;
pub mod pbta;
pub mod sbr;
pub mod ww;
