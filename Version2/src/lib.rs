use std::{fs, io, thread};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::sync::{Arc, mpsc, Mutex};
use log::{error, info};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::Sender;


pub(crate) fn process_input_file(folder_path: &Path, summary: Arc<Mutex<String>>) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        let mut handles = vec![];

        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let summary_clone = Arc::clone(&summary);
                    let handle = thread::spawn(move || {
                        process_input_file(&entry_path, summary_clone);
                    });
                    handles.push(handle);
                } else if entry_path.is_file() {
                    write_to_summary_file(entry_path.to_string_lossy().into_owned(), summary.clone());
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub(crate) fn write_to_summary_file(file_path: String, summary: Arc<Mutex<String>>) {
    let content = add_product_lines(file_path);
    if let Ok(content) = content {
        let mut summary = summary.lock().unwrap();
        summary.push_str(&content);
    }
}
pub(crate) fn write_summary_to_file(file_path: &str, summary: Arc<Mutex<String>>) {
    let summary = summary.lock().unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open summary file");
    file.write_all(summary.as_bytes())
        .expect("Failed to write summary to file");
}


pub(crate) fn add_product_lines(file_path: String) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_quantity = 0;
    let mut first_field = String::new();
    let mut second_field = String::new();
    let mut processed_first_two_fields = false;

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() >= 3 {
            let quantity = fields[2].trim().parse::<i32>().unwrap_or(0);
            total_quantity += quantity;

            if !processed_first_two_fields {
                first_field = fields[0].trim().to_string();
                second_field = fields[1].trim().to_string();
                processed_first_two_fields = true;
            }
        }
    }

    if processed_first_two_fields {
        let result = format!("{}, {}, {} \n", first_field, second_field, total_quantity);
        log_to_file("log.txt", &result).expect("Failed to write log entry.");
        info!("{}", result);
        return Ok(result);
    }

    Ok(String::new())
}

fn log_to_file(file_path: &str, log_entry: &str) -> Result<(), std::io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", log_entry)?;

    Ok(())
}
