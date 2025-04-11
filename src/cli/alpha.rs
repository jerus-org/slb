use std::{
    collections::{HashSet, VecDeque},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

use crate::{Error, WordFilters};

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "wiki-100k.txt";
const DEFAULT_OUTPUT_FILE: &str = "default.txt";
const LINE_LENGTH: usize = 3010;

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
    /// word list output file name
    #[arg(short, long)]
    pub output: Option<String>,
}

impl Cmd {
    pub fn run(self, settings: std::collections::HashMap<String, String>) -> Result<(), Error> {
        // Setup settings
        let mut src_directory = settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();
        let mut src_file = settings
            .get("source_file")
            .map_or(DEFAULT_SOURCE_FILE, |v| v)
            .to_string();

        if let Some(sd) = self.dir {
            src_directory = sd;
        };
        if let Some(sf) = self.file {
            src_file = sf;
        };

        let src = format!("{}/{}", src_directory.clone(), src_file.clone());
        let dest = format!(
            "{}/{}",
            src_directory.clone(),
            self.output.unwrap_or(DEFAULT_OUTPUT_FILE.to_string())
        );

        println!("Files: {} and {}", src, dest);

        let mut words = HashSet::new();
        let mut count_duplicates = 0;

        for line in std::fs::read_to_string(&src)
            .expect("Failed to read words file")
            .lines()
        {
            if !line.is_empty() {
                let ws = line.split_whitespace();
                for w in ws {
                    if !words.insert(w.to_lowercase().to_string()) {
                        count_duplicates += 1;
                    };
                }
            }
        }

        println!(
            "Found {} unique words after excluding {count_duplicates} ",
            words.len()
        );

        let words = words.into_iter().collect::<Vec<String>>();

        tracing::info!("Loaded {} words", words.len());
        const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
        let mut words = words.filter_to_minimum_length(2);
        words.filter_includes_only_letters(ALPHABET);

        tracing::info!("Filtered words includes {} words", words.len());

        write_words(words, &dest);

        Ok(())
    }
}

fn write_words(words: Vec<String>, src: &str) {
    let lines = words_to_lines(words);

    let path = PathBuf::from_str(src).unwrap();

    let mut file = std::fs::File::create(path).unwrap();
    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

fn words_to_lines(words: Vec<String>) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut words = VecDeque::from(words);

    while !words.is_empty() {
        let word = words.pop_front().unwrap();
        if line.is_empty() {
            line = word;
            continue;
        }
        if line.len() + word.len() > LINE_LENGTH {
            lines.push(line.clone());
            line.clear();
        }
        line.push(' ');
        line.push_str(&word);
    }

    lines
}
