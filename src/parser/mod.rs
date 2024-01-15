#![allow(unused_imports)]
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use tokio::time;
use std::time::Duration;

// pub async fn read_log_file(file_path: &str) {
//     let mut file = fs::File::open(file_path).unwrap();
//     let mut interval = time::interval(Duration::from_millis(1000));
//     let mut contents = vec![];
//     let mut position = 0;

    // loop {
    //     contents.truncate(0);
    //     // file.seek(SeekFrom::Start(position as u64));
    //     position += file.read_to_end(&mut contents).unwrap();
    //
    //     // do_process(Contents)
    //
    //     interval.tick().await;
    // }
// }