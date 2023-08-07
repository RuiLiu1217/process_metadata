// Define the event
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Event {
    pub pid: u32,
    pub uid: u32,
    pub exe: String,
    pub idx: u32,
}