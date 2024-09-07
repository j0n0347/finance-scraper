use axum::{routing::get, Router};

pub mod routes;
pub mod utils;

pub fn create_app() -> Router {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/flow/:year/:quarter", get(routes::get_cash_flows_data))
        .route("/income/:year/:quarter", get(routes::get_income_data))
        .route("/balance/:year/:quarter", get(routes::get_balance_data));
    app
}

#[cfg(test)]
mod test {
    use utils::validate_params;

    use super::*;
    #[test]
    fn test_validate_params() {
        let mut year = "2020";
        let mut quarter = "Q3";

        let result1 = validate_params(year, quarter);

        assert_eq!(result1, true);

        year = "3000";

        let result2 = validate_params(year, quarter);

        assert_eq!(result2, false);

        year = "2024";
        quarter = "Q5";

        let result3 = validate_params(year, quarter);

        assert_eq!(result3, false);
    }
}
