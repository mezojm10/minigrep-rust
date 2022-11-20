use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n {contents}");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::<&str>::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::<&str>::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        let query;
        let file_path;
        if args.len() < 3 {
            return Err("Not enough arguments");
        } else if args.len() == 3 {
            query = args[1].clone();
            file_path = args[2].clone();
        } else if args.len() == 4 && (args[1] == "-i" || args[1] == "--ignore-case") {
            ignore_case = true;
            query = args[2].clone();
            file_path = args[3].clone();
        } else {
            return Err("Invalid arguments");
        }

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn one_result_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn multiple_results() {
        let query = "man";
        let contents = "\
Only one thing man cannot do,
woman,
Ahmed
Abdallah
Omdurman
Man";
        assert_eq!(
            vec!["Only one thing man cannot do,", "woman,", "Omdurman"],
            search(query, contents)
        );
    }

    #[test]
    fn multiple_results_case_insensitive() {
        let query = "man";
        let contents = "\
Only one thing man cannot do,
woman,
Ahmed
Abdallah
Omdurman
Man";
        assert_eq!(
            vec!["Only one thing man cannot do,", "woman,", "Omdurman", "Man"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn no_results() {
        let query = "Mazin";
        let contents = "\
Yeah everything about this is amazing.
What do you think?
'I didn't like it actually!'";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn no_results_case_insensitive() {
        let query = "Mazin";
        let contents = "\
Yeah everything about this is awesome.
What do you think?
'I didn't like it actually!'";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn build_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::build(&[
            String::from("./program"),
            String::from("string to search"),
            String::from("filename.txt"),
        ])?;
        assert_eq!(config.file_path, "filename.txt");
        assert_eq!(config.query, "string to search");
        assert!(!config.ignore_case);
        Ok(())
    }

    #[test]
    fn build_config_case_insensitive() -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::build(&[
            String::from("./program"),
            String::from("-i"),
            String::from("string to search"),
            String::from("filename.txt"),
        ])?;
        assert_eq!(config.file_path, "filename.txt");
        assert_eq!(config.query, "string to search");
        assert!(config.ignore_case);

        let config = Config::build(&[
            String::from("./program"),
            String::from("--ignore-case"),
            String::from("string to search"),
            String::from("filename.txt"),
        ])?;
        assert_eq!(config.file_path, "filename.txt");
        assert_eq!(config.query, "string to search");
        assert!(config.ignore_case);
        Ok(())
    }

    #[test]
    fn build_config_invalid() {
        Config::build(&[
            String::from("./program"),
            String::from("string to search but no filename"),
        ])
        .expect_err("Should require filename to be specified");
    }
}
