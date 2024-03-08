use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);

    for line in results {
        println!("{}", line);
    }
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("搜索内容为空\n用法: minigrep.exe 字符串 文件名"),
        };
        let filename = match args.next() {
            Some(val) => val,
            None => return Err("文件名为空\n用法: minigrep.exe 字符串 文件名"),
        };
        Ok(Config { query, filename })
    }
}

//TODO 实现字符级别定位+匹配内容高亮
pub fn search<'a>(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut lines: Vec<String> = Vec::new();
    let mut line_number: i32 = 0;
    for line in contents.lines() {
        line_number = line_number + 1;
        if line.to_lowercase().contains(&query) {
            lines.push(format!("\x1b[92m> 行{line_number} : \x1b[0m{line}"))
        }
    }
    lines
}

pub fn search_new<'a>(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut lines: Vec<String> = Vec::new();
    let mut line_number: i32 = 0;
    for line in contents.lines() {
        line_number = line_number + 1;
        if line.to_lowercase().contains(&query) {
            lines.push(format!("\x1b[92m> 行{line_number} : \x1b[0m{line}"))
        }
    }
    lines
}