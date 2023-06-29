use std::fs;
use std::error::Error;
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}
impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, file_path, ignore_case})
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(config.query, &content)
    } else {
        search(config.query, &content)
    };
    // println!("With text\n{}", content);
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) {
            result.push(line);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "xwphs";
        let content = "\
xwp is handsome.
yeah! xwphs is great.
thank you.";
        assert_eq!(vec!["yeah! xwphs is great."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "xwp";
        let content = "\
wp is handsome.
yeah! XWphs is great.
thank you.";
        assert_eq!(vec!["yeah! XWphs is great."], search_case_insensitive(query, content));
    }
}