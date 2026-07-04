use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    MainMap,
    Comms,
    Settings,
    Scenario,
    Defcon,
}

impl Mode {
    pub fn next(&self) -> Self {
        match self {
            Self::MainMap => Self::Comms,
            Self::Comms => Self::Settings,
            Self::Settings => Self::Defcon,
            Self::Defcon => Self::MainMap,
            Self::Scenario => Self::MainMap,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::MainMap => Self::Defcon,
            Self::Comms => Self::MainMap,
            Self::Settings => Self::Comms,
            Self::Defcon => Self::Settings,
            Self::Scenario => Self::MainMap,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainMap => write!(f, "MAIN MAP"),
            Self::Comms => write!(f, "COMMS"),
            Self::Settings => write!(f, "SETTINGS"),
            Self::Scenario => write!(f, "SCENARIO"),
            Self::Defcon => write!(f, "DEFCON"),
        }
    }
}
