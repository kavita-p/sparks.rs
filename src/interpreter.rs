use core::fmt;
use std::fmt::Display;
use strum_macros::EnumString;

#[derive(PartialEq, Eq, Debug)]
pub enum RollStatus {
    Crit,
    FullSuccess,
    MixedSuccess,
    Failure,
}

#[derive(PartialEq, Eq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ConfidenceLevel {
    Confidence,
    Desperation,
}

impl Display for ConfidenceLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Confidence => write!(f, "confidence"),
            Self::Desperation => write!(f, "desperation"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ForgedType {
    Action,
    Resist,
    Fortune,
    Clear,
}

#[derive(PartialEq, Eq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
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
            Self::Action => write!(f, "Action"),
            Self::Attack => write!(f, "Attack"),
            Self::Defense => write!(f, "Defense"),
            Self::Acquisition => write!(f, "Acquisition"),
            Self::Creation => write!(f, "Creation"),
            Self::Recovery => write!(f, "Recovery"),
            Self::Ratings => write!(f, "Ratings"),
            Self::Watch => write!(f, "Watch"),
            Self::Weather => write!(f, "Weather-watching"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Reply {
    pub title: String,
    pub description: String,
    pub status: RollStatus,
    pub dice: String,
    pub text: Option<String>,
}

pub mod custom;
pub mod fitd;
pub mod pbta;
pub mod sbr;
pub mod ww;
