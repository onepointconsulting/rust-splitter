use std::borrow::Cow;
use fancy_regex::Regex;
use crate::split_data::StructData;
use std::fs::{File};
use utf8_read::Reader;
use crate::{create_or_append, create_split_file_name, delete_on_start, split_data, split_start_predicate};

const STR_LIMIT: usize = 1024 * 4;

struct StringContainer<'a> {
    reader: Reader<&'a File>,
}

impl Iterator for StringContainer<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf_str = create_blank();
        let mut counter = 0;
        for x in self.reader.into_iter() {
            counter += 1;
            buf_str += x.unwrap().to_string().as_str();
            if counter % STR_LIMIT == 0 {
                let copy = buf_str.to_string();
                buf_str = create_blank();
                return Some(copy);
            }
        }
        if counter > 0 {
            return Some(buf_str.to_string());
        } else {
            return None;
        }
    }

}

fn string_iter(reader: Reader<&File>) -> StringContainer {
    StringContainer { reader: reader }
}

pub(crate) fn process_chunks_utf8(struct_data: StructData, separator: Regex) {
    let length_str = struct_data.args.length_str;
    let path = struct_data.path;
    println!("Processing {:?} with size {} and regex {:?}", path.display(), length_str, separator);
    let file = File::open(path.to_str().unwrap()).unwrap();
    let reader = Reader::new(&file);

    let path_string = struct_data.create_target_file();
    let mut chunk_name: Cow<str> = Cow::from("");
    let mut split_rest = "".to_string();
    let mut line_count = 0;
    let mut chunk_count = 1;
    let mut file_option: Option<File> = None;
    for x in string_iter(reader) {
        let data_str = split_rest.to_string() + &x;
        let mut vec: Vec<String> = split_data::split_data(&data_str, &separator);
        split_rest = vec.pop().unwrap().clone();
        if !vec.is_empty() {
            for chunk in vec.iter() {
                if split_start_predicate(length_str, line_count) {
                    chunk_name = create_split_file_name(chunk_count, &path_string);
                    chunk_count += 1;
                    delete_on_start(&mut chunk_name);
                }
                file_option = create_or_append(chunk_name.to_string(), file_option, chunk.to_string());
                line_count += 1;
            }
        }
    }
    create_or_append(chunk_name.to_string(), file_option, split_rest);
}

fn create_blank() -> String {
    "".to_string()
}