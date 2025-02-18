use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use fmc_protocol_derive::ClientBound;

#[derive(ClientBound, Event, Serialize, Deserialize, Debug, Clone)]
pub enum Plugin {
    Enable(String),
    Disable(String),
}
