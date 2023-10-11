use std::{fs, io};
use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub(crate) fn process_input_file(folder_path: &Path) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    process_input_file(&entry_path);
                } else if entry_path.is_file() {
                    write_to_summary_file(entry_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
   }

pub(crate) fn write_to_summary_file(file_path: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("weekly_sales_summary.txt")?;
    let content = add_product_lines(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn add_product_lines(file_path: String) -> Result<(String), Box<dyn Error>>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_quantity = 0; // Initialize the total quantity
    let mut first_field = String::new();
    let mut second_field = String::new();
    let mut processed_first_two_fields = false;
    let mut output_string = String::new();

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
        return Ok(result);
    }

    Ok(String::new())
}