mod cli;

extern crate glob;

use std::borrow::{Borrow, Cow};
use self::glob::glob;
use std::io::{self, Write, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;
use clap::Parser;
use crate::cli::Cli;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<prefix>.+)\.(?P<suffix>.*)").unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(args: cli::Cli) {
    let file = args.pattern;
    let length_str = args.length_str;
    let target_dir = args.target_dir;
    for entry in glob(&file).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => process_path(&path, length_str, &target_dir),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn process_path(path: &std::path::PathBuf, length_str: u16, target_dir: &Option<String>) {
    if path.is_file() {
        println!("Processing {:?} with size {}", path.display(), length_str);
        if let Ok(lines) = read_lines(path) {
            let mut line_count = 0;
            let mut chunk_count = 1;
            let path_string = create_target_file(path, target_dir);
            let mut chunk_name: Cow<str> = Cow::from("");
            let mut existing_file: Option<File> = None;
            for line in lines {
                if let Ok(ip) = line {
                    if line_count % length_str == 0 {
                        chunk_name = RE.replace_all(&path_string, build_replacement(&mut chunk_count));
                        chunk_count += 1;
                        println!("Split ++ {}", chunk_name);
                        delete_on_start(&mut chunk_name);
                    }

                    let file_path = chunk_name.to_string();
                    let mut content = ip.to_string();
                    content += "\r\n";
                    existing_file = create_or_append(&file_path, content, existing_file);
                }
                line_count += 1;
            }
        }
    }
}

fn create_or_append(file_path: &String, content: String, existing_file: Option<File>)
    -> Option<File> {
    if Path::new(&file_path).exists() == false {
        let written_file = File::create(&file_path);
        match written_file {
            Err(e) => {
                println!("{:?}", e);
                None
            },
            Ok(file) => {
                file.borrow().write_all(content.as_bytes()).unwrap();
                Some(file)
            }
        }
    }
    else if let Some(file) = existing_file {
        file.borrow().write_all(content.as_bytes()).unwrap();
        Some(file)
    }
    else {
        let written_file = File::options().append(true).open(file_path);
        match written_file {
            Err(e) => {
                println!("{:?}", e);
                None
            },
            Ok(file) => {
                file.borrow().write_all(content.as_bytes()).unwrap();
                Some(file)
            }
        }
    }
}

fn create_target_file(path: &PathBuf, target_dir: &Option<String>) -> String {
    let path_string = match target_dir {
        Some(s) => {
            let mut base = s.to_string();
            let x = path.file_name().unwrap().to_str().unwrap();
            base += "/";
            base += x;
            base
        },
        None => path.display().to_string()
    };
    path_string
}

fn delete_on_start(chunk_name: &mut Cow<str>) {
    let file_path = chunk_name.to_string();
    if Path::new(&file_path).exists() {
        File::create(&file_path).unwrap().set_len(0);
    }
}

fn build_replacement(chunk_count: & i32) -> String {
    let mut replacement = "${prefix}_".to_string();
    replacement += &*chunk_count.to_string();
    replacement += ".$suffix";
    replacement
}

fn main() {
    let args = Cli::parse();
    read_file(args);
}