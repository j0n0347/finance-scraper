use csv_processor::{process_quarter, process_yearly};
use python::run;
use std::env;
use std::path::PathBuf;
use dotenv::dotenv;
use logic::*;

pub mod logic;

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let (year,quarter);

    if is_produciton(){
        (year, quarter) = parse_config_prod(&args).unwrap();
    } else {
        (year, quarter) = parse_config_dev(&args).unwrap();
    }


    let mut display_options = false;

    if year == 0 {
        display_options = true;
    }

    run_menu(display_options, (year,quarter));    
}
