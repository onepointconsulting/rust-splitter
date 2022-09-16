mod cli;
mod split_data;
mod tests;
mod process_chunks;

extern crate glob;

use std::borrow::{Borrow, Cow};
use self::glob::glob;
use std::io::{self, BufRead, Write};
use std::fs::{File, OpenOptions};
use std::fs;
use std::str;
use std::path::Path;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use clap::Parser;
use crate::cli::Cli;
use crate::process_chunks::process_chunks_utf8;
use crate::split_data::StructData;

const SEPARATOR: &str = "\r\n";

lazy_static! {
    static ref RE_PREFIX_SUFFIX: Regex = Regex::new(r"(?P<prefix>.+)\.(?P<suffix>.*)").unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(args: Cli) {
    for entry in glob(&args.pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                match &args.record_regex {
                    Some(regex_string) => {
                        let separator = Regex::new(regex_string).expect("Invalid regex");
                        process_regex(StructData { path: &path, args: &args }, separator);
                    }
                    None => {
                        process_lines(StructData { path: &path, args: &args })
                    }
                }
            }
            Err(e) => println!("{:?}", e)
        }
    }
}

fn process_regex(struct_data: StructData, separator: Regex) {
    if struct_data.path.is_file() {
        process_chunks_utf8(struct_data, separator);
    }
}

fn process_lines(struct_data: StructData) {
    let length_str = struct_data.args.length_str;
    let path = struct_data.path;
    if path.is_file() {
        println!("Processing {:?} with size {}", path.display(), length_str);
        if let Ok(lines) = read_lines(path) {
            let mut line_count = 0;
            let mut chunk_count = 1;
            let path_string = struct_data.create_target_file();
            let mut chunk_name: Cow<str> = Cow::from("");
            let mut file_option: Option<File> = None;
            for line in lines {
                if let Ok(ip) = line {
                    if split_start_predicate(length_str, line_count) {
                        chunk_name = create_split_file_name(chunk_count, &path_string);
                        chunk_count += 1;
                        delete_on_start(&mut chunk_name);
                    }
                    file_option = create_or_append(chunk_name.to_string(), file_option, ip);
                }
                line_count += 1;
            }
        } else {
            println!("Could not read {:?}", path.display());
        }
    }
}

fn append_sep(s: String) -> String {
    let mut s1 = s.to_string();
    s1 += SEPARATOR;
    s1
}

fn create_split_file_name<'a>(chunk_count: i32, path_string: &'a String) -> Cow<'a, str> {
    let chunk_name = RE_PREFIX_SUFFIX.replace_all(&path_string, build_replacement(&chunk_count));
    println!("Split ++ {}", chunk_name);
    chunk_name
}

fn split_start_predicate(length_str: u16, line_count: u16) -> bool {
    line_count % length_str == 0
}

fn create_or_append(file_path: String, file_option: Option<File>, s: String) -> Option<File> {
    let content = append_sep(s);
    if !Path::new(&file_path).exists() {
        let written_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_path);
        match written_file {
            Err(e) => {
                println!("{:?}", e);
                None
            }
            Ok(file) => {
                file.borrow().write_all(content.as_bytes());
                Some(file)
            }
        }
    } else {
        match file_option {
            None => {
                println!("No file to write!!");
                None
            }
            Some(file) => {
                file.borrow().write_all(content.as_bytes());
                Some(file)
            }
        }
    }
}

fn delete_on_start(chunk_name: &mut Cow<str>) {
    let file_path = chunk_name.to_string();
    if Path::new(&file_path).exists() {
        match fs::remove_file(file_path) {
            Ok(_f) => {
                println!("File {} deleted", chunk_name.to_string())
            }
            Err(_e) => {
                println!("File {} was not deleted", chunk_name.to_string())
            }
        }
    }
}

fn build_replacement(chunk_count: &i32) -> String {
    let mut replacement = "${prefix}_".to_string();
    replacement += &*chunk_count.to_string();
    replacement += ".$suffix";
    replacement
}

fn main() {
    let args = Cli::parse();
    read_file(args);
}