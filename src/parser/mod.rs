use std::{fs, io};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub fn read_log_file(file_path: PathBuf) -> io::Result<()> {
    let mut file = fs::File::open(file_path).unwrap();
    let mut contents = Vec::new();
    let mut position = 0;
    let mut count = 1;

    loop {
        contents.truncate(0);
        file.seek(SeekFrom::Start(position as u64)).expect("Failed to read to position");
        position += file.read_to_end(&mut contents).unwrap();

        let line = String::from_utf8(contents.clone()).unwrap();

        io::stdout().flush().unwrap();
        if !line.is_empty() {
            // io::stdin().read(&mut [0]).unwrap();
            println!("{count}:{}", line.trim_end());
            count+=1;
        }
    }
}
