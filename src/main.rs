use file_search::Config;
use std::{env, process};

fn main() {
    println!("\x1b[31m每一行只会匹配一次,但如果出现,一定在结果输出的内容中\x1b[0m");
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("参数错误: {}", err);
        process::exit(1);
    });
    if let Err(e) = file_search::run(config) {
        eprintln!("运行时发生错误: {}", e);
        process::exit(1);
    }
}
