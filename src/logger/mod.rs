use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

// Pseudo Logger for testing
pub fn start_logger(file_path: &str, output_file: &str) -> io::Result<()> {
    if let Ok(lines) = read_lines(file_path) {
        let mut log_file = File::open(output_file)?;
        let mut user_input= String::new();
        for line in lines.flatten() {
            println!("{}", line);
            io::stdin().read_line(&mut user_input)?;
            log_file.write(line.as_ref())?;
        }
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}