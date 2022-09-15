
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use crate::{append_sep, Cli, read_file, read_lines, SEPARATOR};
    use crate::{glob};

    #[test]
    fn test_append_sep() {
        let s = "test".to_string();
        let s_copy = s.to_string();
        let res = append_sep(s);
        assert_eq!(res, format!("{}{}", s_copy, SEPARATOR));
    }

    #[test]
    fn test_read_lines() {
        let path_str = "./data/*";
        for entry in glob::glob(path_str).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let read_line_res = read_lines(path);
                    assert!(read_line_res.is_ok());
                }
                Err(e) => {
                    panic!("Glob matching failed {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_splitter_with_regex() {
        let target_path = "/tmp";
        let length = 50;
        let cli = Cli {
            pattern: String::from("./data/tb_country.csv"),
            length_str: length,
            target_dir: Some(target_path.to_string()),
            record_regex: Some(String::from("(?<!\\\\)\r\n"))
        };
        read_file(cli);
        let splits = [1, 2, 3];
        for split in splits {
            let expected_file = format!("{}/tb_country_{}.csv", target_path, split) .to_string();
            println!("{}", expected_file);
            let path = Path::new(&expected_file);
            let count = count_lines(path);
            assert!(path.exists());
            if split < 3 {
                assert_eq!(count as u16, length);
            }
        }
    }

    fn count_lines(path: &Path) -> i32{
        let result = File::open(path);
        let mut f = BufReader::new(result.unwrap());
        let mut count = 0;
        let mut line = String::new();
        if let Ok(mut bytes) = f.read_line(&mut line) {
            while bytes > 0 {
                // Do something with the line
                count += 1;
                line.clear();
                match f.read_line(&mut line) {
                    Ok(b) => bytes = b,
                    Err(_) => {},
                }
            }
        }
        count
    }
}