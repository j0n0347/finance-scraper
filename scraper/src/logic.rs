use std::path::PathBuf;

use anyhow::Result;

pub fn parse_config(args: &[String]) -> Result<(i32, i32), std::io::Error> {
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

pub fn is_scraped(path: PathBuf) -> bool { 
    std::path::Path::new(&path).exists()
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

        let result1 = parse_config(&args1).unwrap();
        let result2 = parse_config(&args2).unwrap_err();
        let result3 = parse_config(&args3).unwrap_err();
        let result4 = parse_config(&args4).unwrap_err();
        let result5 = parse_config(&args5).unwrap_err();

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
