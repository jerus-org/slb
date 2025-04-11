use std::{collections::VecDeque, io::Write, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::{Error, WordFilters};

const DEFAULT_SOURCE_DIR: &str = "words";
const DEFAULT_SOURCE_FILE: &str = "mit_words.txt";
const DEFAULT_MINIMUM_WORD_LENGTH: &str = "3";
const LINE_LENGTH: usize = 3010;

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
    /// word list source file
    #[arg(short, long)]
    pub file: Option<String>,
    /// minimum word length
    #[arg(short, long)]
    pub minimum: Option<usize>,
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
        let mut minimum_word_length = settings
            .get("minimum_word_list")
            .map_or(DEFAULT_MINIMUM_WORD_LENGTH, |v| v)
            .to_string()
            .parse::<usize>()
            .unwrap();

        if let Some(sd) = self.dir {
            src_directory = sd;
        };
        if let Some(sf) = self.file {
            src_file = sf;
        };
        if let Some(mwl) = self.minimum {
            minimum_word_length = mwl;
        };

        let src = format!("{}/{}", src_directory.clone(), src_file.clone());

        let mut words = Vec::new();

        for line in std::fs::read_to_string(&src)
            .expect("Failed to read words file")
            .lines()
        {
            if !line.is_empty() {
                let ws = line.split_whitespace();
                for w in ws {
                    words.push(w.to_string());
                }
            }
        }

        tracing::info!("Loaded {} words", words.len());
        let mut words = words.filter_to_minimum_length(minimum_word_length);
        words.filter_no_repeated_letters();

        tracing::info!("Filtered words includes {} words", words.len());

        write_words(words, &src);

        Ok(())
    }
}

fn write_words(words: Vec<String>, src: &str) {
    let lines = words_to_lines(words);

    let mut path = PathBuf::from_str(src).unwrap();
    path.set_extension("slb");

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
