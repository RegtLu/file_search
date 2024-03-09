use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);

    for result in &results {
        println!(
            "\x1b[34m> 行{} 列{}:\x1b[0m {}",
            result.line_number, result.character_number, result.content
        );
    }
    println!("搜索完文件,匹配到{}个结果", results.len());
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

//TODO 实现匹配内容高亮(切分为三块,插入ansi序列,重新格式化)
//TODO 解决一行只匹配一次的问题

pub struct QueryResult {
    line_number: usize,
    character_number: usize,
    content: String,
}

pub fn search<'a>(query: &str, contents: &str) -> Vec<QueryResult> {
    let query = query.to_lowercase();
    let mut lines: Vec<QueryResult> = Vec::new();
    let mut line_number: usize = 0;
    for line in contents.lines() {
        let mut result = QueryResult {
            line_number: 0,
            character_number: 0,
            content: String::new(),
        };
        line_number = line_number + 1;
        let a = line.find(&query);
        let _b = match a {
            Some(usize) => {
                result.line_number = line_number;
                result.character_number = Some(usize).as_slice()[0] + 1;
                result.content = line.to_string();
                lines.push(result);
            }
            None => (),
        };
    }
    lines
}
