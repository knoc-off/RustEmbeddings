use mlua::{Lua, Result as LuaResult};
use std::{fs, sync::Arc};
use crate::evdev_handler::{EventLoop, KeySender};

pub fn setup_lua_environment(lua: &Lua, key_sender: &Arc<KeySender>) -> LuaResult<()> {
    let key_sender_clone = key_sender.clone();
    lua.globals().set("send_key", lua.create_function(move |_, (key_code, time): (u16, u64)| {
        key_sender_clone.press_key(key_code, 1); // Simulate key press
        std::thread::sleep(std::time::Duration::from_millis(time));
        key_sender_clone.press_key(key_code, 0); // Simulate key release
        Ok(())
    })?)?;

    Ok(())
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let script = fs::read_to_string("script.lua").expect("Failed to read Lua script");

    // Setup virtual device and event loop
    //let virtual_device = VirtualDeviceBuilder::new()?.name("Fake Keyboard").build()?;
    let mut event_loop = EventLoop::new("/dev/input/event1");
    let key_sender = Arc::new(event_loop.get_key_sender());

    setup_lua_environment(&lua, &key_sender)?;

    // Load and execute the Lua script
    lua.load(&script).exec()?;

    // Run the event loop with a callback to handle key events
    event_loop.run(|key_code, value| {
        // Here you can call a Lua function or handle the event in Rust.
        // This is a simplified example. You may need to adapt it to your application's architecture.
        println!("Key event received: code={}, value={}", key_code, value);
        // You can invoke Lua functions here based on the key event
    });

    Ok(())
}

