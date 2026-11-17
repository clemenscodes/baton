mod gates;
mod products;
mod stack;

use crate::domain::contexts::gates::coverage::adapter::file_system::GatesCoverageAdapterFileSystemContext;
use crate::domain::contexts::gates::coverage::adapter::java_script_object_notation::GatesCoverageAdapterJavaScriptObjectNotationContext;
use crate::domain::contexts::gates::coverage::adapter::process::GatesCoverageAdapterProcessContext;
use crate::domain::contexts::gates::coverage::adapter::syntax::GatesCoverageAdapterSyntaxContext;
use crate::domain::contexts::gates::coverage::application::GatesCoverageApplicationContext;
use crate::domain::contexts::gates::coverage::command_line::GatesCoverageCommandLineContext;
use crate::domain::contexts::gates::coverage::core::compiler::GatesCoverageCoreCompilerContext;
use crate::domain::contexts::gates::coverage::core::gaps::GatesCoverageCoreGapsContext;
use crate::domain::contexts::gates::coverage::core::invocation::GatesCoverageCoreInvocationContext;
use crate::domain::contexts::gates::coverage::core::mapping_presence::GatesCoverageCoreMappingPresenceContext;
use crate::domain::contexts::gates::coverage::core::profile::directory::GatesCoverageCoreProfileDirectoryContext;
use crate::domain::contexts::gates::coverage::core::profile::tools::GatesCoverageCoreProfileToolsContext;
use crate::domain::contexts::gates::coverage::core::region::GatesCoverageCoreRegionContext;
use crate::domain::contexts::gates::coverage::core::report::GatesCoverageCoreReportContext;
use crate::domain::contexts::gates::coverage::core::source::GatesCoverageCoreSourceContext;
use crate::domain::contexts::gates::coverage::core::span::GatesCoverageCoreSpanContext;
use crate::domain::contexts::gates::coverage::core::test::boundaries::GatesCoverageCoreTestBoundariesContext;
use crate::domain::contexts::gates::coverage::core::test::build::GatesCoverageCoreTestBuildContext;
use crate::domain::contexts::gates::vocabulary::gate::GatesVocabularyGateContext;
use crate::domain::contexts::gates::vocabulary::language::adapter::command_line::GatesVocabularyLanguageAdapterCommandLineContext;
use crate::domain::contexts::gates::vocabulary::language::adapter::file_system::GatesVocabularyLanguageAdapterFileSystemContext;
use crate::domain::contexts::gates::vocabulary::language::adapter::syntax::GatesVocabularyLanguageAdapterSyntaxContext;
use crate::domain::contexts::gates::vocabulary::language::adapter::toms_obvious_minimal_language::GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext;
use crate::domain::contexts::gates::vocabulary::language::application::GatesVocabularyLanguageApplicationContext;
use crate::domain::contexts::gates::vocabulary::language::core::books::GatesVocabularyLanguageCoreBooksContext;
use crate::domain::contexts::gates::vocabulary::language::core::dependencies::GatesVocabularyLanguageCoreDependenciesContext;
use crate::domain::contexts::gates::vocabulary::language::core::foreign_name::GatesVocabularyLanguageCoreForeignNameContext;
use crate::domain::contexts::gates::vocabulary::language::core::language::GatesVocabularyLanguageCoreLanguageContext;
use crate::domain::contexts::gates::vocabulary::language::core::languages::GatesVocabularyLanguageCoreLanguagesContext;
use crate::domain::contexts::products::baton::presentation::command_line_interface::ProductsBatonPresentationCommandLineInterfaceContext;
use crate::domain::contexts::products::baton::presentation::host::ProductsBatonPresentationHostContext;
use crate::domain::contexts::stack::domain_driven_design::core::StackDomainDrivenDesignCoreContext;
use crate::domain::contexts::stack::lock::adapter::control_group::unified_hierarchy::StackLockAdapterControlGroupUnifiedHierarchyContext;
use crate::domain::contexts::stack::lock::adapter::device::program::berkeley_packet_filter::StackLockAdapterDeviceProgramBerkeleyPacketFilterContext;
use crate::domain::contexts::stack::lock::adapter::device::tree::system_file_system::StackLockAdapterDeviceTreeSystemFileSystemContext;
use crate::domain::contexts::stack::lock::adapter::host_helper::elevation_program::StackLockAdapterHostHelperElevationProgramContext;
use crate::domain::contexts::stack::lock::adapter::lock_record::file_system::StackLockAdapterLockRecordFileSystemContext;
use crate::domain::contexts::stack::lock::adapter::pause::standard_library::StackLockAdapterPauseStandardLibraryContext;
use crate::domain::contexts::stack::lock::adapter::program_runner::spawner::StackLockAdapterProgramRunnerSpawnerContext;
use crate::domain::contexts::stack::lock::adapter::task_table::process_file_system::StackLockAdapterTaskTableProcessFileSystemContext;
use crate::domain::contexts::stack::lock::application::StackLockApplicationContext;
use crate::domain::contexts::stack::lock::core::demand::StackLockCoreDemandContext;
use crate::domain::contexts::stack::lock::core::device::address::StackLockCoreDeviceAddressContext;
use crate::domain::contexts::stack::lock::core::device::file::StackLockCoreDeviceFileContext;
use crate::domain::contexts::stack::lock::core::graphics::lock::StackLockCoreGraphicsLockContext;
use crate::domain::contexts::stack::lock::core::graphics::user::StackLockCoreGraphicsUserContext;
use crate::domain::contexts::stack::lock::core::name::StackLockCoreNameContext;
use crate::domain::contexts::stack::process::adapter::spawner::standard_library::StackProcessAdapterSpawnerStandardLibraryContext;
use crate::domain::contexts::stack::process::adapter::spawner::transcript::StackProcessAdapterSpawnerTranscriptContext;
use crate::domain::contexts::stack::process::core::invocation::StackProcessCoreInvocationContext;
use crate::domain::foreign_languages::ForeignLanguages;
use ::gates_vocabulary_language_application::Error as VocabularyError;
use ::gates_vocabulary_language_application::vocabulary::{
    ContextList, ForeignLanguageList, Vocabulary,
};
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use ::stack_domain_driven_design_core::port::Port;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct Contexts;

impl Layer for Contexts {
    type Layer = DomainLayer;
}

impl Contexts {
    fn compose_language_context_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<Context, Error>> {
        let gates_vocabulary_language_core_foreign_name = GatesVocabularyLanguageCoreForeignNameContext;
        let gates_vocabulary_language_core_language = GatesVocabularyLanguageCoreLanguageContext;
        let gates_vocabulary_language_core_dependencies = GatesVocabularyLanguageCoreDependenciesContext;
        let gates_vocabulary_language_core_languages = GatesVocabularyLanguageCoreLanguagesContext;
        let gates_vocabulary_language_core_books = GatesVocabularyLanguageCoreBooksContext;
        let gates_vocabulary_language_application = GatesVocabularyLanguageApplicationContext;
        let gates_vocabulary_language_adapter_file_system = GatesVocabularyLanguageAdapterFileSystemContext;
        let gates_vocabulary_language_adapter_syntax = GatesVocabularyLanguageAdapterSyntaxContext;
        let gates_vocabulary_language_adapter_toms_obvious_minimal_language = GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext;
        let gates_vocabulary_language_adapter_command_line = GatesVocabularyLanguageAdapterCommandLineContext;
        vec![
            Context::try_from(gates_vocabulary_language_core_foreign_name),
            Context::try_from(gates_vocabulary_language_core_language),
            Context::try_from(gates_vocabulary_language_core_dependencies),
            Context::try_from(gates_vocabulary_language_core_languages),
            Context::try_from(gates_vocabulary_language_core_books),
            Context::try_from(gates_vocabulary_language_application),
            Context::try_from(gates_vocabulary_language_adapter_file_system),
            Context::try_from(gates_vocabulary_language_adapter_syntax),
            Context::try_from(gates_vocabulary_language_adapter_toms_obvious_minimal_language),
            Context::try_from(gates_vocabulary_language_adapter_command_line),
        ]
    }

    fn compose_coverage_context_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<Context, Error>> {
        let gates_coverage_application = GatesCoverageApplicationContext;
        let gates_coverage_command_line = GatesCoverageCommandLineContext;
        let coverage_adapter_context_outcomes = self.compose_coverage_adapter_context_outcomes();
        let coverage_core_context_outcomes = self.compose_coverage_core_context_outcomes();
        let coverage_report_context_outcomes = self.compose_coverage_report_context_outcomes();
        vec![
            Context::try_from(gates_coverage_application),
            Context::try_from(gates_coverage_command_line),
        ]
        .into_iter()
        .chain(coverage_adapter_context_outcomes)
        .chain(coverage_core_context_outcomes)
        .chain(coverage_report_context_outcomes)
    }

    fn compose_coverage_adapter_context_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<Context, Error>> {
        let gates_coverage_adapter_file_system = GatesCoverageAdapterFileSystemContext;
        let gates_coverage_adapter_java_script_object_notation = GatesCoverageAdapterJavaScriptObjectNotationContext;
        let gates_coverage_adapter_process = GatesCoverageAdapterProcessContext;
        let gates_coverage_adapter_syntax = GatesCoverageAdapterSyntaxContext;
        vec![
            Context::try_from(gates_coverage_adapter_file_system),
            Context::try_from(gates_coverage_adapter_java_script_object_notation),
            Context::try_from(gates_coverage_adapter_process),
            Context::try_from(gates_coverage_adapter_syntax),
        ]
    }

    fn compose_coverage_core_context_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<Context, Error>> {
        let gates_coverage_core_compiler = GatesCoverageCoreCompilerContext;
        let gates_coverage_core_gaps = GatesCoverageCoreGapsContext;
        let gates_coverage_core_invocation = GatesCoverageCoreInvocationContext;
        let gates_coverage_core_mapping_presence = GatesCoverageCoreMappingPresenceContext;
        let gates_coverage_core_profile_directory = GatesCoverageCoreProfileDirectoryContext;
        let gates_coverage_core_profile_tools = GatesCoverageCoreProfileToolsContext;
        vec![
            Context::try_from(gates_coverage_core_compiler),
            Context::try_from(gates_coverage_core_gaps),
            Context::try_from(gates_coverage_core_invocation),
            Context::try_from(gates_coverage_core_mapping_presence),
            Context::try_from(gates_coverage_core_profile_directory),
            Context::try_from(gates_coverage_core_profile_tools),
        ]
    }

    fn compose_coverage_report_context_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<Context, Error>> {
        let gates_coverage_core_region = GatesCoverageCoreRegionContext;
        let gates_coverage_core_report = GatesCoverageCoreReportContext;
        let gates_coverage_core_source = GatesCoverageCoreSourceContext;
        let gates_coverage_core_span = GatesCoverageCoreSpanContext;
        let gates_coverage_core_test_boundaries = GatesCoverageCoreTestBoundariesContext;
        let gates_coverage_core_test_build = GatesCoverageCoreTestBuildContext;
        vec![
            Context::try_from(gates_coverage_core_region),
            Context::try_from(gates_coverage_core_report),
            Context::try_from(gates_coverage_core_source),
            Context::try_from(gates_coverage_core_span),
            Context::try_from(gates_coverage_core_test_boundaries),
            Context::try_from(gates_coverage_core_test_build),
        ]
    }

    fn compose_context_outcomes(&self) -> impl IntoIterator<Item = Result<Context, Error>> {
        let stack_domain_driven_design_core = StackDomainDrivenDesignCoreContext;
        let products_baton_presentation_host = ProductsBatonPresentationHostContext;
        let products_baton_presentation_command_line_interface = ProductsBatonPresentationCommandLineInterfaceContext;
        let stack_lock_adapter_device_program_berkeley_packet_filter = StackLockAdapterDeviceProgramBerkeleyPacketFilterContext;
        let stack_lock_adapter_device_tree_system_file_system = StackLockAdapterDeviceTreeSystemFileSystemContext;
        let stack_lock_adapter_task_table_process_file_system = StackLockAdapterTaskTableProcessFileSystemContext;
        let stack_lock_adapter_control_group_unified_hierarchy = StackLockAdapterControlGroupUnifiedHierarchyContext;
        let stack_lock_adapter_lock_record_file_system = StackLockAdapterLockRecordFileSystemContext;
        let stack_lock_adapter_host_helper_elevation_program = StackLockAdapterHostHelperElevationProgramContext;
        let stack_lock_adapter_program_runner_spawner = StackLockAdapterProgramRunnerSpawnerContext;
        let stack_lock_adapter_pause_standard_library = StackLockAdapterPauseStandardLibraryContext;
        let stack_lock_application = StackLockApplicationContext;
        let stack_lock_core_graphics_lock = StackLockCoreGraphicsLockContext;
        let stack_lock_core_demand = StackLockCoreDemandContext;
        let stack_lock_core_graphics_user = StackLockCoreGraphicsUserContext;
        let stack_lock_core_device_file = StackLockCoreDeviceFileContext;
        let stack_lock_core_device_address = StackLockCoreDeviceAddressContext;
        let stack_lock_core_name = StackLockCoreNameContext;
        let stack_process_adapter_spawner_standard_library = StackProcessAdapterSpawnerStandardLibraryContext;
        let stack_process_adapter_spawner_transcript = StackProcessAdapterSpawnerTranscriptContext;
        let stack_process_core_invocation = StackProcessCoreInvocationContext;
        let gates_vocabulary_gate = GatesVocabularyGateContext;
        let language_context_outcomes = self.compose_language_context_outcomes();
        let coverage_context_outcomes = self.compose_coverage_context_outcomes();
        vec![
            Context::try_from(stack_domain_driven_design_core),
            Context::try_from(products_baton_presentation_host),
            Context::try_from(products_baton_presentation_command_line_interface),
            Context::try_from(stack_lock_adapter_device_program_berkeley_packet_filter),
            Context::try_from(stack_lock_adapter_device_tree_system_file_system),
            Context::try_from(stack_lock_adapter_task_table_process_file_system),
            Context::try_from(stack_lock_adapter_control_group_unified_hierarchy),
            Context::try_from(stack_lock_adapter_lock_record_file_system),
            Context::try_from(stack_lock_adapter_host_helper_elevation_program),
            Context::try_from(stack_lock_adapter_program_runner_spawner),
            Context::try_from(stack_lock_adapter_pause_standard_library),
            Context::try_from(stack_lock_application),
            Context::try_from(stack_lock_core_graphics_lock),
            Context::try_from(stack_lock_core_demand),
            Context::try_from(stack_lock_core_graphics_user),
            Context::try_from(stack_lock_core_device_file),
            Context::try_from(stack_lock_core_device_address),
            Context::try_from(stack_lock_core_name),
            Context::try_from(stack_process_adapter_spawner_standard_library),
            Context::try_from(stack_process_adapter_spawner_transcript),
            Context::try_from(stack_process_core_invocation),
            Context::try_from(gates_vocabulary_gate),
        ]
        .into_iter()
        .chain(language_context_outcomes)
        .chain(coverage_context_outcomes)
    }
}

impl Port for Contexts {}

impl Vocabulary for Contexts {
    fn try_list_contexts(&self) -> Result<ContextList, VocabularyError> {
        let context_outcomes = self.compose_context_outcomes();
        let context_list: Result<ContextList, Error> = context_outcomes.into_iter().collect();
        context_list.map_err(VocabularyError::from)
    }

    fn try_list_foreign_languages(&self) -> Result<ForeignLanguageList, VocabularyError> {
        let foreign_language_list = |foreign_languages: ForeignLanguages| {
            foreign_languages.into_iter().collect()
        };
        let foreign_languages = ForeignLanguages::try_from(());
        foreign_languages
            .map(foreign_language_list)
            .map_err(VocabularyError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_contexts_and_the_foreign_languages_of_the_vocabulary_come_about() {
        let contexts = Contexts;

        let context_list = contexts.try_list_contexts();
        let foreign_language_list = contexts.try_list_foreign_languages();

        let context_error = context_list.err().map(|error| error.to_string());
        let foreign_language_error = foreign_language_list.err().map(|error| error.to_string());
        let expectation = None;
        assert_eq!(context_error, expectation);
        assert_eq!(foreign_language_error, expectation);
    }
}
