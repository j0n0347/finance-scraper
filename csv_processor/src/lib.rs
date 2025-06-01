#![allow(unused_imports)]
use anyhow::Result;
use csv::*;
use serde::Serialize;
use std::borrow::Borrow;
use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct QuartRecord {
    name: String,
    current_quarter: Option<i32>,
    previous_quarter: Option<i32>,
}

#[derive(Serialize)]
pub struct YearRecord {
    name: String,
    first_year: Option<i32>,
    second_year: Option<i32>,
    thrid_year: Option<i32>,
}

fn collect_entries(path: &PathBuf) -> Result<Vec<DirEntry>> {
    let entries = std::fs::read_dir(path)?.filter_map(Result::ok).collect();

    Ok(entries)
}

pub fn process_quarter(csv_file: &PathBuf) -> Result<()> {
    let files = collect_entries(csv_file).expect("read_dir call failed");
    for file in &files {
        let file_str = file.file_name().into_string().unwrap();
        if file_str.contains("_cleaned.csv") {
            panic!("quarter has already been cleaned");
        }
    }

    for file in files {
        println!("{:?}", file.file_name());
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(file.path())?;
        let mut records: Vec<QuartRecord> = vec![];

        for (index, result) in rdr.records().enumerate() {
            let record = result?;

            // println!("{:?}", record);
            if index < 2 {
                continue;
            }

            if record.len() > 1 {
                let original_string1: String = record.get(1).unwrap_or(&"".to_string()).to_string();
                let cleaned_string1 = original_string1
                    .replace(",", "")
                    .replace(")", "")
                    .replace("(", "-");
                let trimmed1 = cleaned_string1.trim_end();
                let to_int1 = trimmed1.parse::<i32>().ok();

                let original_string2: String = record.get(2).unwrap_or(&"".to_string()).to_string();
                let cleaned_string2 = original_string2
                    .replace(",", "")
                    .replace(")", "")
                    .replace("(", "-");
                let trimmed2 = cleaned_string2.trim_end();
                let to_int2 = trimmed2.parse::<i32>().ok();

                let row = QuartRecord {
                    name: record.get(0).unwrap().to_string(),
                    current_quarter: to_int1,
                    previous_quarter: to_int2,
                };
                records.push(row);
            } else {
                let single = record.get(0).unwrap_or(&"".to_string()).to_string();

                let row = QuartRecord {
                    name: single,
                    current_quarter: None,
                    previous_quarter: None,
                };
                records.push(row)
            }
        }
        let mut filename = file
            .file_name()
            .into_string()
            .expect("error converting to stirng");
        filename.truncate(filename.len() - 4);

        let file_path = format!("{}{}_cleaned.csv", csv_file.display(), filename);
        println!("{}{}", csv_file.display(), filename);
        let file = File::create(file_path)?;
        let mut wtr = csv::Writer::from_writer(file);

        for record in records {
            wtr.serialize(record)?;
        }
    }

    Ok(())
}

pub fn process_yearly(csv_file: &PathBuf) -> Result<()> {
    let files = collect_entries(csv_file)?;
    for file in &files {
        let file_str = file.file_name().into_string().unwrap();
        if file_str.contains("_cleaned.csv") {
            panic!("quarter has already been cleaned");
        }
    }

    for file in files {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(file.path())?;
        let mut records: Vec<YearRecord> = vec![];

        for (index, result) in rdr.records().enumerate() {
            let record = result?;

            // println!("{:?}", record);
            if index < 2 {
                continue;
            }

            if record.len() > 1 {
                let original_string1: String = record.get(1).unwrap_or(&"".to_string()).to_string();
                let cleaned_string1 = original_string1
                    .replace(",", "")
                    .replace(")", "")
                    .replace("(", "-");
                let trimmed1 = cleaned_string1.trim_end();
                let to_int1 = trimmed1.parse::<i32>().ok();

                let original_string2: String = record.get(2).unwrap_or(&"".to_string()).to_string();
                let cleaned_string2 = original_string2
                    .replace(",", "")
                    .replace(")", "")
                    .replace("(", "-");
                let trimmed2 = cleaned_string2.trim_end();
                let to_int2 = trimmed2.parse::<i32>().ok();

                let original_string3: String = record.get(3).unwrap_or(&"".to_string()).to_string();

                let cleaned_string3 = original_string3
                    .replace(",", "")
                    .replace(")", "")
                    .replace("(", "-");
                let trimmed3 = cleaned_string3.trim_end();
                let to_int3 = trimmed3.parse::<i32>().ok();
                // println!("{:?}", cleaned_string3.trim_end());
                let row = YearRecord {
                    name: record.get(0).unwrap().to_string(),
                    first_year: to_int1,
                    second_year: to_int2,
                    thrid_year: to_int3,
                };
                records.push(row);
            } else {
                let single = record.get(0).unwrap_or(&"".to_string()).to_string();

                let row = YearRecord {
                    name: single,
                    first_year: None,
                    second_year: None,
                    thrid_year: None,
                };
                records.push(row)
            }
        }
        let mut filename = file
            .file_name()
            .into_string()
            .expect("error converting to stirng");
        filename.truncate(filename.len() - 4);

        let file_path = format!("{}{}_cleaned.csv", csv_file.display(), filename);
        println!("{}{}", csv_file.display(), filename);
        let file = File::create(file_path)?;
        let mut wtr = csv::Writer::from_writer(file);

        for record in records {
            wtr.serialize(record)?;
        }
    }

    Ok(())
}

// #[cfg(test)]
// mod test {
//     fn 
// }
