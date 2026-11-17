use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct CommandLineArgumentParser;

impl Layer for CommandLineArgumentParser {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for CommandLineArgumentParser {}

impl CommandLineArgumentParser {
    const NAME: &'static str = "clap";

    const NAMES: &'static [&'static str] = &[
        "version",
        "help",
        "about",
        "ArgumentConflict",
        "clap",
        "command",
        "conflicts_with",
        "conflicts_with_all",
        "default_value",
        "default_value_t",
        "env",
        "global",
        "long",
        "MissingRequiredArgument",
        "name",
        "Parser",
        "required",
        "required_unless_present",
        "requires_all",
        "Subcommand",
        "subcommand",
        "trailing_var_arg",
        "value_delimiter",
        "num_args",
        "allow_hyphen_values",
        "UnknownArgument",
        "value_name",
        "value_parser",
    ];
}

impl TryFrom<CommandLineArgumentParser> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: CommandLineArgumentParser) -> Result<Self, Self::Error> {
        let foreign_name_text = CommandLineArgumentParser::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = CommandLineArgumentParser::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_command_line_argument_parser_is_a_name() {
        let command_line_argument_parser = CommandLineArgumentParser;

        let foreign_language = ForeignLanguage::try_from(command_line_argument_parser);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
