#![allow(dead_code)]

use std::env;
use std::path::PathBuf;
use tokio::signal;

use crate::logger::start_logger;
use crate::parser::read_log_file;

mod sandbox;
mod parser;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // This will be replaced by a config file or a commandline argument
    let cur_dir = env::current_dir().unwrap();
    let file_path: PathBuf = [cur_dir.to_str().unwrap(), "resources", "test_eqlog.txt"].iter().collect();
    let log_file: PathBuf = [cur_dir.to_str().unwrap(), "out", "eq_log_file.txt"].iter().collect();

    // TODO: Add a sigint catch
    println!("Current dir: {}\nFile path: {}", cur_dir.display(), file_path.display());

    if &args[1].to_lowercase() == "logger" {
        // Read test log file and write each line stepwise using enter to proceed
        let _ = start_logger(file_path, log_file);
    } else {
        println!("Starting parser");
        match read_log_file(log_file) {
            Ok(_) => println!("Didn't crash"),
            Err(_) => println!("Error"),
        }
    }
    // TODO: Sigint-catch should rotate the logs to make parsing easier with each play session
    signal::ctrl_c().await?;
    println!("ctrl-c received");
    Ok(())

}