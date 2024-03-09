use std::{error::Error, fs, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(config.filename) {
        Ok(c) => c,
        Err(e) => {
            println!("读取文件时发生以下错误:{}", e);
            process::exit(1)
        }
    };
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

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("文件名为空\n用法: minigrep.exe  文件名 字符串"),
        };
        let query = match args.next() {
            Some(val) => val,
            None => return Err("搜索内容为空\n用法: minigrep.exe 文件名 字符串"),
        };
        Ok(Config { query, filename })
    }
}

//TODO  #1  解决一行只匹配一次的 => 补充1
// !    错误1   有时会出现索引错误,原因未知(怀疑是行中存在英文字符导致)
    //TODO  补充1    解决以上(改为.chars()实现,可能需要自写匹配函数(可能解决 #1 ))
//TODO  #2  支持正则表达式搜索 => 自写/crate
//TODO  #3  支持gui => 基于crate
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
                let content1 = if Some(usize).as_slice()[0] < 30 {
                    line[..Some(usize).as_slice()[0]].to_string()
                } else {
                    "...".to_string()
                        + &line[Some(usize).as_slice()[0] - 30..Some(usize).as_slice()[0]]
                            .to_string()
                };
                let content3 = if (line.len() - Some(usize).as_slice()[0] - query.len()) < 30 {
                    line[Some(usize).as_slice()[0] + query.len()..].to_string()
                } else {
                    line[Some(usize).as_slice()[0] + query.len()
                        ..Some(usize).as_slice()[0] + query.len() + 30]
                        .to_string()
                        + &"...".to_string()
                };
                println!(
                    "\x1b[32m> 行{} 字节{}:\x1b[0m {}\x1b[31;103m{}\x1b[0m{}",
                    line_number,
                    Some(usize).as_slice()[0] + 1,
                    content1,
                    &line[Some(usize).as_slice()[0]..Some(usize).as_slice()[0] + query.len()]
                        .to_string(),
                    content3
                )
            }
            None => (),
        };
    }
    result_number
}
