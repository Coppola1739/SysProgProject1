use std::env;
use::std::io;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::time::{Instant, Duration};

fn main() {
    println!("Please enter the data folder path: ");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read path");

    let path = Path::new(file_path.trim());

    let output_file_name = "weekly_sales_summary.txt";
    if !file_exists(output_file_name) {
        let mut file = File::create(output_file_name);
    }

    let start_time = Instant::now();


    lib::process_input_file(&path);

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Phew! I am done");
    println!("Time to complete: {:?}", elapsed_time.as_nanos());
}

fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}
