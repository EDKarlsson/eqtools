#![allow(dead_code)]

use std::env;
use std::path::PathBuf;

use crate::logger::start_logger;

mod sandbox;
mod parser;
mod logger;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // This will be replaced by a config file or a commandline argument
    // Macbook
    // let file_path = "/Users/dank/git/edkarlsson/eqtools/resources/example_logs/test_eqlog.txt";
    let cur_dir = env::current_dir().unwrap();
    /*    let file_path = Path::new(".")
            .join("resources")
            .join("example_logs")
            .join("test_eqlog")
            .set_extension("txt");
    */
    let file_path: PathBuf = [cur_dir.to_str().unwrap(), "resources", "test_eqlog.txt"].iter().collect();

    println!("Current dir: {}\nFile path: {}", cur_dir.display(), file_path.display());

    if &args[1].to_lowercase() == "logger" {
        // Read test log file and write each line stepwise using enter to proceed
        let log_file: PathBuf = [cur_dir.to_str().unwrap(), "out", "eq_log_file.txt"].iter().collect();
        let _ = start_logger(file_path, log_file);
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