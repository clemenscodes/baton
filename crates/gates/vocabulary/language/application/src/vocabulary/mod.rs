mod context_list;
mod foreign_language_list;

pub use context_list::ContextList;
pub use foreign_language_list::ForeignLanguageList;

use crate::Error;
use stack_domain_driven_design_core::port::Port;

pub trait Vocabulary: Port {
    fn try_list_contexts(&self) -> Result<ContextList, Error>;

    fn try_list_foreign_languages(&self) -> Result<ForeignLanguageList, Error>;
}
