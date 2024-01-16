use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

// Pseudo Logger for testing
pub fn start_logger(file_path: PathBuf, output_file: PathBuf) -> io::Result<()> {
    println!("Starting Test Logger: {}\n\tWriting to: {}", file_path.display(), output_file.display());
    let op_display = output_file.display();
    if let Ok(lines) = read_lines(file_path.to_str().unwrap()) {
        let mut log_file = match File::create(&output_file) {
            Err(why) => panic!("couldn't create {}: {}", op_display, why),
            Ok(log_file) => log_file,
        };

        let mut user_input = String::new();
        for (i, mut line) in lines.flatten().enumerate() {
            io::stdin().read_line(&mut user_input)?;
            line.push('\n');
            match log_file.write_all(line.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", op_display, why)
                }
                Ok(_) => {
                    io::stdout().flush().unwrap();
                    let _char = line.pop();
                    print!("{i}: {}", line)
                }
            }
        }
    }
    println!("Done writing");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}