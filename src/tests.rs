
#[cfg(test)]
mod tests {
    use crate::{append_sep, Cli, read_file, read_lines, SEPARATOR};
    use crate::{glob};
    use super::*;

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
        let cli = Cli {
            pattern: String::from("./data/tb_country.csv"),
            length_str: 50,
            target_dir: Some("c:/tmp".to_string()),
            record_regex: Some(String::from("(?<!\\\\)\r\n"))
        };
        read_file(cli)

    }
}