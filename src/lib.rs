use std::{error::Error, fs, process};

///识别非UTF-8文本文件 并 输出统计信息
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
///包装命令行参数
pub struct Config {
    pub query: String,
    pub filename: String,
}
///抛出错误(命令行参数缺失)
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("文件名为空\n用法: file_search.exe  文件名 字符串"),
        };
        let query = match args.next() {
            Some(val) => val,
            None => return Err("搜索内容为空\n用法: file_search.exe 文件名 字符串"),
        };
        Ok(Config { query, filename })
    }
}

//解决  TODO  #1  解决一行只匹配一次的 => 使用split/match_indices进行处理
//TODO  #2  支持正则表达式搜索 => 自写/crate
//放弃  TODO  #3  支持gui => 基于crate
//解决  TODO  #4  搜索结果显示时,应输出原始文本,而非lower_case文本    =>  to_lowercase()赋值新字符串,并使用match_indice替换split,用索引实现get_formatted_string()
//TODO  #5  不输出完整行,只输出附近内容
//TODO  #6  当ANSI转义序列跨越两行时,第二行中结束序列失效   =>通过TODO#5解决

///在文件中搜索字符串
pub fn search<'a>(query: &str, contents: &str) -> usize {
    let lower_query = query.to_lowercase();
    let lower_contents = contents.to_lowercase();
    let mut line_number: usize = 0;
    let mut result_number: usize = 0;
    let mut raw_line_iter = contents.lines();
    for line in lower_contents.lines() {
        let raw_line: &str = match raw_line_iter.next() {
            None => "",
            Some(str) => str,
        };
        line_number = line_number + 1;
        let result = find_string(&lower_query, &line);
        if result.len() == 0 {
            ()
        } else {
            result_number = result_number + result.len();
            let line = get_formatted_string(&query, &raw_line.to_string(), &result);
            println!("\x1b[35m> 行{line_number} :\x1b[0m {}", line)
        }
    }
    return result_number;
}
///在单行文本中搜索字符串
fn find_string(query: &str, contents: &str) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::<usize>::new();
    for zip in contents.match_indices(query) {
        result.push(zip.0)
    }
    return result;
}
///输出格式化后的搜索结果
fn get_formatted_string(query: &str, contents: &String, indexes: &Vec<usize>) -> String {
    let mut formatted_contents = String::new();
    let mut last_index = 0;
    for &index in indexes.iter() {
        formatted_contents += &contents[last_index..index];
        formatted_contents += "\x1b[31;103m";
        formatted_contents += &contents[index..index + query.len()];
        formatted_contents += "\x1b[0m";
        last_index = index + query.len();
    }
    if last_index < contents.len() {
        formatted_contents += &contents[last_index..];
    }

    formatted_contents
}
