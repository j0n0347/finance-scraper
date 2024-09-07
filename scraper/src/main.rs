use csv_processor::{process_quarter, process_yearly};
use python::test_run;
use std::cmp::Ordering;
use std::env;
use std::path::PathBuf;

fn parse_config(args: &[String]) -> (i32, i32) {
    if args.len() < 3 {
        panic!("not enough arguments specified")
    }

    let year = match args[1].parse::<i32>() {
        Ok(year) => year,
        Err(_) => panic!("plase enter in a valid year"),
    };

    if (year < 2001) || (year > 2024) {
        panic!("please enter in a number between 2001-present")
    }

    let quarter = match args[2].parse::<i32>() {
        Ok(quarter) => quarter,
        Err(_) => panic!("please parse in a valid quarter"),
    };

    if quarter > 5 || quarter < 1 {
        panic!("enter in quarters 1 to 4");
    }

    (year, quarter)
}

fn main() {
    println!("---------------------------------");
    println!("-        Finance Scraper        -");
    println!("---------------------------------");

    println!("---------------------------------");

    let args: Vec<String> = env::args().collect();

    let (year, quarter) = parse_config(&args);

    match test_run((year, quarter)) {
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
