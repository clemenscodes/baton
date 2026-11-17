use crate::Error;
use crate::gate::crate_examination::CrateExamination;
use gates_vocabulary_language_core_language::UnknownVocabulary;
use gates_vocabulary_language_core_language::source::Source;
use gates_vocabulary_language_core_language::vocabulary::Spellings;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::specification::CandidateDecision;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct Examination {
    path: PathBuf,
    source: Source,
    crate_examination: Rc<CrateExamination>,
}

impl Layer for Examination {
    type Layer = ApplicationLayer;
}

impl Examination {
    pub(super) fn select_production_spellings(&self) -> Spellings {
        let Self {
            source,
            crate_examination,
            ..
        } = self;
        let package_identifier = crate_examination.package_identifier();
        let production_language = crate_examination.production_language();
        let production_vocabulary = production_language.vocabulary();
        let spellings = production_vocabulary.spellings();
        let production_code = source.production_code();
        let code_spellings = Spellings::from((spellings, production_code));
        let package_identifier_spellings = Spellings::from((spellings, package_identifier));
        &code_spellings | &package_identifier_spellings
    }

    fn select_unknown_vocabulary(&self) -> UnknownVocabulary {
        let Self {
            source,
            crate_examination,
            ..
        } = self;
        let production_language = crate_examination.production_language();
        let test_language = crate_examination.test_language();
        let production_vocabulary = production_language.vocabulary();
        let test_vocabulary = test_language.vocabulary();
        let production_code = source.production_code();
        let test_code = source.test_code();
        let production_unknown_vocabulary = production_vocabulary
            .select_unknown_vocabulary(production_code);
        let test_unknown_vocabulary = test_vocabulary.select_unknown_test_vocabulary(test_code);
        &production_unknown_vocabulary | &test_unknown_vocabulary
    }

    pub(super) fn try_judge_source(&self) -> Result<(), Error> {
        let Self { path, source, .. } = self;
        let unknown_vocabulary = self.select_unknown_vocabulary();
        let source_defects = source.defects();
        let vocabulary_decision = unknown_vocabulary.decide_acceptance();
        let defect_decision = source_defects.decide_acceptance();
        let decision = vocabulary_decision & defect_decision;
        match decision {
            CandidateDecision::Acceptance => Ok(()),
            CandidateDecision::Rejection => {
                let source_path = path.clone();
                let defects = source_defects.clone();
                let error = Error::Language {
                    path: source_path,
                    unknown_vocabulary,
                    defects,
                };
                Err(error)
            }
        }
    }
}

impl From<(PathBuf, Source, Rc<CrateExamination>)> for Examination {
    fn from((path, source, crate_examination): (PathBuf, Source, Rc<CrateExamination>)) -> Self {
        Self {
            path,
            source,
            crate_examination,
        }
    }
}
