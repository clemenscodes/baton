mod book;

pub mod writer;

pub use book::Book;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Books {
    books: Vec<Book>,
}

impl Layer for Books {
    type Layer = DomainLayer;
}

impl ValueObject for Books {}

impl FromIterator<Book> for Books {
    fn from_iter<BooksType: IntoIterator<Item = Book>>(book_sequence: BooksType) -> Self {
        let books = book_sequence.into_iter().collect();
        Self { books }
    }
}

impl<'books> IntoIterator for &'books Books {
    type Item = &'books Book;
    type IntoIter = Iter<'books, Book>;

    fn into_iter(self) -> Self::IntoIter {
        let Books { books } = self;
        books.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_core_language::Language;
    use gates_vocabulary_language_core_language::TermText;
    use gates_vocabulary_language_core_languages::contexts::Context;
    use std::path::Path;

    #[test]
    fn books_yield_every_book_they_are_made_of_in_their_order() {
        const TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        }];
        let term_texts = TERM_TEXTS;
        let first_language = Language::try_from(term_texts).unwrap();
        let last_language = Language::try_from(term_texts).unwrap();
        let first_directory_name = "ship";
        let last_directory_name = "harbor";
        let first_context = Context::from((first_directory_name, first_language));
        let last_context = Context::from((last_directory_name, last_language));
        let first_book = Book::from(&first_context);
        let last_book = Book::from(&last_context);
        let book_list = [first_book, last_book];

        let books: Books = book_list.into_iter().collect();

        let paths: Vec<&Path> = books.into_iter().map(Book::path).collect();
        let first_book_path_text = "crates/ship/docs/UBIQUITOUS_LANGUAGE.md";
        let last_book_path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let first_book_path = Path::new(first_book_path_text);
        let last_book_path = Path::new(last_book_path_text);
        let expectation = [first_book_path, last_book_path];
        assert_eq!(paths, expectation);
    }
}
