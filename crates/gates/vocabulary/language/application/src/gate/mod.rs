mod crate_examination;
mod errors;
mod examination;
mod manifest;
mod path_judgment;
mod term_judgment;
pub mod token;

pub use errors::Errors;
pub use path_judgment::PathJudgment;
pub use term_judgment::TermJudgment;

use crate::Error;
use crate::gate::crate_examination::CrateExamination;
use crate::gate::examination::Examination;
use crate::gate::manifest::{ManifestFile, ManifestPresence};
use crate::gate::token::{TokenReport, TokenSelection};
use crate::vocabulary::Vocabulary;
use gates_vocabulary_language_core_dependencies::decoder::Decoder;
use gates_vocabulary_language_core_dependencies::manifest::Manifest;
use gates_vocabulary_language_core_dependencies::{Dependencies, DependencyList};
use gates_vocabulary_language_core_language::parser::Parser;
use gates_vocabulary_language_core_language::reader::Reader;
use gates_vocabulary_language_core_language::source::{
    Source, SourceSpellings, SourceText, SourceTokens,
};
use gates_vocabulary_language_core_languages::Languages;
use stack_domain_driven_design_core::application_service::ApplicationService;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use stack_domain_driven_design_core::specification::CandidateDecision;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct Gate<ReaderType, DecoderType, ParserType> {
    languages: Languages,
    reader: ReaderType,
    decoder: DecoderType,
    parser: ParserType,
}

impl<ReaderType, DecoderType, ParserType> Layer for Gate<ReaderType, DecoderType, ParserType> {
    type Layer = ApplicationLayer;
}

impl<ReaderType, DecoderType, ParserType> ApplicationService
    for Gate<ReaderType, DecoderType, ParserType>
{
}

impl<ReaderType: Reader, DecoderType: Decoder, ParserType: Parser>
    SynchronousQueryHandler<PathJudgment> for Gate<ReaderType, DecoderType, ParserType>
{
    fn resolve_query(&self, path_judgment: PathJudgment) -> Result<(), Errors> {
        let paths = path_judgment.paths();
        self.try_judge_paths(paths)
    }
}

impl<ReaderType: Reader, DecoderType: Decoder, ParserType: Parser>
    SynchronousQueryHandler<TermJudgment> for Gate<ReaderType, DecoderType, ParserType>
{
    fn resolve_query(&self, term_judgment: TermJudgment) -> Result<(), Errors> {
        let paths = term_judgment.paths();
        self.try_judge_terms(paths)
    }
}

impl<ReaderType: Reader, DecoderType: Decoder, ParserType: Parser>
    SynchronousQueryHandler<TokenSelection> for Gate<ReaderType, DecoderType, ParserType>
{
    fn resolve_query(&self, token_selection: TokenSelection) -> Result<TokenReport, Errors> {
        let paths = token_selection.paths();
        let source_tokens = self.try_select_production_tokens(paths)?;
        let token_report = TokenReport::from(source_tokens);
        Ok(token_report)
    }
}

impl<ReaderType: Reader, DecoderType: Decoder, ParserType: Parser>
    Gate<ReaderType, DecoderType, ParserType>
{
    const MANIFEST_NAME: &'static str = "Cargo.toml";

    fn try_judge_paths(&self, paths: &[PathBuf]) -> Result<(), Errors> {
        let source_paths = self.try_list_sources(paths)?;
        let examinations = self.select_examinations(&source_paths);
        let mut error_list = Vec::new();
        for examination in examinations {
            let source_judgment = |source_examination: Examination| {
                source_examination.try_judge_source()
            };
            let judgment = examination.and_then(source_judgment);
            if let Err(error) = judgment {
                error_list.push(error);
            }
        }
        self.try_compose_judgment(error_list)
    }

    fn try_judge_terms(&self, paths: &[PathBuf]) -> Result<(), Errors> {
        let source_paths = self.try_list_sources(paths)?;
        let mut error_list = Vec::new();
        let mut path_spellings = BTreeMap::new();
        let examinations = self.select_examinations(&source_paths);
        for (source_path, examination) in source_paths.iter().zip(examinations) {
            match examination {
                Ok(examination) => {
                    let spellings = examination.select_production_spellings();
                    let path = source_path.clone();
                    path_spellings.insert(path, spellings);
                }
                Err(error) => error_list.push(error),
            }
        }
        let source_spellings = SourceSpellings::from(path_spellings);
        let term_errors = self.select_term_errors(&source_spellings);
        error_list.extend(term_errors);
        self.try_compose_judgment(error_list)
    }

    fn try_select_production_tokens(&self, paths: &[PathBuf]) -> Result<SourceTokens, Errors> {
        let source_paths = self.try_list_sources(paths)?;
        let mut error_list = Vec::new();
        let mut path_tokens = BTreeMap::new();
        for source_path in &source_paths {
            match self.try_read_source(source_path) {
                Ok(source) => {
                    let production_tokens = source.production_tokens().clone();
                    let path = source_path.clone();
                    path_tokens.insert(path, production_tokens);
                }
                Err(error) => error_list.push(error),
            }
        }
        match error_list.is_empty() {
            true => {
                let source_tokens = SourceTokens::from(path_tokens);
                Ok(source_tokens)
            }
            false => {
                let errors = error_list.into_iter().collect();
                Err(errors)
            }
        }
    }

    fn try_list_sources(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>, Errors> {
        let mut source_paths = Vec::new();
        let mut error_list = Vec::new();
        for path in paths {
            match self.try_list_path_sources(path) {
                Ok(path_sources) => source_paths.extend(path_sources),
                Err(error) => error_list.push(error),
            }
        }
        match error_list.is_empty() {
            true => Ok(source_paths),
            false => {
                let errors = error_list.into_iter().collect();
                Err(errors)
            }
        }
    }

    fn try_list_path_sources(&self, path: &Path) -> Result<Vec<PathBuf>, Error> {
        let Self { reader, .. } = self;
        let file_error = |error: io::Error| {
            let file_path = path.to_path_buf();
            Error::File {
                path: file_path,
                error,
            }
        };
        let source_paths = reader.try_list_sources(path).map_err(file_error)?;
        match source_paths.is_empty() {
            true => {
                let directory_path = path.to_path_buf();
                let source_absence = Error::SourceAbsence {
                    path: directory_path,
                };
                Err(source_absence)
            }
            false => Ok(source_paths),
        }
    }

    fn select_examinations(&self, source_paths: &[PathBuf]) -> Vec<Result<Examination, Error>> {
        let mut crate_examinations = BTreeMap::new();
        let mut examinations = Vec::new();
        for source_path in source_paths {
            let examination = self.try_read_source(source_path).and_then(|source| {
                let crate_directory = self.try_locate_crate_directory(source_path)?;
                let crate_examination = self.try_select_crate_examination(
                    source_path,
                    crate_directory.as_ref(),
                    &crate_examinations,
                )?;
                if let Some(crate_directory) = crate_directory {
                    crate_examinations.insert(crate_directory, Rc::clone(&crate_examination));
                }
                let path = source_path.clone();
                let examination = Examination::from((path, source, crate_examination));
                Ok(examination)
            });
            examinations.push(examination);
        }
        examinations
    }

    fn try_select_crate_examination(
        &self,
        source_path: &Path,
        crate_directory: Option<&PathBuf>,
        crate_examinations: &BTreeMap<PathBuf, Rc<CrateExamination>>,
    ) -> Result<Rc<CrateExamination>, Error> {
        match crate_directory.and_then(|directory| crate_examinations.get(directory)) {
            Some(crate_examination) => Ok(Rc::clone(crate_examination)),
            None => {
                let crate_examination = self.try_compose_crate_examination(source_path)?;
                Ok(Rc::new(crate_examination))
            }
        }
    }

    fn try_compose_judgment(&self, error_list: Vec<Error>) -> Result<(), Errors> {
        match error_list.is_empty() {
            true => Ok(()),
            false => {
                let errors = error_list.into_iter().collect();
                Err(errors)
            }
        }
    }

    fn try_locate_crate_directory(&self, source_path: &Path) -> Result<Option<PathBuf>, Error> {
        let Self { reader, .. } = self;
        let directory_criterion = |directory: &&Path| *directory != source_path;
        for directory in source_path.ancestors().filter(directory_criterion) {
            let manifest_path = directory.join(Self::MANIFEST_NAME);
            match reader.try_open_file(&manifest_path) {
                Ok(_) => {
                    let crate_directory = directory.to_path_buf();
                    return Ok(Some(crate_directory));
                }
                Err(error) if error.kind() == ErrorKind::NotFound => {}
                Err(error) => {
                    let path = manifest_path;
                    return Err(Error::File { path, error });
                }
            }
        }
        Ok(None)
    }

    fn try_compose_crate_examination(&self, source_path: &Path) -> Result<CrateExamination, Error> {
        let Self { languages, .. } = self;
        let dependencies = self.try_read_dependencies(source_path)?;
        let package_identifier = dependencies.package_identifier().clone();
        let production_dependencies = dependencies.production_dependencies();
        let development_dependencies = dependencies.development_dependencies();
        let test_word_language = languages.test_word_language();
        let context_language = languages.compose_context_language(source_path);
        let production_dependency_language = languages
            .compose_dependency_language(production_dependencies);
        let development_dependency_language = languages
            .compose_dependency_language(development_dependencies);
        let production_language = &context_language | &production_dependency_language;
        let test_context_language = &production_language | test_word_language;
        let test_language = &test_context_language | &development_dependency_language;
        let crate_examination = CrateExamination::from((
            package_identifier,
            production_language,
            test_language,
        ));
        Ok(crate_examination)
    }

    fn select_term_errors(&self, source_spellings: &SourceSpellings) -> Errors {
        let Self { languages, .. } = self;
        let contexts = languages.contexts();
        let mut error_list = Vec::new();
        for context in contexts {
            let unused_terms = context.select_unused_terms(source_spellings);
            let decision = unused_terms.decide_acceptance();
            match decision {
                CandidateDecision::Acceptance => {}
                CandidateDecision::Rejection => {
                    let directory = context.directory();
                    let directory_path = directory.path();
                    let path = directory_path.to_path_buf();
                    let error = Error::Terms { path, unused_terms };
                    error_list.push(error);
                }
            }
        }
        error_list.into_iter().collect()
    }

    fn try_read_source(&self, path: &Path) -> Result<Source, Error> {
        let Self { reader, parser, .. } = self;
        let file_error = |error: io::Error| {
            let file_path = path.to_path_buf();
            Error::File {
                path: file_path,
                error,
            }
        };
        let syntax_error = |error: io::Error| {
            let source_path = path.to_path_buf();
            Error::Syntax {
                path: source_path,
                error,
            }
        };
        let file = reader.try_open_file(path).map_err(file_error)?;
        let text = io::read_to_string(file).map_err(file_error)?;
        let file_text = text.as_str();
        let source_text = SourceText::from(file_text);
        parser.try_parse_source(&source_text).map_err(syntax_error)
    }

    fn try_read_dependencies(&self, source_path: &Path) -> Result<Dependencies, Error> {
        let directory_criterion = |directory: &&Path| *directory != source_path;
        let source_directories = source_path.ancestors().filter(directory_criterion);
        let crate_dependencies = self.try_read_crate_dependencies(source_directories)?;
        let package_identifier = crate_dependencies.package_identifier().clone();
        let production_dependencies = crate_dependencies.production_dependencies();
        let development_dependencies = crate_dependencies.development_dependencies();
        let production_closure = self.try_close_dependency_list(production_dependencies)?;
        let development_dependency_closure = self
            .try_close_dependency_list(development_dependencies)?;
        let dependencies = Dependencies::from((
            package_identifier,
            production_closure,
            development_dependency_closure,
        ));
        Ok(dependencies)
    }

    fn try_close_dependency_list(
        &self,
        dependency_list: &DependencyList,
    ) -> Result<DependencyList, Error> {
        let mut closure_paths = BTreeSet::new();
        let mut pending_paths = dependency_list.paths().to_vec();
        while let Some(dependency_path) = pending_paths.pop() {
            let closure_path = dependency_path.clone();
            if closure_paths.insert(closure_path) {
                let dependency_directories = dependency_path.ancestors();
                let crate_dependencies = self
                    .try_read_crate_dependencies(dependency_directories)?;
                let production_dependencies = crate_dependencies.production_dependencies();
                let dependency_paths = production_dependencies.paths();
                pending_paths.extend_from_slice(dependency_paths);
            }
        }
        let closure_list: DependencyList = closure_paths.into_iter().collect();
        let closure = dependency_list | &closure_list;
        Ok(closure)
    }

    fn try_read_crate_dependencies<'path>(
        &self,
        directories: impl Iterator<Item = &'path Path>,
    ) -> Result<Dependencies, Error> {
        let mut manifests: Vec<Manifest> = Vec::new();
        for directory in directories {
            let manifest_presence = self.try_read_manifest(directory)?;
            match manifest_presence {
                ManifestPresence::Presence { manifest } => manifests.push(manifest),
                ManifestPresence::Absence => {}
            }
        }
        let manifest_list = manifests.as_slice();
        let dependencies = Dependencies::from(manifest_list);
        Ok(dependencies)
    }

    fn try_read_manifest(&self, directory: &Path) -> Result<ManifestPresence, Error> {
        let Self {
            reader, decoder, ..
        } = self;
        let manifest_name = Self::MANIFEST_NAME;
        let manifest_path = directory.join(manifest_name);
        let file = match reader.try_open_file(&manifest_path) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                let manifest_absence = ManifestPresence::Absence;
                return Ok(manifest_absence);
            }
            Err(error) => {
                let path = manifest_path;
                let manifest_file_error = Error::File { path, error };
                return Err(manifest_file_error);
            }
        };
        let manifest_file = ManifestFile::from((file, manifest_path, directory));
        let manifest = manifest_file.try_decode_manifest(decoder)?;
        let manifest_presence = ManifestPresence::Presence { manifest };
        Ok(manifest_presence)
    }
}

impl<VocabularyType, ReaderType, DecoderType, ParserType>
    TryFrom<(VocabularyType, ReaderType, DecoderType, ParserType)>
    for Gate<ReaderType, DecoderType, ParserType>
where
    VocabularyType: Vocabulary,
{
    type Error = Error;

    fn try_from(
        (vocabulary, reader, decoder, parser): (
            VocabularyType,
            ReaderType,
            DecoderType,
            ParserType,
        ),
    ) -> Result<Self, Self::Error> {
        let context_list = vocabulary.try_list_contexts()?;
        let foreign_language_list = vocabulary.try_list_foreign_languages()?;
        let gate = |languages: Languages| Self {
            languages,
            reader,
            decoder,
            parser,
        };
        let languages = context_list.try_compose_languages(foreign_language_list);
        languages.map(gate)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::gate::tests::fixture as entry_fixture;
    use crate::gate::tests::fixture as source_fixture;
    use crate::vocabulary::{ContextList, ForeignLanguageList};
    use gates_vocabulary_language_core_dependencies::manifest::entries::Entries;
    use gates_vocabulary_language_core_language::identifier::Identifier;
    use gates_vocabulary_language_core_language::{Language, TermText};
    use gates_vocabulary_language_core_languages::contexts::Context;
    use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
    use stack_domain_driven_design_core::layer::InfrastructureLayer;
    use stack_domain_driven_design_core::port::{DrivenPort, Port};
    use std::io::{Cursor, Read};

    pub(crate) mod fixture {
        use super::*;
        use gates_vocabulary_language_core_dependencies::manifest::entries::Entry;
        use gates_vocabulary_language_core_foreign_name::ForeignName;
        use gates_vocabulary_language_core_language::Code;
        use gates_vocabulary_language_core_language::code::tokens::CodeTokens;
        use gates_vocabulary_language_core_language::code::tokens::code_token::CodeToken;
        use gates_vocabulary_language_core_language::defects::Defects;
        use gates_vocabulary_language_core_language::defects::defect::Defect;
        use gates_vocabulary_language_core_language::occurrence::Occurrence;
        use gates_vocabulary_language_core_language::source::Source;
        use std::path::Path;

        fn occurrences_factory<OccurrencesType: FromIterator<Occurrence>>(
            texts: &[&str],
        ) -> OccurrencesType {
            let line: usize = 1;
            let mut occurrence_list = Vec::new();
            for (position, text) in texts.iter().copied().enumerate() {
                let column = position + 1;
                let occurrence = Occurrence::try_from((text, line, column)).unwrap();
                occurrence_list.push(occurrence);
            }
            occurrence_list.into_iter().collect()
        }

        pub(crate) fn identifier_source_factory(identifier_texts: &[&str]) -> Source {
            let declaration_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn declaration_source_factory(declaration_texts: &[&str]) -> Source {
            let identifier_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn test_declaration_source_factory(declaration_texts: &[&str]) -> Source {
            let identifier_texts: &[&str] = &[];
            let production_code = Code::default();
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let test_code = Code::from((identifiers, declarations));
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn token_source_factory() -> Source {
            let production_code = Code::default();
            let line: usize = 1;
            let column: usize = 4;
            let text = String::from("Treasure");
            let code_token = CodeToken::Declaration { line, column, text };
            let code_token_list = [code_token];
            let production_tokens = code_token_list.into_iter().collect();
            let test_code = Code::default();
            let defects = Defects::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn comment_source_factory(identifier_texts: &[&str]) -> Source {
            let declaration_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let line: usize = 2;
            let column: usize = 1;
            let comment = Defect::Comment { line, column };
            let defect_list = [comment];
            let defects = defect_list.into_iter().collect();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn foreign_name_entry_factory(foreign_name_text: &str) -> Entry {
            let foreign_name = ForeignName::from(foreign_name_text);
            let path = None;
            Entry::from((foreign_name, path))
        }

        pub(crate) fn path_entry_factory(crate_path_text: &str) -> Entry {
            let crate_path = Path::new(crate_path_text);
            let directory_name = crate_path.file_name().unwrap();
            let foreign_name_text = directory_name.to_str().unwrap();
            let foreign_name = ForeignName::from(foreign_name_text);
            let entry_path = crate_path.to_path_buf();
            let path = Some(entry_path);
            Entry::from((foreign_name, path))
        }

        use gates_vocabulary_language_core_dependencies::manifest::ManifestText;
        use stack_domain_driven_design_core::adapter::Adapter;

        pub(crate) const SOURCE_PATH: &str = "/workspace/crates/ship/src/lib.rs";
        pub(crate) const MANIFEST_PATH: &str = "/workspace/crates/ship/Cargo.toml";
        pub(crate) const ROOT_MANIFEST_PATH: &str = "/workspace/Cargo.toml";
        pub(crate) const DIRECTORY_SOURCE_PATH: &str = "/workspace/crates/harbor/src/lib.rs";
        pub(crate) const DIRECTORY_PATH: &str = "/workspace/crates/ship";

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(crate) enum File {
            Text { text: &'static str },
            Source { source: Source },
            Manifest { manifest: Manifest },
            Directory { source_paths: Vec<PathBuf> },
            OpenError,
            ReadError,
            DirectoryError,
            Absence,
        }

        impl Layer for File {
            type Layer = InfrastructureLayer;
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(crate) struct Workspace {
            contexts: Vec<Context>,
            foreign_languages: Vec<ForeignLanguage>,
            files: BTreeMap<PathBuf, File>,
        }

        impl Layer for Workspace {
            type Layer = InfrastructureLayer;
        }

        impl From<(Vec<Context>, Vec<ForeignLanguage>, BTreeMap<PathBuf, File>)> for Workspace {
            fn from(
                (contexts, foreign_languages, files): (
                    Vec<Context>,
                    Vec<ForeignLanguage>,
                    BTreeMap<PathBuf, File>,
                ),
            ) -> Self {
                Self {
                    contexts,
                    foreign_languages,
                    files,
                }
            }
        }

        pub(crate) type FixtureGate = Gate<ReaderAdapter, DecoderAdapter, ParserAdapter>;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub(crate) struct VocabularyAdapter {
            pub(crate) contexts: Vec<Context>,
            pub(crate) foreign_languages: Vec<ForeignLanguage>,
        }

        impl Layer for VocabularyAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Port for VocabularyAdapter {}

        impl Vocabulary for VocabularyAdapter {
            fn try_list_contexts(&self) -> Result<ContextList, Error> {
                let Self { contexts, .. } = self;
                let context_list = contexts.iter().cloned().collect();
                Ok(context_list)
            }

            fn try_list_foreign_languages(&self) -> Result<ForeignLanguageList, Error> {
                let Self {
                    foreign_languages, ..
                } = self;
                let foreign_language_list = foreign_languages.iter().cloned().collect();
                Ok(foreign_language_list)
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
        pub(crate) enum VocabularyErrorAdapter {
            Context,
            ForeignLanguage,
        }

        impl Layer for VocabularyErrorAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Port for VocabularyErrorAdapter {}

        impl Vocabulary for VocabularyErrorAdapter {
            fn try_list_contexts(&self) -> Result<ContextList, Error> {
                match self {
                    Self::Context => {
                        let path_text = "contexts/treasure";
                        let path = PathBuf::from(path_text);
                        let error = Error::SourceAbsence { path };
                        Err(error)
                    }
                    Self::ForeignLanguage => {
                        let context_list = ContextList::default();
                        Ok(context_list)
                    }
                }
            }

            fn try_list_foreign_languages(&self) -> Result<ForeignLanguageList, Error> {
                let path_text = "foreign_languages/anchor";
                let path = PathBuf::from(path_text);
                let error = Error::SourceAbsence { path };
                Err(error)
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub(crate) struct ReaderAdapter {
            files: BTreeMap<PathBuf, File>,
        }

        impl Layer for ReaderAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for ReaderAdapter {}

        impl Port for ReaderAdapter {}

        impl DrivenPort for ReaderAdapter {}

        impl Reader for ReaderAdapter {
            fn try_open_file(&self, path: &Path) -> io::Result<Box<dyn Read + Send>> {
                let Self { files } = self;
                let absence = File::Absence;
                let file = files.get(path).unwrap_or(&absence);
                match file {
                    File::Text { text } => {
                        let file_text: &'static str = text;
                        let cursor = Cursor::new(file_text);
                        let reader: Box<dyn Read + Send> = Box::new(cursor);
                        Ok(reader)
                    }
                    File::Source { source } => {
                        let source_text = format!("{source:?}");
                        let cursor = Cursor::new(source_text);
                        let reader: Box<dyn Read + Send> = Box::new(cursor);
                        Ok(reader)
                    }
                    File::Manifest { manifest } => {
                        let manifest_text = format!("{manifest:?}");
                        let cursor = Cursor::new(manifest_text);
                        let reader: Box<dyn Read + Send> = Box::new(cursor);
                        Ok(reader)
                    }
                    File::OpenError => {
                        let error_text = "refused";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                    File::ReadError => {
                        let error_file = ErrorFile;
                        let reader: Box<dyn Read + Send> = Box::new(error_file);
                        Ok(reader)
                    }
                    File::Directory { .. } | File::DirectoryError | File::Absence => {
                        let error_kind = ErrorKind::NotFound;
                        let error = io::Error::from(error_kind);
                        Err(error)
                    }
                }
            }

            fn try_list_sources(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
                let Self { files } = self;
                match files.get(path) {
                    Some(File::Directory { source_paths }) => Ok(source_paths.clone()),
                    Some(File::DirectoryError) => {
                        let error_text = "refused";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                    Some(_) | None => {
                        let source_path = path.to_path_buf();
                        Ok(vec![source_path])
                    }
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        struct ErrorFile;

        impl Layer for ErrorFile {
            type Layer = InfrastructureLayer;
        }

        impl Read for ErrorFile {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                let error_text = "torn";
                let error = io::Error::other(error_text);
                Err(error)
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub(crate) struct DecoderAdapter {
            manifests: BTreeMap<String, Manifest>,
        }

        impl Layer for DecoderAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for DecoderAdapter {}

        impl Port for DecoderAdapter {}

        impl DrivenPort for DecoderAdapter {}

        impl Decoder for DecoderAdapter {
            fn try_decode_manifest(&self, manifest_text: &ManifestText) -> io::Result<Manifest> {
                let text = manifest_text.text();
                let Self { manifests } = self;
                let manifest = manifests.get(text);
                match manifest {
                    Some(text_manifest) => {
                        let decoder_manifest = text_manifest.clone();
                        Ok(decoder_manifest)
                    }
                    None => {
                        let error_text = "no manifest";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub(crate) struct ParserAdapter {
            sources: BTreeMap<String, Source>,
        }

        impl Layer for ParserAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for ParserAdapter {}

        impl Port for ParserAdapter {}

        impl DrivenPort for ParserAdapter {}

        impl Parser for ParserAdapter {
            fn try_parse_source(&self, source_text: &SourceText) -> io::Result<Source> {
                let text = source_text.text();
                let Self { sources } = self;
                let source = sources.get(text);
                match source {
                    Some(text_source) => {
                        let parser_source = text_source.clone();
                        Ok(parser_source)
                    }
                    None => {
                        let error_text = "1:1: the source is not valid Rust";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                }
            }
        }

        pub(crate) fn context_factory(directory_name: &'static str) -> Context {
            const TERM_TEXTS: &[TermText] = &[
                {
                    let spelling: &[&str] = &["Treasure"];
                    let definition = "A thing worth the search.";
                    TermText::new(spelling, definition)
                },
                {
                    let spelling: &[&str] = &["Chest"];
                    let definition = "A box that holds a treasure.";
                    TermText::new(spelling, definition)
                },
            ];
            let term_texts = TERM_TEXTS;
            let language = Language::try_from(term_texts).unwrap();
            Context::from((directory_name, language))
        }

        pub(crate) fn foreign_language_factory() -> ForeignLanguage {
            let name_texts: &[&str] = &["Anchor"];
            let foreign_name_text = "anchor";
            let foreign_name = ForeignName::from(foreign_name_text);
            ForeignLanguage::try_from((foreign_name, name_texts)).unwrap()
        }

        pub(crate) fn manifest_factory(dependency_list: Vec<Entry>) -> Manifest {
            let package_identifier = Identifier::default();
            let dependency_entries = Entries::try_from(dependency_list).unwrap();
            let development_dependency_entries = Entries::default();
            let workspace_entries = Entries::default();
            Manifest::from((
                package_identifier,
                dependency_entries,
                development_dependency_entries,
                workspace_entries,
            ))
        }

        pub(crate) fn root_manifest_factory(workspace_list: Vec<Entry>) -> Manifest {
            let package_identifier = Identifier::default();
            let dependency_entries = Entries::default();
            let development_dependency_entries = Entries::default();
            let workspace_entries = Entries::try_from(workspace_list).unwrap();
            Manifest::from((
                package_identifier,
                dependency_entries,
                development_dependency_entries,
                workspace_entries,
            ))
        }

        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub(crate) struct Files {
            pub(crate) files: BTreeMap<PathBuf, File>,
        }

        pub(crate) fn files_factory(source_file: File) -> Files {
            let source_path_text = SOURCE_PATH;
            let source_path = PathBuf::from(source_path_text);
            let mut files = BTreeMap::new();
            files.insert(source_path, source_file);
            Files { files }
        }

        pub(crate) fn gate_factory(workspace: Workspace) -> FixtureGate {
            let Workspace {
                contexts,
                foreign_languages,
                files,
            } = workspace;
            let mut manifests = BTreeMap::new();
            let mut sources = BTreeMap::new();
            for file in files.values() {
                match file {
                    File::Manifest { manifest } => {
                        let manifest_text = format!("{manifest:?}");
                        let decoder_manifest = manifest.clone();
                        manifests.insert(manifest_text, decoder_manifest);
                    }
                    File::Source { source } => {
                        let source_text = format!("{source:?}");
                        let parser_source = source.clone();
                        sources.insert(source_text, parser_source);
                    }
                    File::Text { .. }
                    | File::Directory { .. }
                    | File::OpenError
                    | File::ReadError
                    | File::DirectoryError
                    | File::Absence => {}
                }
            }
            let reader_adapter = ReaderAdapter { files };
            let decoder_adapter = DecoderAdapter { manifests };
            let parser_adapter = ParserAdapter { sources };
            let vocabulary_adapter = VocabularyAdapter {
                contexts,
                foreign_languages,
            };
            Gate::try_from((
                vocabulary_adapter,
                reader_adapter,
                decoder_adapter,
                parser_adapter,
            ))
            .unwrap()
        }

        pub(crate) fn crate_gate_factory(source_file: File) -> FixtureGate {
            let crate_name = "ship";
            let crate_context = context_factory(crate_name);
            let contexts = vec![crate_context];
            let foreign_languages = Vec::new();
            let Files { files } = files_factory(source_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn source_directory_gate_factory(directory_file: File) -> FixtureGate {
            let crate_name = "ship";
            let crate_context = context_factory(crate_name);
            let contexts = vec![crate_context];
            let foreign_languages = Vec::new();
            let declaration_texts: &[&str] = &["Zonk"];
            let source = source_fixture::declaration_source_factory(declaration_texts);
            let source_file = File::Source { source };
            let Files { mut files } = files_factory(source_file);
            let directory_path_text = DIRECTORY_PATH;
            let directory_path = PathBuf::from(directory_path_text);
            files.insert(directory_path, directory_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn directory_gate_factory(source_file: File) -> FixtureGate {
            let directory_name = "harbor";
            let directory_context = context_factory(directory_name);
            let contexts = vec![directory_context];
            let foreign_languages = Vec::new();
            let Files { files } = files_factory(source_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn manifest_gate_factory(manifest_file: File) -> FixtureGate {
            let declaration_texts: &[&str] = &["Treasure"];
            let source = source_fixture::declaration_source_factory(declaration_texts);
            let source_file = File::Source { source };
            let crate_name = "ship";
            let crate_context = context_factory(crate_name);
            let contexts = vec![crate_context];
            let foreign_languages = Vec::new();
            let manifest_path_text = MANIFEST_PATH;
            let manifest_path = PathBuf::from(manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn root_manifest_gate_factory(root_manifest_file: File) -> FixtureGate {
            let declaration_texts: &[&str] = &["Treasure"];
            let source = source_fixture::declaration_source_factory(declaration_texts);
            let source_file = File::Source { source };
            let crate_name = "ship";
            let crate_context = context_factory(crate_name);
            let contexts = vec![crate_context];
            let foreign_languages = Vec::new();
            let root_manifest_path_text = ROOT_MANIFEST_PATH;
            let root_manifest_path = PathBuf::from(root_manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(root_manifest_path, root_manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn crate_root_manifest_gate_factory(root_manifest_file: File) -> FixtureGate {
            let declaration_texts: &[&str] = &["Treasure"];
            let source = source_fixture::declaration_source_factory(declaration_texts);
            let source_file = File::Source { source };
            let crate_name = "ship";
            let crate_context = context_factory(crate_name);
            let contexts = vec![crate_context];
            let foreign_languages = Vec::new();
            let dependency_list = Vec::new();
            let manifest = manifest_factory(dependency_list);
            let manifest_file = File::Manifest { manifest };
            let manifest_path_text = MANIFEST_PATH;
            let root_manifest_path_text = ROOT_MANIFEST_PATH;
            let manifest_path = PathBuf::from(manifest_path_text);
            let root_manifest_path = PathBuf::from(root_manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            files.insert(root_manifest_path, root_manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn workspace_gate_factory(source_file: File) -> FixtureGate {
            let directory_name = "harbor";
            let directory_context = context_factory(directory_name);
            let contexts = vec![directory_context];
            let foreign_languages = Vec::new();
            let directory_path_text = "crates/harbor";
            let directory_entry = entry_fixture::foreign_name_entry_factory(directory_name);
            let workspace_entry = entry_fixture::path_entry_factory(directory_path_text);
            let dependency_list = vec![directory_entry];
            let workspace_list = vec![workspace_entry];
            let manifest = manifest_factory(dependency_list);
            let root_manifest = root_manifest_factory(workspace_list);
            let manifest_file = File::Manifest { manifest };
            let root_manifest_file = File::Manifest {
                manifest: root_manifest,
            };
            let declaration_texts: &[&str] = &["treasure"];
            let directory_source = source_fixture::declaration_source_factory(declaration_texts);
            let directory_source_file = File::Source {
                source: directory_source,
            };
            let manifest_path_text = MANIFEST_PATH;
            let root_manifest_path_text = ROOT_MANIFEST_PATH;
            let directory_source_path_text = DIRECTORY_SOURCE_PATH;
            let manifest_path = PathBuf::from(manifest_path_text);
            let root_manifest_path = PathBuf::from(root_manifest_path_text);
            let directory_source_path = PathBuf::from(directory_source_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            files.insert(root_manifest_path, root_manifest_file);
            files.insert(directory_source_path, directory_source_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn foreign_language_gate_factory(source_file: File) -> FixtureGate {
            let foreign_name_text = "anchor";
            let foreign_name_entry = entry_fixture::foreign_name_entry_factory(foreign_name_text);
            let dependency_list = vec![foreign_name_entry];
            let manifest = manifest_factory(dependency_list);
            let manifest_file = File::Manifest { manifest };
            let contexts = Vec::new();
            let foreign_language = foreign_language_factory();
            let foreign_languages = vec![foreign_language];
            let manifest_path_text = MANIFEST_PATH;
            let manifest_path = PathBuf::from(manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn development_dependency_gate_factory(source_file: File) -> FixtureGate {
            let directory_name = "harbor";
            let directory_context = context_factory(directory_name);
            let contexts = vec![directory_context];
            let foreign_languages = Vec::new();
            let directory_path_text = "../harbor";
            let directory_entry = entry_fixture::path_entry_factory(directory_path_text);
            let development_dependency_list = vec![directory_entry];
            let package_identifier = Identifier::default();
            let dependency_entries = Entries::default();
            let development_dependency_entries = Entries::try_from(development_dependency_list)
                .unwrap();
            let workspace_entries = Entries::default();
            let manifest = Manifest::from((
                package_identifier,
                dependency_entries,
                development_dependency_entries,
                workspace_entries,
            ));
            let manifest_file = File::Manifest { manifest };
            let manifest_path_text = MANIFEST_PATH;
            let manifest_path = PathBuf::from(manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn development_dependency_closure_gate_factory(
            directory_manifest_file: File,
        ) -> FixtureGate {
            let declaration_texts: &[&str] = &["Treasure"];
            let source = source_fixture::test_declaration_source_factory(declaration_texts);
            let source_file = File::Source { source };
            let contexts = Vec::new();
            let foreign_languages = Vec::new();
            let directory_path_text = "../harbor";
            let directory_entry = entry_fixture::path_entry_factory(directory_path_text);
            let development_dependency_list = vec![directory_entry];
            let package_identifier = Identifier::default();
            let dependency_entries = Entries::default();
            let development_dependency_entries = Entries::try_from(development_dependency_list)
                .unwrap();
            let workspace_entries = Entries::default();
            let manifest = Manifest::from((
                package_identifier,
                dependency_entries,
                development_dependency_entries,
                workspace_entries,
            ));
            let manifest_file = File::Manifest { manifest };
            let manifest_path_text = MANIFEST_PATH;
            let directory_manifest_path_text = "/workspace/crates/harbor/Cargo.toml";
            let manifest_path = PathBuf::from(manifest_path_text);
            let directory_manifest_path = PathBuf::from(directory_manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            files.insert(directory_manifest_path, directory_manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }

        pub(crate) fn closure_gate_factory(directory_manifest_file: File) -> FixtureGate {
            let declaration_texts: &[&str] = &["Treasure"];
            let source = source_fixture::declaration_source_factory(declaration_texts);
            let source_file = File::Source { source };
            let directory_name = "lighthouse";
            let directory_context = context_factory(directory_name);
            let contexts = vec![directory_context];
            let foreign_languages = Vec::new();
            let dependency_path_text = "../harbor";
            let dependency_entry = entry_fixture::path_entry_factory(dependency_path_text);
            let dependency_list = vec![dependency_entry];
            let manifest = manifest_factory(dependency_list);
            let manifest_file = File::Manifest { manifest };
            let manifest_path_text = MANIFEST_PATH;
            let directory_manifest_path_text = "/workspace/crates/harbor/Cargo.toml";
            let manifest_path = PathBuf::from(manifest_path_text);
            let directory_manifest_path = PathBuf::from(directory_manifest_path_text);
            let Files { mut files } = files_factory(source_file);
            files.insert(manifest_path, manifest_file);
            files.insert(directory_manifest_path, directory_manifest_file);
            let workspace = Workspace::from((contexts, foreign_languages, files));
            gate_factory(workspace)
        }
    }

    pub(crate) mod helper {
        use super::*;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(crate) struct ErrorTexts {
            pub(crate) error_texts: Vec<String>,
        }

        pub(crate) fn error_texts(errors: Errors) -> ErrorTexts {
            let error_text = |error: Error| error.to_string();
            let error_texts = errors.into_iter().map(error_text).collect();
            ErrorTexts { error_texts }
        }
    }

    #[test]
    fn a_source_in_the_language_of_its_context_is_let_through() {
        let declaration_texts: &[&str] = &["Treasure", "chest"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_with_words_of_no_language_named_with_their_lines_and_columns_is_an_error() {
        let declaration_texts: &[&str] = &["Zonk", "flarn", "Kwib"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            r#"[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Zonk` is not made of words of this crate's vocabulary.
[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:2: `flarn` is not made of words of this crate's vocabulary.
[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:3: `Kwib` is not made of words of this crate's vocabulary."#,
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_with_words_of_no_language_and_defects_named_on_a_line_each_is_an_error() {
        let identifier_texts: &[&str] = &["Zonk", "Treasure"];
        let source = source_fixture::comment_source_factory(identifier_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            r#"[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Zonk` is not made of words of this crate's vocabulary.
[LANGUAGE] /workspace/crates/ship/src/lib.rs:2:1: the `doc` attribute is a comment, and code must not contain comments."#,
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_one_context_in_a_source_of_another_is_an_error() {
        let declaration_texts: &[&str] = &["Treasure"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::directory_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Treasure` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_crate_the_root_manifest_leads_to_is_let_through() {
        let declaration_texts: &[&str] = &["Treasure"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::workspace_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_crate_a_dependency_depends_on_by_path_is_let_through() {
        let directory_path_text = "../lighthouse";
        let directory_entry = entry_fixture::path_entry_factory(directory_path_text);
        let dependency_list = vec![directory_entry];
        let manifest = fixture::manifest_factory(dependency_list);
        let manifest_file = fixture::File::Manifest { manifest };
        let gate = fixture::closure_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_crate_no_dependency_leads_to_is_an_error() {
        let manifest_file = fixture::File::Absence;
        let gate = fixture::closure_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Treasure` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_dependency_whose_manifest_cannot_be_read_is_an_error() {
        let manifest_file = fixture::File::ReadError;
        let gate = fixture::closure_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/harbor/Cargo.toml`: torn."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_crate_the_manifest_names_is_let_through() {
        let identifier_texts: &[&str] = &["anchor", "Anchor"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::foreign_language_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_crate_no_manifest_names_at_every_occurrence_is_an_error() {
        let identifier_texts: &[&str] = &["anchor", "Anchor"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            r#"[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `anchor` is not made of words of this crate's vocabulary.
[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:2: `Anchor` is not made of words of this crate's vocabulary."#,
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_that_declares_a_name_of_a_foreign_language_is_an_error() {
        let declaration_texts: &[&str] = &["Anchor"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::foreign_language_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Anchor` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_test_module_passes_foreign_names_and_test_words_in_compounds_and_any_other_word_is_an_error()
     {
        let declaration_texts: &[&str] = &[
            "anchor_factory",
            "anchor_expectation",
            "treasure_factory",
        ];
        let source = source_fixture::test_declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::foreign_language_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:3: `treasure_factory` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_test_word_in_the_test_code_passes_and_in_the_production_code_is_an_error() {
        let declaration_texts: &[&str] = &["treasure_factory", "chest_expectation"];
        let production_source = source_fixture::declaration_source_factory(declaration_texts);
        let test_source = source_fixture::test_declaration_source_factory(declaration_texts);
        let production_file = fixture::File::Source {
            source: production_source,
        };
        let test_file = fixture::File::Source {
            source: test_source,
        };
        let test_path_text = "/workspace/crates/ship/src/tests/mod.rs";
        let test_path = PathBuf::from(test_path_text);
        let crate_name = "ship";
        let crate_context = fixture::context_factory(crate_name);
        let contexts = vec![crate_context];
        let foreign_languages = Vec::new();
        let fixture::Files { mut files } = fixture::files_factory(production_file);
        files.insert(test_path, test_file);
        let workspace = fixture::Workspace::from((contexts, foreign_languages, files));
        let gate = fixture::gate_factory(workspace);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let test_source_path = PathBuf::from(test_path_text);
        let source_paths = [source_path, test_source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            r#"[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `treasure_factory` is not made of words of this crate's vocabulary.
[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:2: `chest_expectation` is not made of words of this crate's vocabulary."#,
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_development_dependency_spells_the_test_code() {
        let declaration_texts: &[&str] = &["treasure_chest"];
        let source = source_fixture::test_declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::development_dependency_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_word_of_a_development_dependency_in_the_production_code_is_an_error() {
        let declaration_texts: &[&str] = &["treasure_chest"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::development_dependency_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `treasure_chest` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_that_is_no_rust_is_an_error() {
        let source_file = fixture::File::Text {
            text: "struct Treasure; 1",
        };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: the source is not valid Rust.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_that_could_not_be_opened_is_an_error() {
        let source_file = fixture::File::OpenError;
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship/src/lib.rs`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_that_could_not_be_read_is_an_error() {
        let source_file = fixture::File::ReadError;
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship/src/lib.rs`: torn."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_whose_manifest_could_not_be_opened_is_an_error() {
        let manifest_file = fixture::File::OpenError;
        let gate = fixture::manifest_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship/Cargo.toml`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_whose_root_manifest_could_not_be_read_is_an_error() {
        let root_manifest_file = fixture::File::ReadError;
        let gate = fixture::root_manifest_gate_factory(root_manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/Cargo.toml`: torn."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_whose_manifest_the_decoder_refuses_is_an_error() {
        let manifest_file = fixture::File::Text { text: "zonk" };
        let gate = fixture::manifest_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] `/workspace/crates/ship/Cargo.toml` is not a valid crate manifest: no manifest.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn every_term_a_source_spells_is_in_use() {
        let declaration_texts: &[&str] = &["treasure_chest"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_term_only_the_test_code_spells_is_an_error() {
        let declaration_texts: &[&str] = &["treasure"];
        let test_declaration_texts: &[&str] = &["chest"];
        let production_source = source_fixture::declaration_source_factory(declaration_texts);
        let test_source = source_fixture::test_declaration_source_factory(test_declaration_texts);
        let production_file = fixture::File::Source {
            source: production_source,
        };
        let test_file = fixture::File::Source {
            source: test_source,
        };
        let test_path_text = "/workspace/crates/ship/src/tests/mod.rs";
        let test_path = PathBuf::from(test_path_text);
        let crate_name = "ship";
        let crate_context = fixture::context_factory(crate_name);
        let contexts = vec![crate_context];
        let foreign_languages = Vec::new();
        let fixture::Files { mut files } = fixture::files_factory(production_file);
        files.insert(test_path, test_file);
        let workspace = fixture::Workspace::from((contexts, foreign_languages, files));
        let gate = fixture::gate_factory(workspace);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let test_source_path = PathBuf::from(test_path_text);
        let source_paths = [source_path, test_source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] `crates/ship` defines the term `Chest`, but no package name and no identifier in production code uses it.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_term_the_package_identifier_of_its_crate_spells_is_in_use() {
        let declaration_texts: &[&str] = &["treasure"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let package_identifier_text = "chest";
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let dependency_entries = Entries::default();
        let development_dependency_entries = Entries::default();
        let workspace_entries = Entries::default();
        let manifest = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        let manifest_file = fixture::File::Manifest { manifest };
        let crate_name = "ship";
        let crate_context = fixture::context_factory(crate_name);
        let contexts = vec![crate_context];
        let foreign_languages = Vec::new();
        let manifest_path_text = fixture::MANIFEST_PATH;
        let manifest_path = PathBuf::from(manifest_path_text);
        let fixture::Files { mut files } = fixture::files_factory(source_file);
        files.insert(manifest_path, manifest_file);
        let workspace = fixture::Workspace::from((contexts, foreign_languages, files));
        let gate = fixture::gate_factory(workspace);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_term_the_production_code_of_a_crate_that_depends_on_it_spells_is_in_use() {
        let declaration_texts: &[&str] = &["treasure_chest"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::workspace_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let directory_source_path_text = fixture::DIRECTORY_SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let directory_source_path = PathBuf::from(directory_source_path_text);
        let source_paths = [source_path, directory_source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_term_only_a_crate_outside_the_run_spells_is_an_error() {
        let declaration_texts: &[&str] = &["treasure_chest"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::workspace_gate_factory(source_file);
        let directory_source_path_text = fixture::DIRECTORY_SOURCE_PATH;
        let directory_source_path = PathBuf::from(directory_source_path_text);
        let source_paths = [directory_source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] `crates/harbor` defines the term `Chest`, but no package name and no identifier in production code uses it.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn the_terms_of_a_context_no_source_of_the_run_lies_in_are_not_judged() {
        let declaration_texts: &[&str] = &["Zonk"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::directory_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_source_whose_terms_cannot_be_judged_is_an_error() {
        let source_file = fixture::File::OpenError;
        let gate = fixture::directory_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_terms(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship/src/lib.rs`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_development_dependency_whose_manifest_cannot_be_read_is_an_error() {
        let directory_manifest_file = fixture::File::ReadError;
        let gate = fixture::development_dependency_closure_gate_factory(directory_manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/harbor/Cargo.toml`: torn."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_dependency_that_leads_back_to_its_crate_is_read_once_and_a_word_beyond_it_is_an_error() {
        let crate_path_text = "../ship";
        let crate_entry = entry_fixture::path_entry_factory(crate_path_text);
        let dependency_list = vec![crate_entry];
        let manifest = fixture::manifest_factory(dependency_list);
        let manifest_file = fixture::File::Manifest { manifest };
        let gate = fixture::closure_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Treasure` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn every_source_under_a_directory_is_judged() {
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = vec![source_path];
        let directory_file = fixture::File::Directory { source_paths };
        let gate = fixture::source_directory_gate_factory(directory_file);
        let directory_path_text = fixture::DIRECTORY_PATH;
        let directory_path = PathBuf::from(directory_path_text);
        let paths = [directory_path];

        let judgment = gate.try_judge_paths(&paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Zonk` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_directory_without_a_source_is_an_error() {
        let source_paths = Vec::new();
        let directory_file = fixture::File::Directory { source_paths };
        let gate = fixture::source_directory_gate_factory(directory_file);
        let directory_path_text = fixture::DIRECTORY_PATH;
        let directory_path = PathBuf::from(directory_path_text);
        let paths = [directory_path];

        let judgment = gate.try_judge_paths(&paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] there is no Rust source under `/workspace/crates/ship`."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn the_terms_of_a_directory_that_cannot_be_listed_are_an_error() {
        let directory_file = fixture::File::DirectoryError;
        let gate = fixture::source_directory_gate_factory(directory_file);
        let directory_path_text = fixture::DIRECTORY_PATH;
        let directory_path = PathBuf::from(directory_path_text);
        let paths = [directory_path];

        let judgment = gate.try_judge_terms(&paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn the_production_tokens_of_a_directory_that_cannot_be_listed_are_an_error() {
        let directory_file = fixture::File::DirectoryError;
        let gate = fixture::source_directory_gate_factory(directory_file);
        let directory_path_text = fixture::DIRECTORY_PATH;
        let directory_path = PathBuf::from(directory_path_text);
        let paths = [directory_path];

        let source_tokens = gate.try_select_production_tokens(&paths);

        let errors = source_tokens.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn the_production_tokens_of_a_source_are_named_with_its_path() {
        let source = source_fixture::token_source_factory();
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = [source_path];

        let source_tokens = gate.try_select_production_tokens(&paths);

        let token_text = source_tokens.unwrap().to_string();
        let expectation = r#"/workspace/crates/ship/src/lib.rs	1	4	declaration	Treasure
"#;
        assert_eq!(token_text, expectation);
    }

    #[test]
    fn the_production_tokens_of_a_source_that_could_not_be_opened_are_an_error() {
        let source_file = fixture::File::OpenError;
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = [source_path];

        let source_tokens = gate.try_select_production_tokens(&paths);

        let errors = source_tokens.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship/src/lib.rs`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_root_manifest_above_a_crate_manifest_that_could_not_be_opened_is_an_error() {
        let root_manifest_file = fixture::File::OpenError;
        let gate = fixture::crate_root_manifest_gate_factory(root_manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let source_paths = [source_path];

        let judgment = gate.try_judge_paths(&source_paths);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/Cargo.toml`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn the_sources_of_one_crate_share_the_examination_of_their_crate() {
        let dependency_list = Vec::new();
        let manifest = fixture::manifest_factory(dependency_list);
        let manifest_file = fixture::File::Manifest { manifest };
        let gate = fixture::manifest_gate_factory(manifest_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = [source_path.clone(), source_path];

        let judgment = gate.try_judge_paths(&paths);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_path_judgment_judges_the_sources_of_its_paths() {
        let declaration_texts: &[&str] = &["Zonk"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = vec![source_path];
        let path_judgment = PathJudgment::from(paths);

        let judgment = gate.resolve_query(path_judgment);

        let errors = judgment.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = [
            "[LANGUAGE] /workspace/crates/ship/src/lib.rs:1:1: `Zonk` is not made of words of this crate's vocabulary.",
        ];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_term_judgment_judges_the_terms_of_the_contexts_of_its_paths() {
        let declaration_texts: &[&str] = &["Treasure", "Chest"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = vec![source_path];
        let term_judgment = TermJudgment::from(paths);

        let judgment = gate.resolve_query(term_judgment);

        let error = judgment.err();
        let error_texts = error.map(helper::error_texts);
        let expectation = None;
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_token_selection_answers_with_the_token_report_of_its_paths() {
        let source = source_fixture::token_source_factory();
        let source_file = fixture::File::Source { source };
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = vec![source_path];
        let token_selection = TokenSelection::from(paths);

        let token_report = gate.resolve_query(token_selection);

        let token_text = token_report.unwrap().to_string();
        let expectation = r#"/workspace/crates/ship/src/lib.rs	1	4	declaration	Treasure
"#;
        assert_eq!(token_text, expectation);
    }

    #[test]
    fn a_token_selection_of_a_source_that_cannot_be_read_answers_with_its_errors() {
        let source_file = fixture::File::OpenError;
        let gate = fixture::crate_gate_factory(source_file);
        let source_path_text = fixture::SOURCE_PATH;
        let source_path = PathBuf::from(source_path_text);
        let paths = vec![source_path];
        let token_selection = TokenSelection::from(paths);

        let token_report = gate.resolve_query(token_selection);

        let errors = token_report.unwrap_err();
        let helper::ErrorTexts { error_texts } = helper::error_texts(errors);
        let expectation = ["[LANGUAGE] cannot read `/workspace/crates/ship/src/lib.rs`: refused."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn a_vocabulary_whose_contexts_cannot_be_listed_makes_no_gate() {
        let vocabulary_error_adapter = fixture::VocabularyErrorAdapter::Context;
        let reader_adapter = fixture::ReaderAdapter::default();
        let decoder_adapter = fixture::DecoderAdapter::default();
        let parser_adapter = fixture::ParserAdapter::default();

        let gate: Result<fixture::FixtureGate, Error> = Gate::try_from((
            vocabulary_error_adapter,
            reader_adapter,
            decoder_adapter,
            parser_adapter,
        ));

        let error_text = gate.err().map(|error| error.to_string());
        let expectation_text = "[LANGUAGE] there is no Rust source under `contexts/treasure`.";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_vocabulary_whose_foreign_languages_cannot_be_listed_makes_no_gate() {
        let vocabulary_error_adapter = fixture::VocabularyErrorAdapter::ForeignLanguage;
        let reader_adapter = fixture::ReaderAdapter::default();
        let decoder_adapter = fixture::DecoderAdapter::default();
        let parser_adapter = fixture::ParserAdapter::default();

        let gate: Result<fixture::FixtureGate, Error> = Gate::try_from((
            vocabulary_error_adapter,
            reader_adapter,
            decoder_adapter,
            parser_adapter,
        ));

        let error_text = gate.err().map(|error| error.to_string());
        let expectation_text = "[LANGUAGE] there is no Rust source under `foreign_languages/anchor`.";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }
}
