extern crate glob;
use self::glob::glob;
use std::io::{self, Write, BufRead};
use std::env;
use std::fs::File;
use std::path::Path;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(file: String, length_str: u32) {
    println!("Processing {} {}", file, length_str);
    for entry in glob(&file).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => process_path(path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn process_path(path: std::path::PathBuf) {
    if path.is_file() {
        println!("Processing {:?}", path.display());
        if let Ok(lines) = read_lines(path) {
            let mut i = 0;
            for line in lines {
                i += 1;
                if let Ok(ip) = line {
                    println!("{} - {}", i, ip);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        io::stderr().write_all(b"Please enter the files you want to split and the number of lines.\
            \r\nExample: spliter.exe <pattern> <size of split>");
    } else {
        let pattern = &args[0];
        let length_str: u32 = (&args[1]).parse().unwrap();
        read_file(pattern.to_string(), length_str);
    }
}