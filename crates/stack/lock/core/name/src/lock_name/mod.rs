use crate::Error;
use stack_domain_driven_design_core::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LockName {
    text: String,
}

impl Layer for LockName {
    type Layer = DomainLayer;
}

impl ValueObject for LockName {}

impl Identifier for LockName {}

impl LockName {
    pub fn text(&self) -> &str {
        &self.text
    }

    fn check_character(character: char) -> bool {
        character.is_ascii_alphanumeric() || character == '_' || character == '-'
    }
}

impl TryFrom<String> for LockName {
    type Error = Error;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let character_presence = text.chars().all(Self::check_character);
        let option_absence = !text.starts_with('-');
        match !text.is_empty() && character_presence && option_absence {
            true => Ok(Self { text }),
            false => Err(Error::LockName { text }),
        }
    }
}

impl fmt::Display for LockName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_of_letters_digits_and_joiners_is_the_name_of_a_lock() {
        let name = LockName::try_from(String::from("rtx3080_main-1")).unwrap();

        assert_eq!(name.text(), "rtx3080_main-1");
        assert_eq!(name.to_string(), "rtx3080_main-1");
    }

    #[test]
    fn an_empty_name_an_option_and_a_path_are_no_name_of_a_lock() {
        let text = ["", "--help", "../rtx3080", "rtx3080 2"];

        let outcomes = text.map(|text| LockName::try_from(String::from(text)));

        let expectations = text.map(|text| {
            Err(Error::LockName {
                text: String::from(text),
            })
        });
        assert_eq!(outcomes, expectations);
    }
}
