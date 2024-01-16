use std::{env, process};
use std::path::PathBuf;
use tokio::signal;

use crate::parser::read_log_file;

mod sandbox;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create a signal capture to exit app gracefully
    tokio::spawn(async {
        match signal::ctrl_c().await {
            Ok(()) => {
                println!("\nClosing eqtools, rotating logs.");
                process::exit(0);
            },
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
                process::exit(1);
            }
        }
    });

    // This will be replaced by a config file or a commandline argument
    let mut cur_dir = env::current_dir().unwrap();
    while !cur_dir.ends_with("eqtools") {
        cur_dir.pop();
    }

    let file_path: PathBuf = [cur_dir.to_str().unwrap(), "resources", "test_eqlog.txt"].iter().collect();
    let log_file: PathBuf = [cur_dir.to_str().unwrap(), "out", "eq_log_file.txt"].iter().collect();

    println!("EQTools Info");
    println!("\tRoot dir:\t{}", cur_dir.display());
    println!("\tInput:\t\t{}", file_path.display());

    println!("Starting EQTools: parser");
    match read_log_file(log_file) {
        Ok(_) => println!("Didn't crash"),
        Err(why) => println!("Error: {}", why),
    }

    // TODO: Sigint-catch should rotate the logs to make parsing easier with each play session
    signal::ctrl_c().await?;
    println!("ctrl-c received");
    Ok(())
}