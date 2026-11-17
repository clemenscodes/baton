use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::mem;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(in crate::identifier::pieces) struct PieceTexts {
    piece_texts: Vec<String>,
}

impl Layer for PieceTexts {
    type Layer = DomainLayer;
}

impl ValueObject for PieceTexts {}

impl PieceTexts {
    const SEPARATOR: char = '_';

    fn split_pieces(segment: &str) -> Vec<String> {
        let letters: Vec<char> = segment.chars().collect();
        let lower_case_criterion = |last_letter: char| {
            last_letter.is_ascii_lowercase() || last_letter.is_ascii_digit()
        };
        let mut piece_text_list = Vec::new();
        let mut current_piece = String::new();
        for (position, letter) in letters.iter().copied().enumerate() {
            let next_position = position + 1;
            let next_letter = letters.get(next_position);
            if letter.is_ascii_uppercase()
                && (current_piece.ends_with(lower_case_criterion)
                    || (next_letter.is_some_and(char::is_ascii_lowercase)
                        && !current_piece.is_empty()))
            {
                let piece_text = mem::take(&mut current_piece);
                piece_text_list.push(piece_text);
            }
            current_piece.push(letter);
        }
        piece_text_list.push(current_piece);
        piece_text_list
    }
}

impl From<&str> for PieceTexts {
    fn from(text: &str) -> Self {
        let separator = Self::SEPARATOR;
        let mut piece_texts = Vec::new();
        for segment in text.split(separator) {
            let segment_piece_texts = Self::split_pieces(segment);
            piece_texts.extend(segment_piece_texts);
        }
        Self { piece_texts }
    }
}

impl IntoIterator for PieceTexts {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { piece_texts } = self;
        piece_texts.into_iter()
    }
}
