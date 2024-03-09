use bincode::{deserialize, serialize};
use shared_memory::*;
use std::thread;
use std::time::Duration;

use evdata::SharedQueue;
use std::fs::File;
use std::io::Write;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        // delete file /tmp/rust-shmem
        //std::fs::remove_file("/tmp/rust-shmem").unwrap();
        let _ = std::fs::remove_file("memory_mapping");
    }
}

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
    let _cleanup = Cleanup;
    let _ = std::fs::remove_file("memory_mapping");

    let shmem = create_or_open_shared_memory("memory_mapping").unwrap();

    let shared_queue = SharedQueue { queue: Vec::new() };
    let serialized = serialize(&shared_queue).unwrap();

    unsafe {
        std::ptr::copy_nonoverlapping(
            serialized.as_ptr(),
            shmem.as_ptr() as *mut u8,
            serialized.len(),
        );
    }

    loop {
        unsafe {
            let data_slice = std::slice::from_raw_parts(shmem.as_ptr(), shmem.len());
            if let Ok(mut shared_queue) = deserialize::<SharedQueue>(data_slice) {
                if !shared_queue.queue.is_empty() {
                    let item = shared_queue.queue.remove(0);
                    println!("\nProcessing: {}", item);

                    let serialized = serialize(&shared_queue).unwrap();
                    std::ptr::copy_nonoverlapping(
                        serialized.as_ptr(),
                        shmem.as_ptr() as *mut u8,
                        serialized.len(),
                    );
                } else {
                    print!(".");
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}
