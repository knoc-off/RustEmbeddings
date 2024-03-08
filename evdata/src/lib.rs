use serde::{Deserialize, Serialize};

// key press detection, and interupting the key press
#[derive(Serialize, Deserialize, Debug)]
pub struct Keypress {
    pub code: u16,
    pub value: i32,
}
