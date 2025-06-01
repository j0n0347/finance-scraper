use csv_processor::{process_quarter, process_yearly};
use python::run;
use std::env;
use std::path::PathBuf;
use dotenv::dotenv;
use logic::*;

pub mod logic;

fn main() {
    println!("---------------------------------");
    println!("-        Finance Scraper        -");
    println!("---------------------------------");

    println!("---------------------------------");

    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let (year, quarter) = parse_config(&args).unwrap();

    let current_dir = std::env::var("CURRENT_DIR").unwrap();

    let path = PathBuf::from(format!("{}/output/AAPL/{}/Q{}/",current_dir, &year, &quarter));

    if is_scraped(path) {
        println!("year/quart has already been scraped");
        println!("exiting application!");
        return;
    }
    match run((year, quarter)) {
        Ok(()) => println!("scraping successfull"),
        Err(e) => panic!("Error scraping: {}", e),
    }

    let path = PathBuf::from(format!("output/AAPL/{}/Q{}/", year, quarter));
    if quarter < 4 {
        match process_quarter(&path) {
            Ok(()) => println!("csv processed successfull"),
            Err(e) => panic!("error while processing csv: {}", e),
        }
    } else {
        match process_yearly(&path) {
            Ok(()) => println!("csv processed successfull"),
            Err(e) => panic!("error while processing csv: {}", e),
        }
    }
}
