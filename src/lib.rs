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

//TODO 实现匹配内容高亮
pub fn search<'a>(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut lines: Vec<String> = Vec::new();
    let mut line_number: usize = 0;

    for line in contents.lines() {
        line_number = line_number + 1;
        let a = line.find(&query);
        let _b = match  a{
            Some(usize) => lines.push(format!("\x1b[92m> 行{line_number} 列{}: \x1b[0m{line}",Some(usize).as_slice()[0]+1)),
            None => (),
        };
    }
    lines.push("已搜索完文件".to_string());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            search("爱我", "我爱你\n你也爱我")[0],
            format!("\x1b[92m> 行1 列1: \x1b[0m我爱你")
        );
    }
}
