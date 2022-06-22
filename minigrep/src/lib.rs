use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
    pub invert_match: bool,
    pub line_number: bool,
    pub count: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let mut ignore_case = false;
        let mut line_number = false;
        let mut invert_match = false;
        let mut count = false;

        if let Some(options) = args.next() {
            if options.contains('i') {
                ignore_case = true;
            }

            if options.contains('n') {
                line_number = true;
            }

            if options.contains('v') {
                invert_match = true;
            }

            if options.contains('c') {
                count = true;
            }
        }

        Ok(Config {
            query,
            filename,
            ignore_case,
            line_number,
            invert_match,
            count,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = search(&contents, &config);

    if config.count {
        println!("{}", results.len());
    } else {
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search(contents: &str, config: &Config) -> Vec<String> {
    return contents
        .lines()
        .enumerate()
        .filter(|(_, line)| matches(line, config))
        .map(|(number, line)| format_line(line, number, config.line_number))
        .collect();
}

fn format_line(line: &str, number: usize, line_number: bool) -> String {
    if line_number {
        format!("{}:{}", number + 1, line)
    } else {
        line.to_string()
    }
}

fn matches(line: &str, config: &Config) -> bool {
    let (line, query) = if config.ignore_case {
        (line.to_lowercase(), config.query.to_lowercase())
    } else {
        // TODO Remove clone
        (line.to_string(), config.query.clone())
    };

    if config.invert_match {
        !line.contains(&query)
    } else {
        line.contains(&query)
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

        assert_eq!(vec!["Rust:", "Trust me."], search(contents, config));
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
            search(contents, config)
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

        assert_eq!(vec!["1:Rust:", "4:Trust me."], search(contents, config));
    }
}
