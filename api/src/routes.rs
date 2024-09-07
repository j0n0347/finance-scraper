use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Value {
    Integer(i32),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuarterTableRow {
    name: String,
    col1: Option<i32>,
    col2: Option<i32>,
}

// #[derive(Deserialize)]
// pub struct Params {
//     year: Option<String>,
//     quarter: Option<String>,
// }

pub async fn get_cash_flows_data(
    Path((year, quarter)): Path<(String, String)>,
) -> Json<Vec<QuarterTableRow>> {
    let path = format!("../output/AAPL/{}/{}/FLOWS_cleaned.csv", year, quarter);
    let mut rdr = csv::Reader::from_path(path).expect("Unable to read path");
    let mut table: Vec<QuarterTableRow> = vec![];
    for result in rdr.records() {
        let record = result.unwrap();

        let num1: Option<i32>;
        let num2: Option<i32>;

        if record[1].is_empty() {
            num1 = None
        } else {
            num1 = match record[1].parse::<i32>() {
                Ok(num1) => Some(num1),
                _ => None,
            };
        }

        if record[2].is_empty() {
            num2 = None
        } else {
            num2 = match record[2].parse::<i32>() {
                Ok(num2) => Some(num2),
                _ => None,
            };
        }

        let row = QuarterTableRow {
            name: record[0].to_owned(),
            col1: num1,
            col2: num2,
        };
        table.push(row)
    }

    Json(table)
}

pub async fn get_income_data(
    Path((year, quarter)): Path<(String, String)>,
) -> Json<Vec<QuarterTableRow>> {
    let path = format!("../output/AAPL/{}/{}/INCOME_cleaned.csv", year, quarter);
    let mut rdr = csv::Reader::from_path(path).expect("Unable to read path");
    let mut table: Vec<QuarterTableRow> = vec![];
    for result in rdr.records() {
        let record = result.unwrap();

        let num1: Option<i32>;
        let num2: Option<i32>;

        if record[1].is_empty() {
            num1 = None
        } else {
            num1 = match record[1].parse::<i32>() {
                Ok(num1) => Some(num1),
                _ => None,
            };
        }

        if record[2].is_empty() {
            num2 = None
        } else {
            num2 = match record[2].parse::<i32>() {
                Ok(num2) => Some(num2),
                _ => None,
            };
        }

        let row = QuarterTableRow {
            name: record[0].to_owned(),
            col1: num1,
            col2: num2,
        };
        table.push(row)
    }

    Json(table)
}

pub async fn get_balance_data(
    Path((year, quarter)): Path<(String, String)>,
) -> Json<Vec<QuarterTableRow>> {
    let path = format!("../output/AAPL/{}/{}/BALANCE_cleaned.csv", year, quarter);
    let mut rdr = csv::Reader::from_path(path).expect("Unable to read path");
    let mut table: Vec<QuarterTableRow> = vec![];
    for result in rdr.records() {
        let record = result.unwrap();

        let num1: Option<i32>;
        let num2: Option<i32>;

        if record[1].is_empty() {
            num1 = None
        } else {
            num1 = match record[1].parse::<i32>() {
                Ok(num1) => Some(num1),
                _ => None,
            };
        }

        if record[2].is_empty() {
            num2 = None
        } else {
            num2 = match record[2].parse::<i32>() {
                Ok(num2) => Some(num2),
                _ => None,
            };
        }

        let row = QuarterTableRow {
            name: record[0].to_owned(),
            col1: num1,
            col2: num2,
        };
        table.push(row)
    }

    Json(table)
}

// possible get all data?
