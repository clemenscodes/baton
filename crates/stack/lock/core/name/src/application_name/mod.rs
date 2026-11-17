use crate::Error;
use stack_domain_driven_design_core::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationName {
    text: String,
}

impl Layer for ApplicationName {
    type Layer = DomainLayer;
}

impl ValueObject for ApplicationName {}

impl Identifier for ApplicationName {}

impl ApplicationName {
    pub fn text(&self) -> &str {
        &self.text
    }

    fn check_character(character: char) -> bool {
        character.is_ascii_alphanumeric() || character == '_' || character == '-'
    }
}

impl TryFrom<String> for ApplicationName {
    type Error = Error;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let character_presence = text.chars().all(Self::check_character);
        let option_absence = !text.starts_with('-');
        match !text.is_empty() && character_presence && option_absence {
            true => Ok(Self { text }),
            false => Err(Error::ApplicationName { text }),
        }
    }
}

impl fmt::Display for ApplicationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_of_letters_digits_and_joiners_is_the_name_of_an_eligible_application() {
        let name = ApplicationName::try_from(String::from("resolve_main-1")).unwrap();

        assert_eq!(name.text(), "resolve_main-1");
        assert_eq!(name.to_string(), "resolve_main-1");
    }

    #[test]
    fn an_empty_name_an_option_and_a_path_are_no_name_of_an_eligible_application() {
        let text = ["", "--help", "../resolve", "resolve 2"];

        let outcomes = text.map(|text| ApplicationName::try_from(String::from(text)));

        let expectations = text.map(|text| {
            Err(Error::ApplicationName {
                text: String::from(text),
            })
        });
        assert_eq!(outcomes, expectations);
    }
}
