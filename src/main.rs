/*
mod evdev_handler;
mod lua_integration;

use evdev_handler::EventLoop;
use lua_integration::setup_lua_environment;
use mlua::Function;
use mlua::Lua;
use std::sync::Arc;

use std::thread::sleep;
use std::time::Duration;
*/
fn main() -> mlua::Result<()> {
   Ok(()) // Remove this line after implementing the main function
}
/*
    let lua = Lua::new();
    let script = std::fs::read_to_string("script.lua").expect("Failed to read Lua script");

    // Create the event loop with the virtual device.
    let mut event_loop = EventLoop::new("/dev/input/event1");

    // Wrap KeySender in Arc for shared access between threads.
    let key_sender = Arc::new(event_loop.get_key_sender());

    // Setup Lua environment with access to `key_sender`.
    setup_lua_environment(&lua, &key_sender)?;

    // Load and execute the Lua script.
    lua.load(&script).exec()?;
    //lua.load(&mut std::fs::File::open("script.lua"));
    //lua.call("my_func", (), None)?;
    //lua.load("my_func()").exec()?;

    sleep(Duration::from_millis(50));
    // Run the event loop in a separate thread if necessary.
    std::thread::spawn(move || {
        event_loop.run(move |key_code, value| {
            let lua = Lua::new(); // Create a new Lua state for thread safety
            lua.load(&script)
                .exec()
                .expect("Failed to load Lua script in thread");
            let _ = setup_lua_environment(&lua, &key_sender);

            // Here, you could call back into Lua or handle the key event in Rust.
            // This example simply prints the event; replace with your logic.
            //println!("Key event received: code={}, value={}", key_code, value);
            //key_sender.press_key(key_code, value);

            let on_keypress: Function = lua
                .globals()
                .get("on_keypress")
                .expect("Failed to find Lua function 'on_keypress'");

            // Call the Lua function with the key code and value
            on_keypress
                .call::<_, ()>((key_code, value))
                .expect("Failed to call 'on_keypress'");

            //lua.call("my_func", (), None)?;

            // run the lua function on key press
            //lua.load("my_func()").exec()?;

            //lua.globals().get::<_, mlua::Function>("on_keypress").unwrap().call::<_, ()>((key_code, value));

            // press key
            // if key == escape exit
            if key_code == 1 {
                std::process::exit(0);
            }

            //device = key_sender.device.clone();
        });
    });

    // Keep the main thread alive or proceed with other logic.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
*/
