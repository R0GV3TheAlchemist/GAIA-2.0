//! Golden Compass as questions, not a ranking engine.

use crate::diamond::Gate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinal {
    NorthTruth,
    EastGrowth,
    SouthCare,
    WestBalance,
    CenterWisdom,
}

impl Cardinal {
    pub fn question(self) -> &'static str {
        match self {
            Self::NorthTruth => "What evidence supports this, and what is uncertain?",
            Self::EastGrowth => "What future capability or life may become possible?",
            Self::SouthCare => "Who may be harmed, and what consent is required?",
            Self::WestBalance => "What limits, tradeoffs, and inequities apply?",
            Self::CenterWisdom => "What bounded, reversible, authorized path remains?",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Compass {
    pub north: Gate,
    pub east: Gate,
    pub south: Gate,
    pub west: Gate,
}

impl Compass {
    pub fn center(&self) -> Gate {
        crate::diamond::Alignment {
            truth: self.north,
            care: self.south,
            growth: self.east,
            balance: self.west,
        }
        .wisdom()
    }

    pub fn is_sentient(&self) -> bool {
        false
    }
}
