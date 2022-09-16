use std::str;
use fancy_regex::Regex;
use substring::Substring;
use crate::cli::Cli;

pub(crate) struct StructData<'a> {
    pub(crate) path: &'a std::path::PathBuf,
    pub(crate) args: &'a Cli,
}

impl StructData<'_> {
    pub(crate) fn create_target_file(&self) -> String {
        let path_string = match &self.args.target_dir {
            Some(s) => {
                let mut base = s.to_string();
                let x = self.path.file_name().unwrap().to_str().unwrap();
                base += "/";
                base += x;
                base
            }
            None => self.path.display().to_string()
        };
        path_string
    }
}

pub fn split_data(data_str: &str, separator: &Regex) -> Vec<String> {
    let mut str_vec: Vec<String> = Vec::new();
    let mut res = separator.find(data_str);
    let mut start = 0;
    let mut end;
    while res.is_ok() {
        let option = res.unwrap();
        if option.is_some() {
            let m = option.unwrap();
            end = m.start();
            let s = byte_slice(data_str, start, end);
            str_vec.push(s.to_string());
            start = m.end();
            res = separator.find_from_pos(data_str, start);
        } else {
            str_vec.push(byte_slice(data_str, start, data_str.len())
                .to_string());
            break;
        }
    }
    str_vec
}

fn byte_slice(s: &str, start: usize, end: usize) -> &str {
    if end <= start { return ""; }

    let mut bidx = 0; // byte index
    let mut start_idx = 0;

    for b in s.bytes() {
        if bidx == start {
            start_idx = bidx;
        }

        if bidx == end {
            return &s[start_idx..bidx];
        }

        bidx += 1;
    }

    // did find start but not end
    &s[start_idx..]
}
