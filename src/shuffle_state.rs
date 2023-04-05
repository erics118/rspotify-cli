use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, clap::ValueEnum, Clone, PartialEq, Eq)]
pub enum ShuffleState {
    Enabled,
    Disabled,
}

impl From<bool> for ShuffleState {
    fn from(val: bool) -> Self {
        if val { Self::Enabled } else { Self::Disabled }
    }
}

impl From<ShuffleState> for bool {
    fn from(val: ShuffleState) -> Self {
        match val {
            ShuffleState::Enabled => true,
            ShuffleState::Disabled => false,
        }
    }
}
