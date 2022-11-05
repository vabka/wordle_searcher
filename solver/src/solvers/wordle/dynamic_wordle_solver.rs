use super::super::Solver;

pub struct WordleSolver<'corpus>
where
    Self: 'corpus,
{
    corpus: &'corpus [&'corpus str],
}

impl<'corpus> Solver for WordleSolver<'corpus> {
    type PossibleGuessesIterator<'a>
    where
        Self: 'a;

    type GuessMetric;

    type BestNextGuessIterator<'a>
    where
        Self: 'a;

    type Constraint;

    type Guess<'a> = &'a str
    where
        Self: 'a;

    fn iter_possible_guesses<'a>(&'a self) -> Self::PossibleGuessesIterator<'a> {
        todo!()
    }

    fn iter_best_next_guess<'a>(&'a self) -> Self::BestNextGuessIterator<'a> {
        todo!()
    }

    fn add_constraint(&mut self, constraint: Self::Constraint) {
        todo!()
    }
}
