use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// Pseudo Logger for testing
pub fn start_logger(file_path: &str) -> io::Result<()> {
    if let Ok(lines) = read_lines(file_path) {
        let mut buffer= String::new();
        for line in lines.flatten() {
            println!("{}", line);
            io::stdin().read_line(&mut buffer)?;
        }
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}