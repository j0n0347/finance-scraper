use chrono::{Datelike, Utc};

pub fn validate_params(year: &str, quarter: &str) -> bool {
    let curr_year = Utc::now().year();
    // println!("{}", curr_year);
    let year_int = year.parse::<i32>().unwrap();
    // println!("year: {}", year_int);
    let quarter_int = &quarter[1..].parse::<i32>().unwrap();
    // println!("quarter: {}", quarter_int);
    if (year_int < 2001 || year_int > curr_year) || (quarter_int < &1 || quarter_int > &4) {
        false
    } else {
        true
    }
}
