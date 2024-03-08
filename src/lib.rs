use serde::{Serialize, Deserialize};


// key press detection, and interupting the key press
#[derive(Serialize, Deserialize, Debug)]
pub struct Keypress {
    code: u16,
    value: i32,
}

