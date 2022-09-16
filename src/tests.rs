
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
        let length: u16 = 50;
        let target_path = "/tmp";
        let file_stem = "tb_country";
        let cli = cli_provider(target_path, file_stem, length);
        read_file(cli);
        let splits = [1, 2, 3];
        for split in splits {
            let expected_file = format!("{}/{}_{}.csv", target_path, file_stem, split) .to_string();
            println!("{}", expected_file);
            let path = Path::new(&expected_file);
            let count = count_lines(path);
            assert!(path.exists());
            if split < splits.len() {
                assert_eq!(count as u16, length);
            }
        }
    }

    #[test]
    fn test_splitter_with_regex_single() {
        let length: u16 = 300;
        let target_path = "/tmp";
        let file_stem = "tb_country_copy";
        let cli = cli_provider(target_path, file_stem, length);
        read_file(cli);
        let expected_file = format!("{}/{}_1.csv", target_path, file_stem);
        let expected_path = Path::new(&expected_file);
        assert!(expected_path.exists());
    }

    #[test]
    fn test_splitter_with_regex_event_small() {
        let length: u16 = 3;
        let target_path = "/tmp";
        let file_stem = "tb_event_small";
        let cli = cli_provider(target_path, file_stem, length);
        read_file(cli);
    }

    fn cli_provider(target_path: &str, file_stem: &str, length: u16) -> Cli {
        let cli = Cli {
            pattern: String::from(format!("./data/{}.csv", file_stem)),
            length_str: length,
            target_dir: Some(target_path.to_string()),
            record_regex: Some(String::from("(?<!\\\\)\r?\n"))
        };
        cli
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