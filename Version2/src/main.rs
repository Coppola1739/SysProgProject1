use std::env;
use::std::io;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use crate::lib::{process_input_file, write_summary_to_file};

fn main() {
    env_logger::init();
    println!("Please enter the data folder path: ");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read path");

    let path = Path::new(file_path.trim());


    let summary = Arc::new(Mutex::new(String::new()));

    let start_time = Instant::now();

    process_input_file(&path, summary.clone());

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    write_summary_to_file("weekly_sales_summary.txt", summary);
    println!("Phew! I am done");
    println!("Time to complete: {:?}", elapsed_time.as_nanos());
}

fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}
