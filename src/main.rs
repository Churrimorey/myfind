use std::{env, process};

use regex::Regex;

use colored::*;

mod find;
mod walk_tree;

fn main() {
    let args: Vec<String> = env::args().collect();

    //参数1：搜索目录  参数2：要搜索的正则表达式
    if args.len() < 3 {
        eprintln!(
            "{}{} {}",
            "使用方式：".red(),
            args[0].red(),
            "<目标目录> <要搜索的正则表达式>".red()
        );
        process::exit(1);
    }
    //-v --verbose是第一个参数或者最后一个参数
    let len = args.len();
    // println!("{:?}", args);
    let flag = {
        if args[1].eq("-v") || args[1].eq("--verbose") {
            1
        } else if args[len - 1].eq("-v") || args[len - 1].eq("--verbose") {
            2
        } else {
            0
        }
    };

    let lbegin = {
        if flag == 1 {
            2
        } else {
            1
        }
    };

    let mut begin = lbegin + 1;
    while args[begin].contains("\\") || args[begin].contains("/") {
        begin += 1;
    }

    let end = {
        if flag == 2 {
            len - 1
        } else {
            len
        }
    };

    let mut final_matches: Vec<String> = Vec::new();

    // println!("lbegin: {}, begin: {}, end: {}", lbegin, begin, end);
    for j in lbegin..begin {
        let mut dir_matches: Vec<String> = Vec::new();
        for i in begin..end {
            let pattern = &args[i];
            let regex = match Regex::new(pattern) {
                Ok(re) => re,
                Err(err) => {
                    eprintln!("{} '{}': {}", "无效的正则表达式".red(), pattern.red(), err);
                    process::exit(1);
                }
            };

            match find::find(&args[j], &regex, flag) {
                Ok(matches) => {
                    if matches.is_empty() {
                        dir_matches.clear();
                        break;
                    } else {
                        if i == begin {
                            dir_matches = matches;
                        } else {
                            let tmp_matches: Vec<&String> = matches
                                .iter()
                                .filter(|&s| dir_matches.contains(s))
                                .collect();
                            dir_matches.clear();
                            for s in tmp_matches {
                                dir_matches.push(s.to_string());
                            }
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{}：{}", "发生错误".red(), error);
                    process::exit(1);
                }
            }
        }
        let dir_matches: Vec<&String> = dir_matches
            .iter()
            .filter(|&s| !final_matches.contains(s))
            .collect();
        for filename in dir_matches {
            final_matches.push(filename.to_string());
        }
    }

    if final_matches.is_empty() {
        println!("{}", "未找到匹配项。".bright_yellow());
    } else {
        println!("{}", "找到以下匹配项：".bright_yellow());
        for file in final_matches {
            println!("{}", file.blue());
        }
    }
}
