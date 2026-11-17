mod texts;

pub(super) use texts::PieceTexts;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct Piece {
    text: String,
}

impl Layer for Piece {
    type Layer = DomainLayer;
}

impl ValueObject for Piece {}

impl Piece {
    pub(crate) fn text(&self) -> &str {
        match self {
            Self { text } => text,
        }
    }
}

impl From<String> for Piece {
    fn from(text: String) -> Self {
        Self { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_piece_keeps_its_text() {
        let piece_text_literal = "Harbor";
        let piece_text = String::from(piece_text_literal);
        let piece = Piece::from(piece_text);

        let text = piece.text();

        assert_eq!(text, piece_text_literal);
    }
}
