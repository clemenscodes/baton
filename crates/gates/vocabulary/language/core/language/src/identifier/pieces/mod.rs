mod piece;

use crate::identifier::pieces::piece::Piece;
use crate::identifier::pieces::piece::PieceTexts;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct Pieces {
    pieces: Vec<Piece>,
}

impl Layer for Pieces {
    type Layer = DomainLayer;
}

impl ValueObject for Pieces {}

impl From<&str> for Pieces {
    fn from(text: &str) -> Self {
        let piece_texts = PieceTexts::from(text);
        let piece_criterion = |piece_text: &String| !piece_text.is_empty();
        let pieces = piece_texts
            .into_iter()
            .filter(piece_criterion)
            .map(Piece::from)
            .collect();
        Self { pieces }
    }
}

impl<'pieces> IntoIterator for &'pieces Pieces {
    type Item = &'pieces Piece;
    type IntoIter = Iter<'pieces, Piece>;

    fn into_iter(self) -> Self::IntoIter {
        let Pieces { pieces } = self;
        pieces.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_text_falls_apart_into_its_pieces_at_the_underscore() {
        let text = "is_empty";

        let pieces = Pieces::from(text);

        let piece_texts: Vec<&str> = pieces.into_iter().map(Piece::text).collect();
        let expectation = ["is", "empty"];
        assert_eq!(piece_texts, expectation);
    }

    #[test]
    fn a_text_falls_apart_into_its_pieces_at_an_upper_case_letter() {
        let text = "IntoIterator";

        let pieces = Pieces::from(text);

        let piece_texts: Vec<&str> = pieces.into_iter().map(Piece::text).collect();
        let expectation = ["Into", "Iterator"];
        assert_eq!(piece_texts, expectation);
    }

    #[test]
    fn upper_case_letters_are_one_piece_until_a_lower_case_letter_opens_the_next() {
        let text = "MMRLeaderboard";

        let pieces = Pieces::from(text);

        let piece_texts: Vec<&str> = pieces.into_iter().map(Piece::text).collect();
        let expectation = ["MMR", "Leaderboard"];
        assert_eq!(piece_texts, expectation);
    }

    #[test]
    fn a_digit_belongs_to_the_piece_it_follows() {
        let text = "ak47Weapon";

        let pieces = Pieces::from(text);

        let piece_texts: Vec<&str> = pieces.into_iter().map(Piece::text).collect();
        let expectation = ["ak47", "Weapon"];
        assert_eq!(piece_texts, expectation);
    }

    #[test]
    fn a_digit_after_an_underscore_opens_a_piece_of_its_own() {
        let text = "level_2";

        let pieces = Pieces::from(text);

        let piece_texts: Vec<&str> = pieces.into_iter().map(Piece::text).collect();
        let expectation = ["level", "2"];
        assert_eq!(piece_texts, expectation);
    }

    #[test]
    fn a_letter_outside_the_alphabet_of_a_word_stays_inside_its_piece() {
        let text = "größe_slot";

        let pieces = Pieces::from(text);

        let piece_texts: Vec<&str> = pieces.into_iter().map(Piece::text).collect();
        let expectation = ["größe", "slot"];
        assert_eq!(piece_texts, expectation);
    }

    #[test]
    fn underscores_alone_yield_no_piece() {
        let text = "__";

        let pieces = Pieces::from(text);

        let expectation = Pieces::default();
        assert_eq!(pieces, expectation);
    }
}
