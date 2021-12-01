use std::env;
use std::fs;
mod tests;

#[derive(Debug)]
pub struct Config<'a> {
    target: &'a str,
    content: String,
    is_case_sensitive: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineData<'a> {
    line_no: i32,
    content: &'a str,
}

impl<'a> LineData<'a> {
    pub fn new(line_no: i32, content: &str) -> LineData {
        LineData { line_no, content }
    }
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }

        let target = &args[1];
        let filename = &args[2];

        // read the file specified by the string and if we can't,
        // throw the error back to the user
        fs::read_to_string(filename)
            .and_then(|file_contents| {
                Ok(Config {
                    target,
                    content: file_contents,
                    is_case_sensitive: env::var("IS_CASE_SENSITIVE").is_ok(),
                })
            })
            .map_err(|e| format!("{:?}", e))
    }

    pub fn from_data(target: &str, content: String) -> Config {
        Config {
            target,
            content,
            is_case_sensitive: env::var("IS_CASE_SENSITIVE").is_ok(),
        }
    }
}

pub fn search<'a>(config: &'a Config) -> Vec<LineData<'a>> {
    let lower_cased_target = config.target.to_ascii_lowercase();
    config
        .content
        .lines()
        .into_iter()
        .enumerate()
        .filter_map(|(line_no, content)| match config.is_case_sensitive {
            true => {
                if content.contains(config.target) {
                    return Some(LineData {
                        line_no: line_no as i32,
                        content: content.trim(),
                    });
                }
                return None;
            }
            false => {
                if content.to_lowercase().contains(&lower_cased_target) {
                    return Some(LineData {
                        line_no: line_no as i32,
                        content: content.trim(),
                    });
                }
                return None;
            }
        })
        .collect()
}
