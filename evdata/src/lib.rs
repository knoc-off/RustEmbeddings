use serde::{Deserialize, Serialize};
// mutex
use std::sync::Mutex;
use std::collections::HashMap;

// key press detection, and interupting the key press
#[derive(Serialize, Deserialize, Debug)]
pub struct Keypress {
    pub code: u16,
    pub value: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SharedQueue {
    pub queue: Vec<String>,
}

// about 8192 bytes of shared memory
#[derive(Serialize, Deserialize, Debug)]
pub struct SharedQueueMutex {
    // keys that need to be simulated
    pub to_press_queue: Mutex<Vec<Keypress>>,
    // keys that are currently being pressed
    pub pressed_queue: Mutex<Vec<Keypress>>,
    // keys that are currently being pressed, but have been filtered by the hashmap
    pub pressed_filtered_queue: Mutex<Vec<Keypress>>,
    // keys that will be processed, so should not be auto sent to the to_press_queue
    pub processed_keys_hashmap: Mutex<HashMap<u16, bool>>,
}
