//! Prepares a list of words from a file for use in by the solver.

use std::collections::HashMap;

pub trait WordFilters {
    fn filter_to_minimum_length(self, length: usize) -> Self;
    fn filter_no_repeated_letters(&mut self) -> &mut Self;
    fn filter_excludes_letters(self, exclude: &str) -> Self;
    fn filter_includes_only_letters(&mut self, include: &str) -> &mut Self;
    fn filter_includes_any_letters(self, include: &str) -> Self;
    fn filter_includes_all_letters(self, include: &str) -> Self;
    fn filter_includes_same_letters(&mut self, include: &str) -> &mut Self;
    fn filter_includes_specific_letters_in_volume(self, include: &str) -> Self;
}

impl WordFilters for Vec<String> {
    #[tracing::instrument(skip(self))]
    fn filter_to_minimum_length(self, length: usize) -> Self {
        let mut lc = self
            .iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>();
        lc.retain(|word| word.len() >= length);

        tracing::info!("Filtered to {} words", lc.len());
        lc
    }

    #[tracing::instrument(skip(self))]
    fn filter_no_repeated_letters(&mut self) -> &mut Self {
        self.retain(|word| {
            let mut chars = word.chars();
            let mut a = chars.next().unwrap();
            for b in chars {
                if a == b {
                    return false;
                }
                a = b
            }
            true
        });

        tracing::info!("With no repeated letters there are {} words", self.len());
        self
    }

    #[tracing::instrument(skip(self))]
    fn filter_excludes_letters(self, exclude: &str) -> Self {
        let excludes = self
            .iter()
            .filter(|word| {
                let chars = word.chars();
                for char in chars {
                    if exclude.contains(char) {
                        return false;
                    }
                }
                true
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            exclude,
            excludes.len()
        );
        excludes
    }

    #[tracing::instrument(skip(self))]
    fn filter_includes_only_letters(&mut self, include: &str) -> &mut Self {
        self.retain(|word| {
            let chars = word.chars();
            for char in chars {
                if !include.contains(char) && char != ' ' {
                    return false;
                }
            }
            true
        });

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            include,
            self.len()
        );
        self
    }

    #[tracing::instrument(skip(self, include))]
    fn filter_includes_all_letters(self, include: &str) -> Self {
        let includes = self
            .iter()
            .filter(|word| {
                let chars = include.chars();
                for char in chars {
                    if !word.contains(char) {
                        return false;
                    }
                }
                true
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            include,
            includes.len()
        );
        includes
    }

    #[tracing::instrument(skip(self, include))]
    fn filter_includes_any_letters(self, include: &str) -> Self {
        let includes = self
            .iter()
            .filter(|word| {
                let chars = word.chars();
                for char in chars {
                    if include.contains(char) {
                        return true;
                    }
                }
                false
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        tracing::info!(
            "With no letters from the exclusion list ({})there are {} words",
            include,
            includes.len()
        );
        includes
    }

    #[tracing::instrument(skip(self, anagram))]
    fn filter_includes_same_letters(&mut self, anagram: &str) -> &mut Self {
        let anagram_dist = anagram
            .chars()
            .map(|c| (c, 1))
            .collect::<HashMap<char, usize>>();

        tracing::debug!("Letter distribution: {:?}", anagram_dist);

        self.retain(|word| {
            let word_dist = word
                .chars()
                .map(|c| (c, 1))
                .collect::<HashMap<char, usize>>();

            word_dist == anagram_dist && word.len() == anagram.len() && word.as_str() != anagram
        });

        tracing::info!("There are {} anagrams of {}", self.len(), anagram);
        self
    }

    fn filter_includes_specific_letters_in_volume(self, letters: &str) -> Self {
        let mut letter_distribution = HashMap::new();
        for letter in letters.chars() {
            if letter != ' ' {
                *letter_distribution.entry(letter).or_insert(0) += 1;
            }
        }

        let mut output = Vec::new();

        for word in self {
            let mut test_dist = letter_distribution.clone();
            let mut good_word = true;
            for letter in word.chars() {
                if test_dist.contains_key(&letter) && test_dist.get(&letter).unwrap() > &0 {
                    *test_dist.get_mut(&letter).unwrap() -= 1;
                } else {
                    good_word = false;
                    break;
                }
            }
            if good_word {
                output.push(word);
            }
        }

        output
    }
}

#[allow(dead_code)]
fn no_repeated_letter(word: &str) -> bool {
    let mut chars = word.chars();
    let mut a = chars.next().unwrap();
    for b in chars {
        if a == b {
            return false;
        }
        a = b
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_repeated_letter() {
        let words = vec!["ab", "aa", "all", "success", "fulfil", "greatness"];

        let mut passed = vec![];
        let mut failed = vec![];
        for word in words {
            if no_repeated_letter(word) {
                passed.push(word);
            } else {
                failed.push(word);
            }
        }
        assert_eq!(passed, vec!["ab", "fulfil"]);
        assert_eq!(failed, vec!["aa", "all", "success", "greatness"]);
    }

    use super::WordFilters;

    #[test]
    fn test_filter_exclude_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_excludes_letters("cl");
        assert_eq!(filtered, vec!["ab", "treat", "greatness"]);
    }

    #[test]
    fn test_filter_include_all_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "alternate",
            "grown",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_includes_all_letters("al");
        assert_eq!(filtered, vec!["all", "alternate"]);
    }

    #[test]
    fn test_filter_include_any_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "grown",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.filter_includes_any_letters("al");
        assert_eq!(
            filtered,
            vec!["ab", "loser", "all", "fulfil", "treat", "greatness",]
        );
    }

    #[test]
    fn test_filter_include_only_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let mut filtered = words.clone();
        filtered.filter_includes_only_letters("acubsel");
        assert_eq!(filtered, vec!["ab", "all", "success"]);

        let mut filtered = words;
        filtered.filter_includes_only_letters("a cub sel");
        assert_eq!(filtered, vec!["ab", "all", "success"]);
    }

    #[test]
    fn test_filter_include_same_letters() {
        let words = [
            "ab",
            "loser",
            "all",
            "simper",
            "spirem",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let mut filtered = words.clone();
        filtered.filter_includes_same_letters("primes");
        assert_eq!(filtered, vec!["simper", "spirem"]);
    }

    #[test]
    fn test_filter_letters_in_volume() {
        let words = [
            "ab",
            "loser",
            "all",
            "simper",
            "spirem",
            "success",
            "fulfil",
            "treat",
            "greatness",
        ];

        let words = words.iter().map(|w| w.to_string()).collect::<Vec<String>>();

        let filtered = words.clone();
        let filtered = filtered.filter_includes_specific_letters_in_volume("abloserimpucftgn");
        assert_eq!(filtered, vec!["ab", "loser", "simper", "spirem"]);
    }
}
