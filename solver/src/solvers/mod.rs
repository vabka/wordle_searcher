pub mod wordle;

pub trait Solver {
    type PossibleGuessesIterator<'a>: Iterator<Item = Self::Guess<'a>>
    where
        Self: 'a;

    type GuessMetric;

    type BestNextGuessIterator<'a>: Iterator<Item = (Self::Guess<'a>, Self::GuessMetric)>
    where
        Self: 'a;

    type Constraint;
    type Guess<'a>
    where
        Self: 'a;

    fn iter_possible_guesses<'a>(&'a self) -> Self::PossibleGuessesIterator<'a>;
    fn iter_best_next_guess<'a>(&'a self) -> Self::BestNextGuessIterator<'a>;
    fn add_constraint(&mut self, constraint: Self::Constraint);
}
