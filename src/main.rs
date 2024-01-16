#![allow(dead_code)]
use std::env;

use crate::logger::start_logger;

mod sandbox;
mod parser;
mod logger;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // This will be replaced by a config file or a commandline argument
    let file_path = "/Users/dank/git/edkarlsson/eqtools/resources/example_logs/test_eqlog.txt";
    let path = env::current_dir().unwrap();
    println!("File path: {file_path}, current dir: {:?}", path);

    if &args[1].to_lowercase() == "logger" {
        println!("Starting test logger");
        // Read test log file and write each line stepwise using enter to proceed
        let log_file = format!("{}/eq_log_file.txt", path.to_str().unwrap());
        println!("Test output log file: {}", log_file);
        let _ = start_logger(file_path, log_file.as_str());
    } else {
        println!("Starting parser");
    }
    Ok(())
}

/*
// use std::error::Error;
// use crate::parser::read_log_file;

// Tokio impl
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let log_file_name = "/Users/dank/git/edkarlsson/eqtools/resources/example_logs/test_eqlog.txt";
    read_log_file(log_file_name).await?;
    Ok(())
}
*/
/*#[async_std::main]
//async fn main() -> Result<(), Box<dyn Error>> {
//    let _ = server::p2p::main();
//    Ok(())
//}
*/