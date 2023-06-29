// reconstruct
// 1. 分离出函数
// 2. 聚合变量
// 3. 错误处理
// 4. 分离逻辑代码到库包中
use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parse arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file: {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

