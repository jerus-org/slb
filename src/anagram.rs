use std::collections::{HashMap, VecDeque};

use colorful::Colorful;

use crate::{DEFAULT_SOURCE_DIR, DEFAULT_WORDS_SOURCE_FILE, Error, WordFilters};

const DEFAULT_LIMIT: usize = 200;

#[derive(Debug, Default)]
pub struct Anagram {
    settings: HashMap<String, String>,
    letters: Vec<char>,
    word_source: String,
    words: Vec<String>,
    solutions: Vec<String>,
    distribution: HashMap<usize, i32>,
    limit: Option<usize>,
}

impl Anagram {
    pub fn new(letters: &str, settings: HashMap<String, String>) -> Result<Self, Error> {
        if letters.len() < 3 || letters.len() > 26 {
            return Err(Error::TooFewOrManyLetters(letters.len()));
        }

        let letters = letters
            .chars()
            .map(|l| l.to_ascii_lowercase())
            .collect::<Vec<char>>();

        Ok(Self {
            settings,
            letters,
            ..Default::default()
        })
    }

    pub fn set_word_source(&mut self, dir: Option<String>, file: Option<String>) -> &mut Self {
        // Setup settings
        let mut src_directory = self
            .settings
            .get("source_dir")
            .map_or(DEFAULT_SOURCE_DIR, |v| v)
            .to_string();
        let mut src_file = self
            .settings
            .get("source_words_file")
            .map_or(DEFAULT_WORDS_SOURCE_FILE, |v| v)
            .to_string();

        if let Some(sd) = dir {
            src_directory = sd;
        };
        if let Some(sf) = file {
            src_file = sf;
        };

        let src = format!("{}/{}", src_directory.clone(), src_file.clone());
        println!("Using word list: {}", src);
        tracing::info!("Using word list: {}", src);

        self.word_source = src;

        self
    }

    pub fn load_words(&mut self) -> &mut Self {
        let mut words = Vec::new();

        for line in std::fs::read_to_string(&self.word_source)
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

        self.words = words;

        self
    }

    #[tracing::instrument(skip(self))]
    pub fn find_solutions(&mut self) -> Result<&mut Self, Error> {
        tracing::trace!("{}", self.letters.clone().iter().collect::<String>());

        let mut filtered = self.words.clone();

        if self.letters.contains(&' ') {
            tracing::trace!("Found a space in letters");
            let anagram = self.letters.iter().collect::<String>();
            filtered.filter_includes_only_letters(
                &self
                    .letters
                    .iter()
                    .filter(|c| c != &&' ')
                    .collect::<String>(),
            );
            let mut finder = AnagramFinder::new(filtered.clone(), &anagram);
            if let Some(limit) = self.limit {
                finder.set_limit(limit);
            }
            let anagrams = finder.find_anagrams();
            filtered = anagrams;
            // let mut solutions = Vec::new();
        } else {
            tracing::debug!("{} words found", filtered.len());
            filtered.filter_includes_same_letters(&self.letters.iter().collect::<String>());
            tracing::debug!("{} words found", filtered.len());
        }

        let final_list = filtered
            .iter()
            .cloned()
            .inspect(|w| {
                self.count_solution(w.len());
            })
            .collect::<Vec<String>>();

        tracing::trace!("Final list: {:#?}", final_list);
        self.solutions = final_list;

        Ok(self)
    }

    pub fn count_solution(&mut self, chain_length: usize) -> &mut Self {
        if let Some(count) = self.distribution.get(&chain_length) {
            let v = count + 1;
            self.distribution.insert(chain_length, v);
        } else {
            self.distribution.insert(chain_length, 1);
        }

        self
    }

    pub fn word_source_string(&self) -> String {
        let s1 = "Using words sourced from ".light_cyan().dim().to_string();
        let s2 = self.word_source.clone().light_cyan().bold().to_string();
        format!("{}{}", s1, s2)
    }

    pub fn distribution_string(&self) -> String {
        let mut s = String::new();
        let mut distributions = self.distribution.iter().collect::<Vec<_>>();
        distributions.sort_by(|a, b| a.0.cmp(b.0));

        for d in distributions {
            s.push_str(&format!(
                "  - {:3.0} solutions with {:2.0} characters\n",
                d.1, d.0
            ));
        }
        s
    }

    pub fn solutions_title(&self) -> String {
        let intro = "Words using the letters ";
        let mut ul = String::new();
        for _ in 0..(intro.len() + self.letters.len()) {
            ul.push('‾');
        }

        let summary = format!(
            "{}{}",
            intro.yellow().bold(),
            self.letters.iter().collect::<String>().blue().bold()
        );
        format!("{}\n{}", summary, ul.bold().yellow())
    }

    pub fn solutions_string(&self) -> String {
        let mut s = String::new();
        let mut solutions = self
            .solutions
            .iter()
            .map(|s| (s.len(), s))
            .collect::<Vec<_>>();
        solutions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut word_length = solutions.first().unwrap_or(&(0, &"".to_string())).0;

        s.push_str("  ");
        s.push_str(
            &format!(
                "{} Solutions with {} letters.",
                self.distribution.get(&word_length).unwrap_or(&0),
                word_length
            )
            .underlined()
            .yellow()
            .to_string(),
        );
        s.push_str("\n\n");

        for solution in solutions {
            if solution.0 != word_length {
                word_length = solution.0;
                s.push_str("\n  ");
                s.push_str(
                    &format!(
                        "{} Solutions with {} letters.",
                        self.distribution.get(&word_length).unwrap_or(&0),
                        word_length
                    )
                    .underlined()
                    .yellow()
                    .to_string(),
                );
                s.push_str("\n\n");
            }
            s.push_str(&format!("    {}\n", solution.1));
        }
        s
    }
}

#[derive(Debug, Default)]
struct AnagramFinder {
    words: Vec<String>,
    anagram: String,
    new_anagram: String,
    anagrams: Vec<String>,
    limit: Option<usize>,
}

impl AnagramFinder {
    fn new(words: Vec<String>, anagram: &str) -> Self {
        Self {
            words,
            anagram: anagram.to_string(),
            ..Default::default()
        }
    }

    fn set_new_anagram(&mut self, value: &str) {
        self.new_anagram = value.to_string();
    }

    fn set_limit(&mut self, value: usize) {
        self.limit = Some(value);
    }

    #[tracing::instrument(skip(self))]
    fn find_anagrams(&mut self) -> Vec<String> {
        tracing::trace!("Anagram phrases using letters `{}`", self.anagram);

        // Get the base distribution of letters in the anagram as each letter can only be used once
        let mut base_letter_pool = HashMap::new();
        for letter in self.anagram.chars() {
            if letter != ' ' {
                *base_letter_pool.entry(letter).or_insert(0) += 1;
            }
        }

        let base_list = self
            .words
            .clone()
            .filter_includes_any_letters(&self.anagram);

        tracing::trace!("base_list: {:?} with anagram {}", base_list, &self.anagram);

        let mut base_list = base_list.filter_includes_specific_letters_in_volume(&self.anagram);
        tracing::trace!(
            "specific base_list: {:?} with anagram `{}`",
            base_list,
            &self.anagram
        );

        base_list.sort_by_key(|a| a.len());
        base_list.reverse();
        tracing::debug!("filtered, sorted and reversed: {:?}", base_list);

        let mut base_queue = VecDeque::from(base_list);

        loop {
            let limit = if let Some(limit) = self.limit {
                limit
            } else {
                DEFAULT_LIMIT
            };
            if self.anagrams.len() >= limit {
                break;
            }
            let Some(word) = base_queue.pop_front() else {
                break;
            };

            let mut build_anagram = self.new_anagram.clone();
            if !build_anagram.is_empty() {
                build_anagram.push(' ');
            }
            build_anagram.push_str(&word);

            tracing::debug!("phrase: {:?}", build_anagram);

            let mut letter_pool = base_letter_pool.clone();
            for letter in word.chars() {
                *letter_pool.entry(letter).or_insert(0) -= 1;
            }
            letter_pool.retain(|_, v| *v > 0);

            if letter_pool.is_empty() {
                self.anagrams.push(build_anagram);
                continue;
            }

            tracing::trace!("New letter pool: {:?}", letter_pool);
            let new_anagram = letter_pool.into_keys().collect::<String>();
            let mut new_finder = AnagramFinder::new(self.words.clone(), &new_anagram);
            new_finder.set_new_anagram(&build_anagram);
            let anagrams = new_finder.find_anagrams();
            self.anagrams.extend(anagrams);
        }

        self.anagrams.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_phrases_using_letters() {
        let words = [
            "apt",
            "flowerpot",
            "followers",
            "in",
            "pom",
            "min",
            "slain",
            "slam",
            "tap",
            "tin",
            "tip",
            "waterfalls",
        ];

        let anagram = "parliament of owls";

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let mut anagrams = AnagramFinder::new(words, anagram);
        let _ = anagrams.find_anagrams();

        let anagrams = anagrams.anagrams;

        assert_eq!(
            anagrams,
            vec![
                String::from("waterfalls pom in"),
                String::from("waterfalls in pom"),
                String::from("followers tap min"),
                String::from("followers min tap"),
                String::from("followers min apt"),
                String::from("followers apt min"),
                String::from("flowerpot slam in"),
                String::from("flowerpot in slam")
            ]
        );
    }
}
