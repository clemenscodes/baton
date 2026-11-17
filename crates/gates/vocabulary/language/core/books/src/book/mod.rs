use gates_vocabulary_language_core_language::Terms;
use gates_vocabulary_language_core_languages::contexts::Context;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Book {
    directory: PathBuf,
    path: PathBuf,
    terms: Terms,
}

impl Layer for Book {
    type Layer = DomainLayer;
}

impl ValueObject for Book {}

impl Book {
    const DOCUMENTS: &'static str = "docs";
    const FILE_NAME: &'static str = "UBIQUITOUS_LANGUAGE.md";
    const HEADING: &'static str = "# The ubiquitous language";
    const TABLE_HEAD: &'static str = "| Word | What it means |";
    const TABLE_RULE: &'static str = "|---|---|";
    const SEPARATOR: char = '|';
    const SEPARATOR_ESCAPE: &'static str = "\\|";

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn term_count(&self) -> usize {
        self.terms.term_count()
    }
}

impl From<&Context> for Book {
    fn from(context: &Context) -> Self {
        let context_directory = context.directory();
        let directory_path = context_directory.path();
        let documents = Self::DOCUMENTS;
        let file_name = Self::FILE_NAME;
        let directory = directory_path.join(documents);
        let path = directory.join(file_name);
        let language = context.language();
        let terms = language.terms().clone();
        Self {
            directory,
            path,
            terms,
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { terms, .. } = self;
        let heading = Self::HEADING;
        let table_head = Self::TABLE_HEAD;
        let table_rule = Self::TABLE_RULE;
        let mut text = format!("{heading}\n\n{table_head}\n{table_rule}\n");
        let separator = Self::SEPARATOR;
        let separator_escape = Self::SEPARATOR_ESCAPE;
        for term in terms {
            let definition_text = term.definition().to_string();
            let definition = definition_text.replace(separator, separator_escape);
            let row = format!("| {term} | {definition} |\n");
            text.push_str(&row);
        }
        fmt::Display::fmt(&text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_core_language::Language;
    use gates_vocabulary_language_core_language::TermText;

    mod fixture {
        use super::*;

        pub(super) fn context_factory(term_texts: &[TermText]) -> Context {
            let language = Language::try_from(term_texts).unwrap();
            let directory_name = "harbor";
            Context::from((directory_name, language))
        }
    }

    #[test]
    fn the_book_of_a_context_is_under_the_directory_of_that_context() {
        const TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        }];
        let term_texts = TERM_TEXTS;
        let context = fixture::context_factory(term_texts);

        let book = Book::from(&context);

        let book_path = book.path();
        let book_directory = book.directory();
        let expectation_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let expectation = Path::new(expectation_text);
        let directory_expectation_text = "crates/harbor/docs";
        let directory_expectation = Path::new(directory_expectation_text);
        assert_eq!(book_path, expectation);
        assert_eq!(book_directory, directory_expectation);
    }

    #[test]
    fn the_book_of_a_context_under_another_root_is_under_that_root() {
        let term_texts: &[TermText] = &[{
            let spelling: &[&str] = &["Quay"];
            let definition = "The wall a ship is moored along.";
            TermText::new(spelling, definition)
        }];
        let language = Language::try_from(term_texts).unwrap();
        let root_text = "packages";
        let root = Path::new(root_text);
        let directory_name = "quay";
        let context = Context::from((root, directory_name, language));

        let book = Book::from(&context);

        let book_path = book.path();
        let expectation_text = "packages/quay/docs/UBIQUITOUS_LANGUAGE.md";
        let expectation = Path::new(expectation_text);
        assert_eq!(book_path, expectation);
    }

    #[test]
    fn a_book_is_the_heading_the_head_and_one_row_for_every_term() {
        const TERM_TEXTS: &[TermText] = &[
            {
                let spelling: &[&str] = &["Treasure"];
                let definition = "The object a team brings to the chest.";
                TermText::new(spelling, definition)
            },
            {
                let spelling: &[&str] = &["Treasure", "Chest"];
                let definition = "A chest that holds a treasure.";
                TermText::new(spelling, definition)
            },
        ];
        let term_texts = TERM_TEXTS;
        let context = fixture::context_factory(term_texts);
        let book = Book::from(&context);

        let book_text = book.to_string();

        let expectation = r#"# The ubiquitous language

| Word | What it means |
|---|---|
| Treasure | The object a team brings to the chest. |
| Treasure Chest | A chest that holds a treasure. |
"#;
        assert_eq!(book_text, expectation);
    }

    #[test]
    fn a_separator_inside_a_definition_does_not_break_the_row_it_is_in() {
        const TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Mooring"];
            let definition = "The rope | or the chain that holds a ship.";
            TermText::new(spelling, definition)
        }];
        let term_texts = TERM_TEXTS;
        let context = fixture::context_factory(term_texts);
        let book = Book::from(&context);

        let book_text = book.to_string();

        let expectation = r#"# The ubiquitous language

| Word | What it means |
|---|---|
| Mooring | The rope \| or the chain that holds a ship. |
"#;
        assert_eq!(book_text, expectation);
    }

    #[test]
    fn a_book_counts_the_terms_it_is_made_of() {
        const TERM_TEXTS: &[TermText] = &[
            {
                let spelling: &[&str] = &["Treasure"];
                let definition = "The object a team brings to the chest.";
                TermText::new(spelling, definition)
            },
            {
                let spelling: &[&str] = &["Chest"];
                let definition = "The chest a team delivers to.";
                TermText::new(spelling, definition)
            },
        ];
        let term_texts = TERM_TEXTS;
        let context = fixture::context_factory(term_texts);
        let book = Book::from(&context);

        let term_count = book.term_count();

        let expectation: usize = 2;
        assert_eq!(term_count, expectation);
    }

    #[test]
    fn the_book_of_a_context_without_a_term_is_the_heading_and_the_head_of_an_empty_table() {
        const TERM_TEXTS: &[TermText] = &[];
        let term_texts = TERM_TEXTS;
        let context = fixture::context_factory(term_texts);
        let book = Book::from(&context);

        let book_text = book.to_string();

        let term_count = book.term_count();
        let expectation = r#"# The ubiquitous language

| Word | What it means |
|---|---|
"#;
        let term_count_expectation: usize = 0;
        assert_eq!(book_text, expectation);
        assert_eq!(term_count, term_count_expectation);
    }
}
