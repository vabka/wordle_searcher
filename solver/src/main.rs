use std::{
    error::Error,
    fmt::Display,
    io::{self, prelude::*, BufReader},
};
use std::{fs::File, path::Path};

use wordle_searcher::*;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    //let args: Vec<String> = std::env::args().collect();
    //let path = &args[1];
    let path = r"C:\Users\Vabka\Downloads\russian_nouns_v2.0\russian_nouns.txt";

    let corpus = read_all_lines_lowercase_with_exact_length(path, 5)?;
    let mut game: WordleGame<5, 6> = WordleGame::new(corpus);
    loop {
        let guess = get_guess()?;
        if let Err(e) = game.add_guess(guess) {
            match e.error {
                AddGuessErrorVariant::WordLength { expected_length: _ } => {
                    eprintln!("Invalid guess length!");
                    continue;
                }
                AddGuessErrorVariant::NoMoreAttempts { total_attempts: _ } => {
                    eprintln!("No more attempts!");
                    break;
                }
            }
        }
        let mut count = 0;
        println!("Возможные варианты:");
        for word in game.iter_corpus() {
            println!("{}", word);
            count += 1;
        }
        println!("Всего {}", count);
        if count <= 1 {
            println!(
                "Больше подсказать не могу. Возможно решено за {}",
                game.performed_guesses()
            );
            break;
        }
        println!("================================");
    }
    Ok(())
}

fn get_guess<const WORD_LENGTH: usize>() -> Result<WordleLine<WORD_LENGTH>, GuessError> {
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
        let mut vec = Vec::with_capacity(WORD_LENGTH);
        vec.extend(
            trimmed_word
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
                }),
        );

        Ok(WordleLine::new(vec.to_fixed_sized_array().unwrap()))
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

trait ToFixedSizedArray {
    type Item;
    type Error;
    fn to_fixed_sized_array<const SIZE: usize>(self) -> Result<[Self::Item; SIZE], Self::Error>;
}

impl<T: Copy> ToFixedSizedArray for Vec<T> {
    type Item = T;
    type Error = Self;
    fn to_fixed_sized_array<const SIZE: usize>(self) -> Result<[Self::Item; SIZE], Self::Error> {
        if self.capacity() == SIZE {
            Ok(unsafe { *self.as_ptr().cast() })
        } else {
            Err(self)
        }
    }
}
