use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
    pub invert_match: bool,
    pub line_number: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let mut ignore_case = false;
        let mut line_number = false;
        let mut invert_match = false;

        if args.len() > 3 {
            let options = &args[3];
            if options.contains("i") {
                ignore_case = true;
            }

            if options.contains("n") {
                line_number = true;
            }

            if options.contains("v") {
                invert_match = true;
            }
        }

        Ok(Config {
            query,
            filename,
            ignore_case,
            line_number,
            invert_match,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&contents, config)
    } else {
        search(&contents, config)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, config: Config) -> Vec<String> {
    let mut results = Vec::new();

    for (number, line) in contents.lines().enumerate() {
        if config.invert_match {
            if !line.contains(&config.query) {
                push_line(&mut results, line, config.line_number, number)
            }
        } else {
            if line.contains(&config.query) {
                push_line(&mut results, line, config.line_number, number)
            }
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(contents: &'a str, config: Config) -> Vec<String> {
    let query = config.query.to_lowercase();
    let mut results = Vec::new();

    for (number, line) in contents.lines().enumerate() {
        if config.invert_match {
            if !line.to_lowercase().contains(&query) {
                push_line(&mut results, line, config.line_number, number)
            }
        } else {
            if line.to_lowercase().contains(&query) {
                push_line(&mut results, line, config.line_number, number)
            }
        }
    }

    return results;
}

fn push_line(results: &mut Vec<String>, line: &str, line_number: bool, number: usize) {
    if line_number {
        results.push(format!("{}:{}", number + 1, line));
    } else {
        results.push(format!("{}", line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Duct tape.";

        let config = Config {
            query: query.to_string(),
            filename: "".to_string(),
            ignore_case: false,
            invert_match: false,
            line_number: false,
        };

        assert_eq!(vec!["safe, fast, productive."], search(contents, config));
    }

    #[test]
    fn case_sensitive_invert_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Duct tape.";

        let config = Config {
            query: query.to_string(),
            filename: "".to_string(),
            ignore_case: false,
            invert_match: true,
            line_number: false,
        };

        assert_eq!(
            vec!["Rust:", "Pick Three.", "Duct tape."],
            search(contents, config)
        );
    }

    #[test]
    fn case_sensitive_with_line_number() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Duct tape.";

        let config = Config {
            query: query.to_string(),
            filename: "".to_string(),
            ignore_case: false,
            invert_match: false,
            line_number: true,
        };

        assert_eq!(vec!["2:safe, fast, productive."], search(contents, config));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust me.";

        let config = Config {
            query: query.to_string(),
            filename: "".to_string(),
            ignore_case: true,
            invert_match: false,
            line_number: false,
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(contents, config)
        );
    }

    #[test]
    fn case_insensitive_invert_match() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust me.";

        let config = Config {
            query: query.to_string(),
            filename: "".to_string(),
            ignore_case: true,
            invert_match: true,
            line_number: false,
        };

        assert_eq!(
            vec!["safe, fast, productive.", "Pick Three."],
            search_case_insensitive(contents, config)
        );
    }

    #[test]
    fn case_insensitive_with_line_number() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust me.";

        let config = Config {
            query: query.to_string(),
            filename: "".to_string(),
            ignore_case: true,
            invert_match: false,
            line_number: true,
        };

        assert_eq!(
            vec!["1:Rust:", "4:Trust me."],
            search_case_insensitive(contents, config)
        );
    }
}
