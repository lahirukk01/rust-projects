use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    case_insensitive: bool,
}

impl Config {
    pub fn build<I: Iterator<Item = String>>(args: &mut I) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path."),
        };

        let args_map = Self::build_args_map(args);
        let case_insensitive = Self::is_case_insensitive(&args_map);

        Ok(Config {
            query,
            file_path,
            case_insensitive,
        })
    }

    fn build_args_map<I: Iterator<Item = String>>(args: I) -> HashMap<String, bool> {
        let mut args_map: HashMap<String, bool> = HashMap::new();

        for arg in args {
            if arg.starts_with("-") {
                args_map.insert(arg.clone(), true);
            }
        }

        args_map
    }

    fn is_case_insensitive(args_map: &HashMap<String, bool>) -> bool {
        args_map.contains_key("-i")
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn case_insensitive(&self) -> bool {
        self.case_insensitive
    }
}

fn search<R: BufRead>(query: &str, reader: &mut R, is_case_insensitive: bool) -> Vec<String> {
    let mut results = Vec::new();

    for line_result in reader.lines() {
        if let Ok(mut line) = line_result {
            if is_case_insensitive == true {
                line = line.to_lowercase();
            }

            if line.contains(query) {
                results.push(line);
                // println!("{}", line);
            }
        }
    }

    // reader
    //     .lines()
    //     .filter(|mut line| {
    //         if is_case_insensitive == true {
    //             line = line.to_lowercase();
    //         }
    //         line.contains(query)
    //     })
    //     .collect();

    results
    // Ok(())
}

pub fn run(config: &Config) -> Result<(), Error> {
    let file = File::open(&config.file_path)?;
    let mut reader = BufReader::new(file);

    let results = search(&config.query, &mut reader, config.case_insensitive());

    // for line_result in reader.lines() {
    //     let line = line_result?;

    //     if line.contains(&config.query) {
    //         println!("{}", line);
    //     }
    // }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
        Pick three.";

        let mut reader = BufReader::new(Cursor::new(contents));

        assert_eq!(
            vec!["safe, fast, productive.".to_string()],
            search(&query, &mut reader, false)
        );
    }
}
