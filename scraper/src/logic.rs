use csv_processor::{process_quarter, process_yearly};
use python::run;
use std::env;
use std::path::PathBuf;

use anyhow::{Error, Result};

pub fn run_menu(display_options: bool, (year, quarter): (i32, i32)) {

    println!("---------------------------------");
    println!("-        Finance Scraper        -");
    println!("---------------------------------");

    println!("---------------------------------");

    if display_options {
        if !check_output() {
            println!("-     no scraped information       -");
            println!("-re-run application with parameters-");
            println!("-or select automatic scrape options-");
            println!("-        Scraping Options          -");
            println!("-1: APPLE DEFAULT SCRAPE           -");
            println!("-2: EXIT APPLICATION               -");

            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let menu_option: i32 = line.trim().parse().unwrap_or(-1); 

            match menu_option {
                1 => {
                    println!("APPLE SCRAPE SELECTED  ");
                    println!("this may take some time");
                    match default_scrape_appl(){
                        Ok(()) => {
                            println!("performed default scrape of data successfully");
                            return;
                        }
                        Err(e) => panic!("error occured during default scrape: {}", e)
                    }
                },
                2 => {
                    println!("Exiting application!!");
                    return;
                }
                _ => {
                    println!("please select valid options");
                    return;
                },
            }
        }
    }
    run_cli((year,quarter));
}

fn run_cli((year, quarter): (i32,i32)) {
    let current_dir = std::env::var("CURRENT_DIR").unwrap();

    let path = PathBuf::from(format!(
        "{}/output/AAPL/{}/Q{}/",
        &current_dir, &year, &quarter
    ));
    println!("{}", &path.display());

    if check_path(&path) {
        println!("year/quart has already been scraped");
        println!("exiting application!");
        return;
    }
    match run((year, quarter)) {
        Ok(()) => println!("scraping successfull"),
        Err(e) => panic!("Error scraping: {}", e),
    }

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

fn default_scrape_appl() -> Result<(), Error> {
    let current_dir = std::env::var("CURRENT_DIR").unwrap();

    let years = vec![2020, 2021, 2022, 2023, 2024];
    for year in 0..years.len() {
        match run((years[year], 1)) {
            Ok(()) => {
                println!("scraped first quarter successfully");

                let path = PathBuf::from(format!(
                    "{}/output/AAPL/{}/Q{}/",
                    &current_dir, &years[year], 1
                ));
                process_quarter(&path).unwrap();
            }
            Err(e) => panic!("Error scraping: {}", e),
        }
        match run((years[year], 2)) {
            Ok(()) => {
                println!("scraped second quarter successfully");
                let path = PathBuf::from(format!(
                    "{}/output/AAPL/{}/Q{}/",
                    &current_dir, &years[year], 2
                ));
                process_quarter(&path).unwrap();
            }
            Err(e) => panic!("Error scraping: {}", e),
        }
        match run((years[year], 3)) {
            Ok(()) => {
                println!("scraped third quarter successfully");
                let path = PathBuf::from(format!(
                    "{}/output/AAPL/{}/Q{}/",
                    &current_dir, &years[year], 3
                ));
                process_quarter(&path).unwrap();
            }
            Err(e) => panic!("Error scraping: {}", e),
        }
        match run((years[year], 4)) {
            Ok(()) => {
                println!("scraped annual report successfully");
                let path = PathBuf::from(format!(
                    "{}/output/AAPL/{}/Q{}/",
                    &current_dir, &years[year], 4
                ));
                process_yearly(&path).unwrap();
            }
            Err(e) => panic!("Error scraping: {}", e),
        }
    }

    Ok(())
}

pub fn check_output() -> bool {
    let current_dir = std::env::var("CURRENT_DIR").unwrap();
    let path = PathBuf::from(format!("{}/output", &current_dir));
    println!("checking path: {}", path.display());
    if check_path(&path) {
        return true;
    }
    false
}
pub fn is_produciton() -> bool {
    env::var("APP_ENV").unwrap_or_else(|_| "development".into()) == "production"
}

pub fn parse_config_dev(args: &[String]) -> Result<(i32, i32), std::io::Error> {
    println!("argument amount: {}", args.len());

    if args.len() == 2 {
        println!("no arguments specified, showing menu");
        return Ok((0, 0));
    }

    if args.len() < 4 {
        panic!("not enough arguments specified")
    }

    let year = args[2].parse::<i32>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "please enter in a valid year",
        )
    })?;

    if (year < 2001) || (year > 2024) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "please enter in a number between 2001-present",
        ));
    }

    let quarter = args[3].parse::<i32>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "please enter in a valid quarter",
        )
    })?;

    if quarter >= 5 || quarter < 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "enter in quarters 1 to 4",
        ));
    }

    Ok((year, quarter))
}

pub fn parse_config_prod(args: &[String]) -> Result<(i32, i32), std::io::Error> {
    println!("argument amount: {}", args.len());

    if args.len() == 1 {
        return Ok((0, 0));
    }

    if args.len() < 3 {
        panic!("not enough arguments specified")
    }

    let year = args[1].parse::<i32>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "please enter in a valid year",
        )
    })?;

    if (year < 2001) || (year > 2024) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "please enter in a number between 2001-present",
        ));
    }

    let quarter = args[2].parse::<i32>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "please enter in a valid quarter",
        )
    })?;

    if quarter >= 5 || quarter < 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "enter in quarters 1 to 4",
        ));
    }

    Ok((year, quarter))
}

pub fn check_path(path: &PathBuf) -> bool {
    std::path::Path::new(path).exists()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_config() {
        let args1 = [String::from(""), String::from("2020"), String::from("3")];
        let args2 = [String::from(""), String::from("2020"), String::from("5")];
        let args3 = [String::from(""), String::from("1985"), String::from("3")];
        let args4 = [String::from(""), String::from("ligma"), String::from("3")];
        let args5 = [
            String::from(""),
            String::from("2020"),
            String::from("ligma"),
        ];

        let result1 = parse_config_prod(&args1).unwrap();
        let result2 = parse_config_prod(&args2).unwrap_err();
        let result3 = parse_config_prod(&args3).unwrap_err();
        let result4 = parse_config_prod(&args4).unwrap_err();
        let result5 = parse_config_prod(&args5).unwrap_err();

        assert_eq!(result1, (2020, 3));
        assert_eq!(result2.to_string(), "enter in quarters 1 to 4");
        assert_eq!(
            result3.to_string(),
            "please enter in a number between 2001-present"
        );
        assert_eq!(result4.to_string(), "please enter in a valid year");
        assert_eq!(result5.to_string(), "please enter in a valid quarter")
    }
}

// TODO: make unit test for is_scraped() fn
