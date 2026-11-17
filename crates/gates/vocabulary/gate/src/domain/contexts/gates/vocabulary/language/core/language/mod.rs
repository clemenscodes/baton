use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageCoreLanguageContext;

impl Layer for GatesVocabularyLanguageCoreLanguageContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageCoreLanguageContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/core/language";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Core", "Language"];
            let definition = "The crate gates_vocabulary_language_core_language, the vocabulary and the terms a source is judged against, with the ports of the parser and the reader.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Language"];
            let definition = "The vocabulary and the terms a source is judged against, and the library crate that judges.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vocabulary"];
            let definition = "The spellings and the names a language admits, against which it judges every identifier of a source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Term"];
            let definition = "A spelling of one or more words with its definition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Term", "Text"];
            let definition = "The texts of the words of a spelling and the text of a definition, as a context writes a term.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spelling"];
            let definition = "The words of a term in their order, and a word a language admits alone, as a spelling of one word.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Definition"];
            let definition = "The text that states what a term means, without the whitespace around it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Order"];
            let definition = "The order of two words, which compare by their texts in lower case.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Word"];
            let definition = "A text of letters and digits of the alphabet of ASCII that begins with a letter, compared and ordered without regard to its case.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "An identifier a language admits in exactly its words, without a definition; and the text a file, a directory, a crate, an attribute, a module, a function or a key of a manifest is called by, such as Cargo.toml, cfg or path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identifier"];
            let definition = "A name of the code of a source, as Rust tokenizes it, and the words it falls apart into; an identifier with a piece that is no word is a defect of its source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Occurrence"];
            let definition = "An identifier as a source writes it, with the line and the column it starts at and the words it falls apart into.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string, such as the text of a source, of a word, of a stem or of a definition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source"];
            let definition = "The production code and the test code of one file, and its defects, as the parser parses them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Defect"];
            let definition = "A place of a source that breaks a rule of the code apart from its words, with its line and its column: a test attribute in front of the test module that opens no module tests at the bottom of the source, a comment anywhere, or an identifier with a piece that is no word, with its text and the error of that word.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["End"];
            let definition = "The position after the last word of a piece, as the spellings count the words of an identifier, and the start after every token of a text, at which a text without a test module ends.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Separator"];
            let definition = "A letter or a text between two parts: the bar between the columns of a row, the underscore between the words of an identifier, the hyphen between the words of the name of a package, the space between the words of a spelling or a term text, the line break between the lines of an error of a source or of unused terms, and the point in front of the suffix of a temporary path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Letter"];
            let definition = "One character of a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lower", "Case"];
            let definition = "The case of the small letters, in which words are compared.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Piece"];
            let definition = "A part of an identifier between underscores and changes of case, which has to be a word; and words of an identifier one after another, which the spellings admit or reject.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start"];
            let definition = "The line and the column a token begins at, and the position a piece begins at.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Position"];
            let definition = "The index of a letter in a text, of a word in an identifier or of a component in a path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Plural", "Ending"];
            let definition = "The s at the end of a plural.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sibilant", "Plural", "Ending"];
            let definition = "The es at the end of the plural of a word whose singular ends in a sibilant.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sibilant", "Letters"];
            let definition = "The letters s, x, z and h, after which a plural ends in es.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sibilant", "Stem"];
            let definition = "What a word in the plural leaves once its sibilant plural ending is cut off, where it ends in one of the sibilant letters.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wye", "Plural", "Ending"];
            let definition = "The ies at the end of a plural whose singular ends in the letter wye, the y.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wye", "Ending"];
            let definition = "The wye, the y, a stem ends in, in the place of the wye plural ending.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wye", "Stem"];
            let definition = "What a word in the plural leaves once its wye plural ending is cut off, before the wye ending is put behind it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Double", "Letter", "Ending"];
            let definition = "The ss at the end of a word, which is no plural.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decide", "Piece"];
            let definition = "To decide whether the spellings admit a piece, its last word as it stands or as one of its stems.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unknown", "Vocabulary"];
            let definition = "The occurrences of the identifiers of a source that a language does not admit, in the order of their lines and their columns.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unknown", "Occurrence"];
            let definition = "An occurrence of a source whose identifier its languages do not admit.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Unknown", "Vocabulary"];
            let definition = "To select every occurrence of an identifier the production code of a source names that neither the names nor the spellings of a vocabulary admit, and every occurrence of an identifier it declares that its spellings do not admit.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Crate"];
            let definition = "A package of Rust, which a manifest describes and a context covers.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The location of a file or a directory, and the path of Rust that names an attribute, such as cfg_attr.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Redefinition"];
            let definition = "The error of the term texts of a language that define a term a second time, with the term text of the second definition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reader"];
            let definition = "The port that opens a file, whose text is then read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Open", "File"];
            let definition = "To open a file for its text through the reader.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File"];
            let definition = "A file of the file system, as the reader opens it and the writer creates it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error"];
            let definition = "A refusal, and the type that states it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of the library: its model of languages, sources and books, and the ports through which it reaches the world.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Parser"];
            let definition = "The port that parses the text of a source into the identifiers its production code and its test code name and declare.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Parse", "Source"];
            let definition = "To parse the text of a source into its production code, its test code and its defects, as the parser does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Declaration"];
            let definition = "An identifier a source declares: the name of an item, a field, a variant, a generic parameter, a lifetime, a label, a renamed import or a binding of a pattern, a parameter or a closure, which only the terms of its languages admit.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Attribute"];
            let definition = "An attribute that tests, the attribute test, or the attribute cfg or cfg_attr whose condition names test anywhere, which a source carries only as the attribute cfg(test) of its test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Comment"];
            let definition = "A doc comment of a source, which the syntax holds as the attribute doc, and which code carries none of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unknown", "Declaration"];
            let definition = "A declaration of a source that the spellings of its languages do not admit.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Count"];
            let definition = "How many of a thing there are.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List"];
            let definition = "Several values in their order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Attribute"];
            let definition = "An attribute of a source, an item, a field or an expression, such as the test attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bare", "Text"];
            let definition = "A text without the whitespace around it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Current", "Piece"];
            let definition = "The piece the letters of an identifier are added to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hasher"];
            let definition = "The state a word is hashed into, and the type of that state.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Item"];
            let definition = "A part of a source or of a module at its top: a function, a type, a constant, a module, a trait, an impl block, an import or a macro.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Last", "Letter"];
            let definition = "The letter at the end of the current piece.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Last", "Position"];
            let definition = "The position of the last word of a piece.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Last", "Word"];
            let definition = "The word a piece ends in, the only word of a piece that may stand in the plural.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Last", "Word", "Form"];
            let definition = "The last word of a piece, or one of its stems.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Leading", "Words"];
            let definition = "The words of a piece in front of its last word.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line"];
            let definition = "A line of the text of a source, counted from one, and a line of the text of an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Literal"];
            let definition = "A string, a character or a number of a source, which names nothing; only its suffix is an identifier.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Next", "Letter"];
            let definition = "The letter after the one at hand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Next", "Position"];
            let definition = "The position of the letter after the one at hand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Singular"];
            let definition = "What a word in the plural leaves once its plural ending is cut off.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stem"];
            let definition = "A form a word in the plural may stem from, for which a vocabulary that holds that form admits the word: the word without its plural ending, its sibilant stem, or its wye stem with the wye ending behind it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Stem"];
            let definition = "To compose the stems of a word that ends in the plural ending and not in the double letter ending: its wye stem with the wye ending behind it where it ends in the wye plural ending, and otherwise its sibilant stem where it has one and the word without its plural ending where that leaves a letter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test"];
            let definition = "The condition test of a configuration, under which the tests of a crate are compiled.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token"];
            let definition = "A unit of the text of a source as proc_macro2 lexes it: an identifier, a literal, a punctuation or a group.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value"];
            let definition = "A value of a manifest as TOML decodes it, such as a table of dependencies or the path of an entry.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Column"];
            let definition = "A column of a line of a source, counted in letters from one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Augend"];
            let definition = "The value on the left of the bar that joins two vocabularies, two languages, two lists of spellings, two lists of names, two lists of terms, two unknown vocabularies or two dependency lists.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Addend"];
            let definition = "The value on the right of the bar that joins two vocabularies, two languages, two lists of spellings, two lists of names, two lists of terms, two unknown vocabularies or two dependency lists; the terms of both stay.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decide", "Words"];
            let definition = "To decide whether the spellings spell the words of an identifier, or of a part of one, one after another without a gap.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decision"];
            let definition = "The candidate decision of a specification of a language, such as its spellings or a directory, whether it accepts a candidate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Punctuation"];
            let definition = "A token that is one sign of punctuation or one delimiter of a group, as Rust tokenizes it, and no name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Code"];
            let definition = "The occurrences of the identifiers a part of a source names and of those it declares: its production code or its test code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Production"];
            let definition = "What a crate ships: the code of its sources in front of their test modules, as opposed to their tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Production", "Code"];
            let definition = "The code of a source in front of its test module, whose names its languages admit and whose declarations only the terms of its contexts and of the contexts of its path dependencies spell, and the only code whose identifiers keep a term of a context in use.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Code"];
            let definition = "The code of the test module of a source apart from the names of its test functions, every identifier of which the spellings, the names and the test words of its test language spell, whether it names or declares it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unused", "Term"];
            let definition = "A term of a context that no identifier of the production code of the sources of a run in its directory or in the directory of a crate that depends on it, and no package identifier of them, spells; the gate refuses it with the option terms.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Unused", "Terms"];
            let definition = "To select the terms of a context whose spellings no source of the run spells, and none where no source of the run lies in its directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Spellings"];
            let definition = "The spellings the production code and the package identifier of every source of a run spell, by the path of the source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Unknown", "Test", "Vocabulary"];
            let definition = "To select every occurrence of an identifier the test code of a source names or declares that neither the spellings of a vocabulary nor its names, taken as spellings, spell.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Front", "Word"];
            let definition = "A word of an identifier in front of a piece.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Back", "Word"];
            let definition = "A word of an identifier behind a piece.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Criterion"];
            let definition = "A closure that decides whether an element belongs to what a function selects, named for that element, such as the criterion of a directory, a component, an occurrence or a piece.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Singular", "Stem"];
            let definition = "To compose the stems of a word in the plural that ends in no wye, its sibilant stem and its singular.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Text"];
            let definition = "The text of a file that holds a source, which the parser parses.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List", "Sources"];
            let definition = "To list the sources a path names: the path itself where it is no directory, and every Rust file under it, in their order, where it is one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Unknown", "Occurrences"];
            let definition = "To select every occurrence of a code whose identifier a specification rejects.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Start", "Spellings"];
            let definition = "To select the spelling of every piece of an identifier that begins at a start position, where the words in front of it and the words behind it are spelled as well.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Sibilant", "Stem"];
            let definition = "To compose the sibilant stem of a word, its text without the sibilant plural ending where a sibilant letter ends what is left.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Plural", "Stem"];
            let definition = "The stem a plural word leaves without its plural ending, such as harbor of harbors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Plural", "Stem"];
            let definition = "To compose the plural stem of a word, its text without the plural ending where something is left.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Segment"];
            let definition = "A part of the text of an identifier between two separators, which splits into pieces where the case of its letters changes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Split", "Pieces"];
            let definition = "To split a segment into the text of every piece, where an upper case letter follows a lower case letter or a digit, or starts a word after other upper case letters.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageCoreLanguageContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageCoreLanguageContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageCoreLanguageContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageCoreLanguageContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_gates_vocabulary_language_core_language_context_is_a_term() {
        let gates_vocabulary_language_core_language_context = GatesVocabularyLanguageCoreLanguageContext;

        let context = Context::try_from(gates_vocabulary_language_core_language_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
