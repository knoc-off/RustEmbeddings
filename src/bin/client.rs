use bincode::{deserialize, serialize};
use shared_memory::*;
use std::fs::File;
use std::io::Read;

use evdata::SharedQueue;

fn create_or_open_shared_memory( flink :&str ) -> Result<Shmem, ShmemError> {
    // Create or open the shared memory mapping
    let shmem = match ShmemConf::new().size(8192).flink(flink).create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(flink).open().unwrap(),
        Err(e) => {
            eprintln!("Unable to create or open shmem flink {flink} : {e}");
            return Err(e);
        }
    };

    Ok(shmem)
}

fn main() {
    // socket connection
    //let comms = "/tmp/rust-uds.sock";
    //use std::os::unix::net::UnixStream;

    let shmem = create_or_open_shared_memory("memory_mapping").unwrap();
    //ShmemConf::new()
    //    .os_id(&id)
    //    .open()
    //    .expect("Failed to open shared memory");

    println!("Connected to shared memory with id: {}", shmem.get_os_id());


    unsafe {
        let data_slice = std::slice::from_raw_parts(shmem.as_ptr(), shmem.len());
        if let Ok(mut shared_queue) = deserialize::<SharedQueue>(data_slice) {
            shared_queue.queue.push("Hello from client!".to_string());

            let serialized = serialize(&shared_queue).unwrap();
            if serialized.len() > shmem.len() {
                panic!("Shared memory is not large enough to hold the updated data.");
            }
            std::ptr::copy_nonoverlapping(serialized.as_ptr(), shmem.as_ptr() as *mut u8, serialized.len());
        } else {
            println!("Failed to deserialize shared queue.");
        }
    }
}

