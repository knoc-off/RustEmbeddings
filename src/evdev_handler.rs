use evdev::{uinput::VirtualDevice, uinput::VirtualDeviceBuilder, Device, EventType, InputEvent};
//use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        eprintln!("Doing some final cleanup");
        Device::open("/dev/input/event1").unwrap().ungrab().unwrap();
    }
}

#[derive(Clone)]
pub struct KeySender {
    device: Arc<Mutex<VirtualDevice>>,
}

impl KeySender {
    pub fn new(input_device: &Device) -> Self {
        let keys = input_device
            .supported_keys()
            .expect("Failed to get supported keys");

        KeySender {
            device: Arc::new(Mutex::new(
                VirtualDeviceBuilder::new()
                    .unwrap()
                    .name("Fake Keyboard")
                    .with_keys(keys)
                    .unwrap()
                    .build()
                    .unwrap(),
            )),
        }
    }

    pub fn press_key(&self, key_code: u16, state: i32) {
        let device = self.device.clone();
        let key_event = InputEvent::new(EventType::KEY, key_code.into(), state);
        if let Err(e) = device.lock().unwrap().emit(&[key_event]) {
            eprintln!("Error emitting key event: {:?}", e);
        };
    }
}

pub struct EventLoop {
    key_sender: KeySender,
    input_device: Device,
}

impl EventLoop {
    pub fn new(device_path: &str) -> Self {
        let input_device = Device::open(device_path).expect("Failed to open input device");
        let key_sender = KeySender::new(&input_device);
        EventLoop {
            key_sender,
            input_device,
        }
    }

    pub fn run(&mut self, callback: impl Fn(u16, i32) + Sync + Send + 'static) {
        // grab the input device
        self.input_device.grab().unwrap();
        loop {
            let events = self.input_device.fetch_events().unwrap();
            for ev in events {
                if ev.event_type() == EventType::KEY {
                    callback(ev.code(), ev.value());
                    //self.key_sender.press_key(ev.code(), ev.value());
                }
            }
            sleep(Duration::from_millis(10)); // Small delay to reduce CPU usage
        }
    }

    pub fn get_key_sender(&self) -> KeySender {
        self.key_sender.clone()
    }
}
