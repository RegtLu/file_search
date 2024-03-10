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
//TODO  #2  支持正则表达式搜索 => 自写/crate
//TODO  #3  支持gui => 基于crate
pub fn search<'a>(query: &str, contents: &str) -> usize {
    let query = query.to_lowercase();
    let mut line_number: usize = 0;
    let mut result_number: usize = 0;
    for line in contents.lines() {
        line_number = line_number + 1;
        let result = find_string(&query, &line);
        if result.len() == 1 {
            ()
        } else {
            result_number = result_number + result.len() - 1;
            let line = get_formatted_string(&query, &result);
            println!("\x1b[35m> 行{line_number} :\x1b[0m {}", line)
        }
    }
    return result_number;
}

fn find_string(query: &str, contents: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::<String>::new();
    for zip in contents.split(query) {
        result.push(zip.to_string())
    }
    return result;
}
fn get_formatted_string(query: &str, contents: &Vec<String>) -> String {
    let mut line: String = String::new();
    for index in 0..contents.len() - 1 {
        line = line + &contents[index] + "\x1b[31;103m" + query + "\x1b[0m";
    }
    line = line + &contents[contents.len() - 1];
    return line;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 多匹配() {
        let query = "我";
        let contents = "我141@Löwe 老虎我 Léopard Gepardia我";
        let result = find_string(query, contents);
        let mut line: String = String::new();
        for index in 0..result.len() - 1 {
            line = line + &result[index] + "\x1b[31;103m" + query + "\x1b[0m";
        }
        line = line + &result[result.len() - 1];
        println!("{}", line);

        assert_eq!(vec!["", "141@Löwe 老虎", " Léopard Gepardia", ""], result);
    }
}
