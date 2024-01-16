use std::{env, io, process};
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use tokio::signal;

/// Logger: main
///
/// Pseudo Logger for testing
///
#[tokio::main]
async fn main() -> io::Result<()> {

    // Create a signal capture to exit app gracefully
    tokio::spawn(async {
        match signal::ctrl_c().await {
            Ok(()) => {
                println!("\nShutting Logger down..");
                process::exit(0);
            },
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
                process::exit(1);
            }
        }
    });

    println!("Done writing");
    // Get current dir and make sure that it's in the project root
    //  TODO: Replace with a more dynamic way to load files from the system, like args or env file
    let mut cur_dir = env::current_dir().unwrap();
    while !cur_dir.ends_with("eqtools") {
        cur_dir.pop();
    }

    let test_input_file: PathBuf = [cur_dir.to_str().unwrap(), "resources", "test_eqlog.txt"].iter().collect();
    let output_file: PathBuf = [cur_dir.to_str().unwrap(), "out", "eq_log_file.txt"].iter().collect();

    println!("Logger Info");
    println!("\tRoot dir:\t{}", cur_dir.display());
    println!("\tInput:\t\t{}", test_input_file.display());
    println!("\tOutput:\t\t{}", output_file.display());

    let op_display = output_file.display();
    if let Ok(lines) = read_lines(test_input_file.to_str().unwrap()) {
        let mut log_file = match File::create(&output_file) {
            Err(why) => panic!("couldn't create {}: {}", op_display, why),
            Ok(log_file) => log_file,
        };

        for (i, mut line) in lines.flatten().enumerate() {
            // Allows to step through the log file by reading input
            io::stdout().flush().unwrap();
            io::stdin().read(&mut [0]).unwrap();
            line.push('\n');
            match log_file.write_all(line.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", op_display, why)
                }
                Ok(_) => {
                    let _char = line.pop();
                    print!("{i}: {}", line);
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}