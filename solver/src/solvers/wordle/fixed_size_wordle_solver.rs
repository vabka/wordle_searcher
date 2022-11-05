use std::{
    collections::HashMap,
    fmt::Display,
    iter::{Map, Take},
    // ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::super::Solver;

pub struct NaiveSolver<const WORD_LENGTH: usize> {
    lines: Vec<WordleLine<WORD_LENGTH>>,
    performed_guesses: usize,
    corpus: Vec<String>,
    corpus_exclude: Vec<String>,
    //hard_mode: bool,
}

impl<const WORD_LENGTH: usize> NaiveSolver<WORD_LENGTH> {
    pub fn new(corpus: Vec<String>) -> Self {
        Self {
            corpus,
            lines: vec![],
            performed_guesses: 0,
            corpus_exclude: vec![],
            // hard_mode: false,
        }
    }

    pub fn iter_corpus<'game>(&'game self) -> CorpusIterator<'game, WORD_LENGTH> {
        CorpusIterator::new(self)
    }

    pub fn add_guess(
        &mut self,
        guess: WordleLine<WORD_LENGTH>,
    ) -> Result<(), AddGuessError<WORD_LENGTH>> {
        if guess.len() != WORD_LENGTH {
            Err(AddGuessError {
                guess,
                error: AddGuessErrorVariant::WordLength {
                    expected_length: WORD_LENGTH,
                },
            })
        } else {
            self.lines.push(guess);
            self.performed_guesses += 1;
            Ok(())
        }
    }

    // pub fn get_frequencies(&self) -> (usize, HashMap<char, Frequencies<WORD_LENGTH>>) {
    //     let mut entropy = HashMap::with_capacity(50);
    //     let mut total = 0;
    //     for word in self.iter_corpus() {
    //         for (i, character) in word.chars().take(WORD_LENGTH).enumerate() {
    //             let entry: &mut Frequencies<WORD_LENGTH> = entropy.entry(character).or_default();
    //             entry.positions[i] += 1;
    //         }
    //         total += 1;
    //     }
    //     (total, entropy)
    // }

    // fn calcualte_entropy(count: usize, frequencies: HashMap<char, Frequencies<WORD_LENGTH>>){
    //     for (character, frequencies) in frequencies.iter() {
    //         let mut exact = [0f32; WORD_LENGTH];
    //         let mut another = [0f32; WORD_LENGTH];
    //         let mut miss = 0f32;
    //     }
    // }

    pub fn is_excluded(&self, word: &str) -> bool {
        self.corpus_exclude.iter().any(|w| w == word)
    }

    pub fn exclude(&mut self, word: String) -> Result<(), ExcludeWordError> {
        if self.is_excluded(&word) {
            Err(ExcludeWordError::AlreadyExcluded)
        } else if WORD_LENGTH == word.len() {
            self.corpus_exclude.push(word);
            Ok(())
        } else {
            Err(ExcludeWordError::InvalidLength {
                expected_length: WORD_LENGTH,
            })
        }
    }

    pub fn performed_guesses(&self) -> usize {
        self.performed_guesses
    }
}

// pub struct Frequencies<const WORD_LENGTH: usize> {
//     positions: [usize; WORD_LENGTH],
// }

// pub struct Probability<
//     const WORD_LENGTH: usize,
//     P: Add + AddAssign + Sub + SubAssign + Div + DivAssign + Mul + MulAssign + Sized + From<usize> + Copy,
// > {
//     exact: [P; WORD_LENGTH],
//     except: [P; WORD_LENGTH],
//     omitted: P,
// }

// impl<const WORD_LENGTH: usize> Frequencies<WORD_LENGTH> {
//     pub fn positions(&self) -> &[usize] {
//         &self.positions
//     }
// }

// impl<const WORD_LENGTH: usize> Default for Frequencies<WORD_LENGTH> {
//     fn default() -> Self {
//         Self {
//             positions: [0; WORD_LENGTH],
//         }
//     }
// }

pub struct CorpusIterator<'game, const WL: usize> {
    game: &'game NaiveSolver<WL>,
    pos: usize,
}

impl<'game, const WL: usize> CorpusIterator<'game, WL> {
    pub fn new(game: &'game NaiveSolver<WL>) -> Self {
        Self { game, pos: 0 }
    }
}

pub enum ExcludeWordError {
    InvalidLength { expected_length: usize },
    AlreadyExcluded,
}

impl<'game, const WORD_LENGTH: usize> Iterator for CorpusIterator<'game, WORD_LENGTH> {
    type Item = &'game str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.game.corpus.len() <= self.pos {
                return None;
            }

            let word = &self.game.corpus[self.pos];

            let possible_word = &self
                .game
                .lines
                .iter()
                .all(|line| line.satisfies(word.as_str()));

            if *possible_word {
                if !self.game.is_excluded(word) {
                    self.pos += 1;
                    return Some(word.as_str());
                }
            }

            self.pos += 1;
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AddGuessError<const WL: usize> {
    pub guess: WordleLine<WL>,
    pub error: AddGuessErrorVariant,
}

impl<const WORD_LENGTH: usize> Display for AddGuessError<WORD_LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error {
            AddGuessErrorVariant::WordLength { expected_length: _ } => {
                write!(f, "Invalid guess length")
            }
            AddGuessErrorVariant::NoMoreAttempts { total_attempts: _ } => {
                write!(f, "No more attempts")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddGuessErrorVariant {
    WordLength { expected_length: usize },
    NoMoreAttempts { total_attempts: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct WordleLine<const WORD_LENGTH: usize> {
    pub chars: [(char, WordleCharStatus); WORD_LENGTH],
}

impl<const WL: usize> WordleLine<WL> {
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn satisfies(&self, word: &str) -> bool {
        if word.chars().count() != self.len() {
            return false;
        }
        for (word_character, (guess_character, status)) in word.chars().zip(&self.chars) {
            match status {
                WordleCharStatus::Inexistent => {
                    if word_character.eq(guess_character) {
                        return false;
                    }
                }
                WordleCharStatus::Existing => {
                    if word_character.eq(guess_character) {
                        return false;
                    }
                }
                WordleCharStatus::Good => {
                    if word_character.ne(guess_character) {
                        return false;
                    }
                }
            }
        }

        for (guess_char, status) in &self.chars {
            match status {
                WordleCharStatus::Inexistent => {
                    if word.chars().any(|c| c.eq(guess_char)) {
                        return false;
                    }
                }
                WordleCharStatus::Existing => {
                    if word.chars().all(|c| c.ne(guess_char)) {
                        return false;
                    }
                }
                WordleCharStatus::Good => {}
            }
        }
        true
    }

    pub fn new(letters: [(char, WordleCharStatus); WL]) -> WordleLine<WL> {
        WordleLine { chars: letters }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum WordleCharStatus {
    Inexistent,
    Existing,
    Good,
}

impl<const WORD_LENGTH: usize> Solver for NaiveSolver<WORD_LENGTH> {
    type PossibleGuessesIterator<'a> = CorpusIterator<'a, WORD_LENGTH> where Self: 'a;
    type GuessMetric = ();

    type BestNextGuessIterator<'a> = BestNextGuessIterator<'a, WORD_LENGTH>
    where
        Self: 'a;

    type Constraint = WordleLine<WORD_LENGTH>;

    type Guess<'a> = &'a str;

    fn iter_possible_guesses<'a>(&'a self) -> Self::PossibleGuessesIterator<'a> {
        todo!()
    }

    fn iter_best_next_guess<'a>(&'a self) -> Self::BestNextGuessIterator<'a> {
        BestNextGuessIterator {
            inner: self.iter_corpus(),
            taken: 0,
        }
    }

    fn add_constraint(&mut self, constraint: Self::Constraint) {
        todo!()
    }
}

pub struct BestNextGuessIterator<'a, const WL: usize> {
    inner: CorpusIterator<'a, WL>,
    taken: usize,
}

impl<'a, const WL: usize> Iterator for BestNextGuessIterator<'a, WL> {
    type Item = (&'a str, ());

    fn next(&mut self) -> Option<Self::Item> {
        if self.taken < 10 {
            self.taken += 1;
            Some((self.inner.next()?, ()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tets {
    use super::*;

    #[test]
    fn filter_corpus_after_successful_guess() {
        let corpus = vec![
            "aaa".to_string(),
            "bbb".to_string(),
            "ccc".to_string(),
            "abc".to_string(),
            "cba".to_string(),
        ];
        let mut game: NaiveSolver<3> = NaiveSolver::new(corpus);
        let guess = WordleLine {
            chars: [
                ('a', WordleCharStatus::Existing),
                ('b', WordleCharStatus::Good),
                ('c', WordleCharStatus::Existing),
            ],
        };
        let add_result = game.add_guess(guess);
        assert_eq!(Ok(()), add_result);

        let filtered_corpus: Vec<&str> = game.iter_corpus().collect();

        assert_eq!(vec!["cba"], filtered_corpus);
    }

    // #[test]
    // fn vocabulary() {
    //     let corpus = vec![
    //         "aaa".to_string(),
    //         "bbb".to_string(),
    //         "ccc".to_string(),
    //         "abc".to_string(),
    //         "cba".to_string(),
    //     ];
    //     let game: NaiveSolver<3, 6> = NaiveSolver::new(corpus);
    //     let (count, map) = game.get_frequencies();
    //     assert_eq!(count, 5);
    //     assert_eq!(map[&'a'].positions(), [2, 1, 2]);
    //     assert_eq!(map[&'b'].positions(), [1, 3, 1]);
    //     assert_eq!(map[&'c'].positions(), [2, 1, 2]);
    // }
}
