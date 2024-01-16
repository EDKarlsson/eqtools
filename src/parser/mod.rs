#![allow(unused_imports)]

use std::{fs, io};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn read_log_file(file_path: PathBuf) -> io::Result<()> {
    let mut file = fs::File::open(file_path).unwrap();
    let mut contents = Vec::new();
    let mut position = 0;
    let mut user_input = String::new();

    loop {
        contents.truncate(0);
        file.seek(SeekFrom::Start(position as u64)).expect("Failed to read to position");
        position += file.read_to_end(&mut contents).unwrap();

        // do_process(Contents)
        println!("Read line: {:?}", contents);
        io::stdin().read_line(&mut user_input)?;
        io::stdout().flush().unwrap();
    }
}
