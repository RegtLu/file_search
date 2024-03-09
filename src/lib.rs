use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;
    let results = search(&config.query, &contents);
    println!("搜索完文件,匹配到{}个结果", results);
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

//TODO 解决一行只匹配一次的问题 => 没有头绪

pub fn search<'a>(query: &str, contents: &str) -> usize {
    let query = query.to_lowercase();
    let mut line_number: usize = 0;
    let mut result_number: usize = 0;
    for line in contents.lines() {
        line_number = line_number + 1;
        let a = line.find(&query);
        let _b = match a {
            Some(usize) => {
                result_number = result_number + 1;
                println!(
                    "\x1b[32m> 行{} 列{}:\x1b[0m {}\x1b[31;103m{}\x1b[0m{}",
                    line_number,
                    Some(usize).as_slice()[0] + 1,
                    line[0..Some(usize).as_slice()[0]].to_string(),
                    line[Some(usize).as_slice()[0]..Some(usize).as_slice()[0] + query.len()]
                        .to_string(),
                    line[Some(usize).as_slice()[0] + query.len()..].to_string()
                )
            }
            None => (),
        };
    }
    result_number
}
