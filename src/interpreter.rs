use core::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum RollStatus {
    Crit,
    FullSuccess,
    MixedSuccess,
    Failure,
}

#[derive(PartialEq, Eq, Debug, poise::ChoiceParameter)]
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

#[derive(PartialEq, Eq, Debug, poise::ChoiceParameter)]
pub enum ForgedType {
    #[name = "action"]
    Action,
    #[name = "resist"]
    Resist,
    #[name = "fortune/downtime"]
    Fortune,
    #[name = "clear stress"]
    Clear,
}

#[derive(PartialEq, Eq, Debug, poise::ChoiceParameter)]
pub enum WildType {
    #[name = "Action"]
    Action,
    #[name = "Attack"]
    Attack,
    #[name = "Defense"]
    Defense,
    #[name = "Acquisition"]
    Acquisition,
    #[name = "Creation"]
    Creation,
    #[name = "Recovery"]
    Recovery,
    #[name = "Ratings"]
    Ratings,
    #[name = "Watch"]
    Watch,
    #[name = "Weather-watching"]
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
