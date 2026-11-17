# The ubiquitous language

| Word | What it means |
|---|---|
| Addend | The value on the right of the bar that joins two vocabularies, two languages, two lists of spellings, two lists of names, two lists of terms, two unknown vocabularies or two dependency lists; the terms of both stay. |
| Attribute | An attribute of a source, an item, a field or an expression, such as the test attribute. |
| Augend | The value on the left of the bar that joins two vocabularies, two languages, two lists of spellings, two lists of names, two lists of terms, two unknown vocabularies or two dependency lists. |
| Back Word | A word of an identifier behind a piece. |
| Bare Text | A text without the whitespace around it. |
| Code | The occurrences of the identifiers a part of a source names and of those it declares: its production code or its test code. |
| Column | A column of a line of a source, counted in letters from one. |
| Comment | A doc comment of a source, which the syntax holds as the attribute doc, and which code carries none of. |
| Compose Plural Stem | To compose the plural stem of a word, its text without the plural ending where something is left. |
| Compose Sibilant Stem | To compose the sibilant stem of a word, its text without the sibilant plural ending where a sibilant letter ends what is left. |
| Compose Singular Stem | To compose the stems of a word in the plural that ends in no wye, its sibilant stem and its singular. |
| Compose Stem | To compose the stems of a word that ends in the plural ending and not in the double letter ending: its wye stem with the wye ending behind it where it ends in the wye plural ending, and otherwise its sibilant stem where it has one and the word without its plural ending where that leaves a letter. |
| Count | How many of a thing there are. |
| Crate | A package of Rust, which a manifest describes and a context covers. |
| Criterion | A closure that decides whether an element belongs to what a function selects, named for that element, such as the criterion of a directory, a component, an occurrence or a piece. |
| Current Piece | The piece the letters of an identifier are added to. |
| Decide Piece | To decide whether the spellings admit a piece, its last word as it stands or as one of its stems. |
| Decide Words | To decide whether the spellings spell the words of an identifier, or of a part of one, one after another without a gap. |
| Decision | The candidate decision of a specification of a language, such as its spellings or a directory, whether it accepts a candidate. |
| Declaration | An identifier a source declares: the name of an item, a field, a variant, a generic parameter, a lifetime, a label, a renamed import or a binding of a pattern, a parameter or a closure, which only the terms of its languages admit. |
| Defect | A place of a source that breaks a rule of the code apart from its words, with its line and its column: a test attribute in front of the test module that opens no module tests at the bottom of the source, a comment anywhere, or an identifier with a piece that is no word, with its text and the error of that word. |
| Definition | The text that states what a term means, without the whitespace around it. |
| Domain | The layer of the library: its model of languages, sources and books, and the ports through which it reaches the world. |
| Double Letter Ending | The ss at the end of a word, which is no plural. |
| End | The position after the last word of a piece, as the spellings count the words of an identifier, and the start after every token of a text, at which a text without a test module ends. |
| Error | A refusal, and the type that states it. |
| File | A file of the file system, as the reader opens it and the writer creates it. |
| Front Word | A word of an identifier in front of a piece. |
| Gates Vocabulary Language Core Language | The crate gates_vocabulary_language_core_language, the vocabulary and the terms a source is judged against, with the ports of the parser and the reader. |
| Hasher | The state a word is hashed into, and the type of that state. |
| Identifier | A name of the code of a source, as Rust tokenizes it, and the words it falls apart into; an identifier with a piece that is no word is a defect of its source. |
| Item | A part of a source or of a module at its top: a function, a type, a constant, a module, a trait, an impl block, an import or a macro. |
| Language | The vocabulary and the terms a source is judged against, and the library crate that judges. |
| Last Letter | The letter at the end of the current piece. |
| Last Position | The position of the last word of a piece. |
| Last Word | The word a piece ends in, the only word of a piece that may stand in the plural. |
| Last Word Form | The last word of a piece, or one of its stems. |
| Leading Words | The words of a piece in front of its last word. |
| Letter | One character of a text. |
| Line | A line of the text of a source, counted from one, and a line of the text of an error. |
| List | Several values in their order. |
| List Sources | To list the sources a path names: the path itself where it is no directory, and every Rust file under it, in their order, where it is one. |
| Literal | A string, a character or a number of a source, which names nothing; only its suffix is an identifier. |
| Lower Case | The case of the small letters, in which words are compared. |
| Name | An identifier a language admits in exactly its words, without a definition; and the text a file, a directory, a crate, an attribute, a module, a function or a key of a manifest is called by, such as Cargo.toml, cfg or path. |
| Next Letter | The letter after the one at hand. |
| Next Position | The position of the letter after the one at hand. |
| Occurrence | An identifier as a source writes it, with the line and the column it starts at and the words it falls apart into. |
| Open File | To open a file for its text through the reader. |
| Order | The order of two words, which compare by their texts in lower case. |
| Parse Source | To parse the text of a source into its production code, its test code and its defects, as the parser does. |
| Parser | The port that parses the text of a source into the identifiers its production code and its test code name and declare. |
| Path | The location of a file or a directory, and the path of Rust that names an attribute, such as cfg_attr. |
| Piece | A part of an identifier between underscores and changes of case, which has to be a word; and words of an identifier one after another, which the spellings admit or reject. |
| Plural Ending | The s at the end of a plural. |
| Plural Stem | The stem a plural word leaves without its plural ending, such as harbor of harbors. |
| Position | The index of a letter in a text, of a word in an identifier or of a component in a path. |
| Production | What a crate ships: the code of its sources in front of their test modules, as opposed to their tests. |
| Production Code | The code of a source in front of its test module, whose names its languages admit and whose declarations only the terms of its contexts and of the contexts of its path dependencies spell, and the only code whose identifiers keep a term of a context in use. |
| Punctuation | A token that is one sign of punctuation or one delimiter of a group, as Rust tokenizes it, and no name. |
| Reader | The port that opens a file, whose text is then read. |
| Redefinition | The error of the term texts of a language that define a term a second time, with the term text of the second definition. |
| Segment | A part of the text of an identifier between two separators, which splits into pieces where the case of its letters changes. |
| Select Start Spellings | To select the spelling of every piece of an identifier that begins at a start position, where the words in front of it and the words behind it are spelled as well. |
| Select Unknown Occurrences | To select every occurrence of a code whose identifier a specification rejects. |
| Select Unknown Test Vocabulary | To select every occurrence of an identifier the test code of a source names or declares that neither the spellings of a vocabulary nor its names, taken as spellings, spell. |
| Select Unknown Vocabulary | To select every occurrence of an identifier the production code of a source names that neither the names nor the spellings of a vocabulary admit, and every occurrence of an identifier it declares that its spellings do not admit. |
| Select Unused Terms | To select the terms of a context whose spellings no source of the run spells, and none where no source of the run lies in its directory. |
| Separator | A letter or a text between two parts: the bar between the columns of a row, the underscore between the words of an identifier, the hyphen between the words of the name of a package, the space between the words of a spelling or a term text, the line break between the lines of an error of a source or of unused terms, and the point in front of the suffix of a temporary path. |
| Sibilant Letters | The letters s, x, z and h, after which a plural ends in es. |
| Sibilant Plural Ending | The es at the end of the plural of a word whose singular ends in a sibilant. |
| Sibilant Stem | What a word in the plural leaves once its sibilant plural ending is cut off, where it ends in one of the sibilant letters. |
| Singular | What a word in the plural leaves once its plural ending is cut off. |
| Source | The production code and the test code of one file, and its defects, as the parser parses them. |
| Source Spellings | The spellings the production code and the package identifier of every source of a run spell, by the path of the source. |
| Source Text | The text of a file that holds a source, which the parser parses. |
| Spelling | The words of a term in their order, and a word a language admits alone, as a spelling of one word. |
| Split Pieces | To split a segment into the text of every piece, where an upper case letter follows a lower case letter or a digit, or starts a word after other upper case letters. |
| Start | The line and the column a token begins at, and the position a piece begins at. |
| Stem | A form a word in the plural may stem from, for which a vocabulary that holds that form admits the word: the word without its plural ending, its sibilant stem, or its wye stem with the wye ending behind it. |
| Term | A spelling of one or more words with its definition. |
| Term Text | The texts of the words of a spelling and the text of a definition, as a context writes a term. |
| Test | The condition test of a configuration, under which the tests of a crate are compiled. |
| Test Attribute | An attribute that tests, the attribute test, or the attribute cfg or cfg_attr whose condition names test anywhere, which a source carries only as the attribute cfg(test) of its test module. |
| Test Code | The code of the test module of a source apart from the names of its test functions, every identifier of which the spellings, the names and the test words of its test language spell, whether it names or declares it. |
| Text | A string, such as the text of a source, of a word, of a stem or of a definition. |
| Token | A unit of the text of a source as proc_macro2 lexes it: an identifier, a literal, a punctuation or a group. |
| Unknown Declaration | A declaration of a source that the spellings of its languages do not admit. |
| Unknown Occurrence | An occurrence of a source whose identifier its languages do not admit. |
| Unknown Vocabulary | The occurrences of the identifiers of a source that a language does not admit, in the order of their lines and their columns. |
| Unused Term | A term of a context that no identifier of the production code of the sources of a run in its directory or in the directory of a crate that depends on it, and no package identifier of them, spells; the gate refuses it with the option terms. |
| Value | A value of a manifest as TOML decodes it, such as a table of dependencies or the path of an entry. |
| Vocabulary | The spellings and the names a language admits, against which it judges every identifier of a source. |
| Word | A text of letters and digits of the alphabet of ASCII that begins with a letter, compared and ordered without regard to its case. |
| Wye Ending | The wye, the y, a stem ends in, in the place of the wye plural ending. |
| Wye Plural Ending | The ies at the end of a plural whose singular ends in the letter wye, the y. |
| Wye Stem | What a word in the plural leaves once its wye plural ending is cut off, before the wye ending is put behind it. |
