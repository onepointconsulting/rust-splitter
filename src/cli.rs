use clap::Parser;

/// Simple program used to split files using glob patterns. Files can be split by line (default mode) or using a regular expression.
#[derive(Parser)]
pub(crate) struct Cli {
    /// The pattern used to list files, like e.g. *.csv or /media/**/*.csv
    #[clap(short, long)]
    pub(crate) pattern: String,
    /// The length of the split in terms of lines, like e.g. 50
    #[clap(short, long)]
    pub(crate) length_str: u16,
    /// An optional target directory for all of the files. If not specified the split files will be in the folder of the original file.
    #[clap(short, long)]
    pub(crate) target_dir: Option<String>,
    /// A regular expression used to split the lines, like e.g: (?<!\\)\r?\n
    #[clap(short, long)]
    pub(crate) record_regex: Option<String>
}