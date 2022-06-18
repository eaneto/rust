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
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(
            &config.query,
            &contents,
            config.invert_match,
            config.line_number,
        )
    } else {
        search(
            &config.query,
            &contents,
            config.invert_match,
            config.line_number,
        )
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    invert_match: bool,
    line_number: bool,
) -> Vec<String> {
    let mut results = Vec::new();

    for (number, line) in contents.lines().enumerate() {
        if invert_match {
            if !line.contains(&query) {
                if line_number {
                    results.push(format!("{}:{}", number + 1, line));
                } else {
                    results.push(format!("{}", line));
                }
            }
        } else {
            if line.contains(&query) {
                if line_number {
                    results.push(format!("{}:{}", number + 1, line));
                } else {
                    results.push(format!("{}", line));
                }
            }
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    invert_match: bool,
    line_number: bool,
) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (number, line) in contents.lines().enumerate() {
        if invert_match {
            if !line.to_lowercase().contains(&query) {
                if line_number {
                    results.push(format!("{}:{}", number + 1, line));
                } else {
                    results.push(format!("{}", line));
                }
            }
        } else {
            if line.to_lowercase().contains(&query) {
                if line_number {
                    results.push(format!("{}:{}", number + 1, line));
                } else {
                    results.push(format!("{}", line));
                }
            }
        }
    }

    return results;
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false, false)
        );
    }

    #[test]
    fn case_sensitive_invert_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Duct tape.";

        assert_eq!(
            vec!["Rust:", "Pick Three.", "Duct tape."],
            search(query, contents, true, false)
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

        assert_eq!(
            vec!["2:safe, fast, productive."],
            search(query, contents, false, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents, false, false)
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

        assert_eq!(
            vec!["safe, fast, productive.", "Pick Three."],
            search_case_insensitive(query, contents, true, false)
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

        assert_eq!(
            vec!["1:Rust:", "4:Trust me."],
            search_case_insensitive(query, contents, false, true)
        );
    }
}
