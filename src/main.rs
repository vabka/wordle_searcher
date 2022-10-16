use std::{
    error::Error,
    fmt::Display,
    io::{self, prelude::*, BufReader},
};
use std::{fs::File, path::Path};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[0];
    // let path = r"C:\Users\Vabka\Downloads\russian_nouns_v2.0\russian_nouns.txt";

    let corpus = read_all_lines_lowercase_with_exact_length(path, 5)?;
    let mut game = WordleGame::new(corpus, 6, 5);
    loop {
        let guess = get_guess()?;
        if let Err(e) = game.add_guess(guess) {
            match e.error {
                AddGuessErrorVariant::WordLength { expected_length: _ } => {
                    eprint!("Invalid guess length!");
                    continue;
                }
                AddGuessErrorVariant::NoMoreAttempts { total_attempts: _ } => {
                    eprint!("No more attempts!");
                    break;
                }
            }
        }
        let mut count = 0;
        println!("Возможные варианты:");
        for word in game.iter_corpus() {
            println!("{}", word);
            count +=1;
        }
        println!("Всего {}", count);
        if count <= 1 {
            println!("Больше подсказать не могу. Возможно решено за {}", game.performed_guesses);
            break;
        }       
        println!("================================");
    }
    Ok(())
}

fn get_guess() -> Result<WordleLine, GuessError> {
    println!("Введённое слово: ");
    let mut word = String::with_capacity(12);
    io::stdin().read_line(&mut word)?;
    let trimmed_word = word.trim();

    println!("Маска (*-не угадано. ?-не на своём месте): ");
    let mut mask = String::with_capacity(12);
    io::stdin().read_line(&mut mask)?;
    let trimmed_mask = mask.trim();

    let word_chars = trimmed_word.chars();
    let mask_chars = trimmed_mask.chars();
    if word_chars.count() == mask_chars.count() {
        let letters = trimmed_word
            .chars()
            .zip(trimmed_mask.chars())
            .filter_map(|(ch, mask_ch)| {
                if mask_ch == '*' {
                    Some((ch, WordleCharStatus::Inexistent))
                } else if mask_ch == '?' {
                    Some((ch, WordleCharStatus::Existing))
                } else if mask_ch == ch {
                    Some((ch, WordleCharStatus::Good))
                } else {
                    None
                }
            })
            .collect();
        Ok(WordleLine::new(letters))
    } else {
        Err(GuessError::WordMaskLength)
    }
}

#[derive(Debug)]
enum GuessError {
    IO,
    WordMaskLength,
}
impl From<io::Error> for GuessError {
    fn from(_: io::Error) -> Self {
        GuessError::IO
    }
}
impl Error for GuessError {}

impl Display for GuessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessError::IO => write!(f, "I/O error"),
            GuessError::WordMaskLength => write!(f, "word.len() != mask.len()"),
        }
    }
}
fn read_all_lines_lowercase_with_exact_length(
    path: impl AsRef<Path>,
    length: usize,
) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut corpus = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim().to_lowercase();
            if trimmed.chars().count() == length {
                corpus.push(trimmed.to_string());
            }
        }
    }
    Ok(corpus)
}

pub struct WordleGame {
    lines: Vec<WordleLine>,
    word_length: usize,
    performed_guesses: usize,
    attempts: usize,
    corpus: Vec<String>,
    corpus_exclude: Vec<String>,
    hard_mode: bool,
}

impl WordleGame {
    pub fn new(corpus: Vec<String>, attempts: usize, word_length: usize) -> Self {
        Self {
            corpus,
            lines: Vec::with_capacity(attempts),
            word_length: word_length,
            performed_guesses: 0,
            attempts,
            corpus_exclude: vec![],
            hard_mode: false,
        }
    }

    pub fn iter_corpus<'game>(&'game self) -> CorpusIterator<'game> {
        CorpusIterator::new(self)
    }

    pub fn add_guess(&mut self, guess: WordleLine) -> Result<(), AddGuessError> {
        if self.performed_guesses == self.attempts {
            Err(AddGuessError {
                guess,
                error: AddGuessErrorVariant::NoMoreAttempts {
                    total_attempts: self.attempts,
                },
            })
        } else if guess.len() != self.word_length {
            Err(AddGuessError {
                guess,
                error: AddGuessErrorVariant::WordLength {
                    expected_length: self.word_length,
                },
            })
        } else {
            self.lines.push(guess);
            self.performed_guesses += 1;
            Ok(())
        }
    }

    pub fn is_excluded(&self, word: &str) -> bool {
        self.corpus_exclude.iter().any(|w| w == word)
    }

    pub fn exclude(&mut self, word: String) -> Result<(), ExcludeWordError> {
        if self.is_excluded(&word) {
            Err(ExcludeWordError::AlreadyExcluded)
        } else if self.word_length == word.len() {
            self.corpus_exclude.push(word);
            Ok(())
        } else {
            Err(ExcludeWordError::InvalidLength {
                expected_length: self.word_length,
            })
        }
    }
}

pub struct CorpusIterator<'game> {
    game: &'game WordleGame,
    pos: usize,
}

impl<'game> CorpusIterator<'game> {
    pub fn new(game: &'game WordleGame) -> Self {
        Self { game, pos: 0 }
    }
}

pub enum ExcludeWordError {
    InvalidLength { expected_length: usize },
    AlreadyExcluded,
}
impl<'game> Iterator for CorpusIterator<'game> {
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
pub struct AddGuessError {
    pub guess: WordleLine,
    pub error: AddGuessErrorVariant,
}

impl Display for AddGuessError {
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
pub struct WordleLine {
    pub chars: Vec<(char, WordleCharStatus)>,
}

impl WordleLine {
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

    fn new(letters: Vec<(char, WordleCharStatus)>) -> WordleLine {
        WordleLine { chars: letters }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum WordleCharStatus {
    Inexistent,
    Existing,
    Good,
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
        let mut game = WordleGame::new(corpus, 6, 3);
        let guess = WordleLine {
            chars: vec![
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
}
