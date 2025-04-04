use std::{collections::HashMap, fs::read_dir};

use clap::Parser;

use crate::Error;
const DEFAULT_SOURCE_DIR: &str = "words";

#[derive(Parser, Debug, Clone)]
pub struct Cmd {
    /// word list source directory
    #[arg(short, long)]
    pub dir: Option<String>,
}

impl Cmd {
    pub fn run(self, settings: HashMap<String, String>) -> Result<(), Error> {
        // Setup settings
        let mut src_directory = settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();

        if let Some(sd) = self.dir {
            src_directory = sd;
        };

        for entry in read_dir(src_directory).unwrap().flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some() && path.extension().unwrap() == "slb" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                println!("{}", file_name);
            }
        }

        Ok(())
    }
}
