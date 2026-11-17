# The outputs

Every output of the flake is a row of this table, and the gate `outputs` refuses a flake and a
table that do not state the same thing. An output that is added and not recorded here fails, and
so does a row that names an output that is gone. The flake builds for the one system `x86_64-linux`.

| Output | Kind | What it is |
|---|---|---|
| `adapter` | check | a crate that reaches an adapter |
| `baton` | package | the program `baton`, which runs a program inside the lock of a GPU and prints the state of every lock |
| `baton_host` | package | the helper `baton_host` run as root, which administers the locks of the GPUs and answers the requests of `baton` |
| `book` | check | the books of the crates |
| `comment` | check | the comments of Nix, shell and Rust |
| `commit_gate` | package | the program the commit message hook runs, it refuses a message that is not a conventional commit or that carries a trailer |
| `coverage` | package | the program `tool_coverage` of `crates/gates/coverage/command_line` |
| `coverage-gates_coverage_adapter_file_system` | check | the regions of the crate `gates_coverage_adapter_file_system` no test reaches |
| `coverage-gates_coverage_adapter_java_script_object_notation` | check | the regions of the crate `gates_coverage_adapter_java_script_object_notation` no test reaches |
| `coverage-gates_coverage_adapter_process` | check | the regions of the crate `gates_coverage_adapter_process` no test reaches |
| `coverage-gates_coverage_adapter_syntax` | check | the regions of the crate `gates_coverage_adapter_syntax` no test reaches |
| `coverage-gates_coverage_application` | check | the regions of the crate `gates_coverage_application` no test reaches |
| `coverage-gates_coverage_command_line` | check | the regions of the crate `gates_coverage_command_line` no test reaches |
| `coverage-gates_coverage_core_compiler` | check | the regions of the crate `gates_coverage_core_compiler` no test reaches |
| `coverage-gates_coverage_core_gaps` | check | the regions of the crate `gates_coverage_core_gaps` no test reaches |
| `coverage-gates_coverage_core_invocation` | check | the regions of the crate `gates_coverage_core_invocation` no test reaches |
| `coverage-gates_coverage_core_mapping_presence` | check | the regions of the crate `gates_coverage_core_mapping_presence` no test reaches |
| `coverage-gates_coverage_core_profile_directory` | check | the regions of the crate `gates_coverage_core_profile_directory` no test reaches |
| `coverage-gates_coverage_core_profile_tools` | check | the regions of the crate `gates_coverage_core_profile_tools` no test reaches |
| `coverage-gates_coverage_core_region` | check | the regions of the crate `gates_coverage_core_region` no test reaches |
| `coverage-gates_coverage_core_report` | check | the regions of the crate `gates_coverage_core_report` no test reaches |
| `coverage-gates_coverage_core_source` | check | the regions of the crate `gates_coverage_core_source` no test reaches |
| `coverage-gates_coverage_core_span` | check | the regions of the crate `gates_coverage_core_span` no test reaches |
| `coverage-gates_coverage_core_test_boundaries` | check | the regions of the crate `gates_coverage_core_test_boundaries` no test reaches |
| `coverage-gates_coverage_core_test_build` | check | the regions of the crate `gates_coverage_core_test_build` no test reaches |
| `coverage-gates_vocabulary_gate` | check | the regions of the crate `gates_vocabulary_gate` no test reaches |
| `coverage-gates_vocabulary_language_adapter_command_line` | check | the regions of the crate `gates_vocabulary_language_adapter_command_line` no test reaches |
| `coverage-gates_vocabulary_language_adapter_file_system` | check | the regions of the crate `gates_vocabulary_language_adapter_file_system` no test reaches |
| `coverage-gates_vocabulary_language_adapter_syntax` | check | the regions of the crate `gates_vocabulary_language_adapter_syntax` no test reaches |
| `coverage-gates_vocabulary_language_adapter_toms_obvious_minimal_language` | check | the regions of the crate `gates_vocabulary_language_adapter_toms_obvious_minimal_language` no test reaches |
| `coverage-gates_vocabulary_language_application` | check | the regions of the crate `gates_vocabulary_language_application` no test reaches |
| `coverage-gates_vocabulary_language_core_books` | check | the regions of the crate `gates_vocabulary_language_core_books` no test reaches |
| `coverage-gates_vocabulary_language_core_dependencies` | check | the regions of the crate `gates_vocabulary_language_core_dependencies` no test reaches |
| `coverage-gates_vocabulary_language_core_foreign_name` | check | the regions of the crate `gates_vocabulary_language_core_foreign_name` no test reaches |
| `coverage-gates_vocabulary_language_core_language` | check | the regions of the crate `gates_vocabulary_language_core_language` no test reaches |
| `coverage-gates_vocabulary_language_core_languages` | check | the regions of the crate `gates_vocabulary_language_core_languages` no test reaches |
| `coverage-products_baton_presentation_command_line_interface` | check | the regions of the crate `products_baton_presentation_command_line_interface` no test reaches |
| `coverage-products_baton_presentation_host` | check | the regions of the crate `products_baton_presentation_host` no test reaches |
| `coverage-stack_domain_driven_design_core` | check | the regions of the crate `stack_domain_driven_design_core` no test reaches |
| `coverage-stack_lock_adapter_control_group_unified_hierarchy` | check | the regions of the crate `stack_lock_adapter_control_group_unified_hierarchy` no test reaches |
| `coverage-stack_lock_adapter_device_program_berkeley_packet_filter` | check | the regions of the crate `stack_lock_adapter_device_program_berkeley_packet_filter` no test reaches |
| `coverage-stack_lock_adapter_device_tree_system_file_system` | check | the regions of the crate `stack_lock_adapter_device_tree_system_file_system` no test reaches |
| `coverage-stack_lock_adapter_host_helper_elevation_program` | check | the regions of the crate `stack_lock_adapter_host_helper_elevation_program` no test reaches |
| `coverage-stack_lock_adapter_lock_record_file_system` | check | the regions of the crate `stack_lock_adapter_lock_record_file_system` no test reaches |
| `coverage-stack_lock_adapter_pause_standard_library` | check | the regions of the crate `stack_lock_adapter_pause_standard_library` no test reaches |
| `coverage-stack_lock_adapter_program_runner_spawner` | check | the regions of the crate `stack_lock_adapter_program_runner_spawner` no test reaches |
| `coverage-stack_lock_adapter_task_table_process_file_system` | check | the regions of the crate `stack_lock_adapter_task_table_process_file_system` no test reaches |
| `coverage-stack_lock_application` | check | the regions of the crate `stack_lock_application` no test reaches |
| `coverage-stack_lock_core_demand` | check | the regions of the crate `stack_lock_core_demand` no test reaches |
| `coverage-stack_lock_core_device_address` | check | the regions of the crate `stack_lock_core_device_address` no test reaches |
| `coverage-stack_lock_core_device_file` | check | the regions of the crate `stack_lock_core_device_file` no test reaches |
| `coverage-stack_lock_core_graphics_lock` | check | the regions of the crate `stack_lock_core_graphics_lock` no test reaches |
| `coverage-stack_lock_core_graphics_user` | check | the regions of the crate `stack_lock_core_graphics_user` no test reaches |
| `coverage-stack_lock_core_name` | check | the regions of the crate `stack_lock_core_name` no test reaches |
| `coverage-stack_process_adapter_spawner_standard_library` | check | the regions of the crate `stack_process_adapter_spawner_standard_library` no test reaches |
| `coverage-stack_process_adapter_spawner_transcript` | check | the regions of the crate `stack_process_adapter_spawner_transcript` no test reaches |
| `coverage-stack_process_core_invocation` | check | the regions of the crate `stack_process_core_invocation` no test reaches |
| `crate-name` | check | the name of a crate and of its project against its path |
| `deadnix` | check | the bindings of Nix nothing reads |
| `default` | shell | the one shell with the toolchain, the patched rustfmt, moon, nextest, cargo-llvm-cov, the Nix formatter, statix and deadnix |
| `dependencies` | check | the `dependsOn` of a `moon.yml` against its `Cargo.toml` |
| `format` | check | the format of Nix and of the shaders |
| `formatter` | formatter | the Nix tree formatter, run by the `format` gate and by `nix fmt` |
| `gates_coverage_adapter_file_system` | check | the crate `gates_coverage_adapter_file_system`, built and tested |
| `gates_coverage_adapter_java_script_object_notation` | check | the crate `gates_coverage_adapter_java_script_object_notation`, built and tested |
| `gates_coverage_adapter_process` | check | the crate `gates_coverage_adapter_process`, built and tested |
| `gates_coverage_adapter_syntax` | check | the crate `gates_coverage_adapter_syntax`, built and tested |
| `gates_coverage_application` | check | the crate `gates_coverage_application`, built and tested |
| `gates_coverage_command_line` | check | the crate `gates_coverage_command_line`, built and tested |
| `gates_coverage_core_compiler` | check | the crate `gates_coverage_core_compiler`, built and tested |
| `gates_coverage_core_gaps` | check | the crate `gates_coverage_core_gaps`, built and tested |
| `gates_coverage_core_invocation` | check | the crate `gates_coverage_core_invocation`, built and tested |
| `gates_coverage_core_mapping_presence` | check | the crate `gates_coverage_core_mapping_presence`, built and tested |
| `gates_coverage_core_profile_directory` | check | the crate `gates_coverage_core_profile_directory`, built and tested |
| `gates_coverage_core_profile_tools` | check | the crate `gates_coverage_core_profile_tools`, built and tested |
| `gates_coverage_core_region` | check | the crate `gates_coverage_core_region`, built and tested |
| `gates_coverage_core_report` | check | the crate `gates_coverage_core_report`, built and tested |
| `gates_coverage_core_source` | check | the crate `gates_coverage_core_source`, built and tested |
| `gates_coverage_core_span` | check | the crate `gates_coverage_core_span`, built and tested |
| `gates_coverage_core_test_boundaries` | check | the crate `gates_coverage_core_test_boundaries`, built and tested |
| `gates_coverage_core_test_build` | check | the crate `gates_coverage_core_test_build`, built and tested |
| `gates_vocabulary_gate` | check | the crate `gates_vocabulary_gate`, built and tested |
| `gates_vocabulary_language_adapter_command_line` | check | the crate `gates_vocabulary_language_adapter_command_line`, built and tested |
| `gates_vocabulary_language_adapter_file_system` | check | the crate `gates_vocabulary_language_adapter_file_system`, built and tested |
| `gates_vocabulary_language_adapter_syntax` | check | the crate `gates_vocabulary_language_adapter_syntax`, built and tested |
| `gates_vocabulary_language_adapter_toms_obvious_minimal_language` | check | the crate `gates_vocabulary_language_adapter_toms_obvious_minimal_language`, built and tested |
| `gates_vocabulary_language_application` | check | the crate `gates_vocabulary_language_application`, built and tested |
| `gates_vocabulary_language_core_books` | check | the crate `gates_vocabulary_language_core_books`, built and tested |
| `gates_vocabulary_language_core_dependencies` | check | the crate `gates_vocabulary_language_core_dependencies`, built and tested |
| `gates_vocabulary_language_core_foreign_name` | check | the crate `gates_vocabulary_language_core_foreign_name`, built and tested |
| `gates_vocabulary_language_core_language` | check | the crate `gates_vocabulary_language_core_language`, built and tested |
| `gates_vocabulary_language_core_languages` | check | the crate `gates_vocabulary_language_core_languages`, built and tested |
| `interface` | check | the imports and the publications of a layer |
| `layer` | check | the layer of a type and the direction of a dependency |
| `outputs` | check | this page against the flake |
| `participle` | check | the terms whose head is a participle |
| `pipeline` | check | the checks of the flake against the tasks of moon |
| `products_baton_presentation_command_line_interface` | check | the crate `products_baton_presentation_command_line_interface`, built and tested |
| `products_baton_presentation_host` | check | the crate `products_baton_presentation_host`, built and tested |
| `rustfmt` | check | the format of Rust |
| `shell` | check | the shell scripts, through shellcheck |
| `sibling` | check | the siblings whose names begin with the same word |
| `signature` | check | the signatures, the casts and the numbers of the production code |
| `stack_domain_driven_design_core` | check | the crate `stack_domain_driven_design_core`, built and tested |
| `stack_lock_adapter_control_group_unified_hierarchy` | check | the crate `stack_lock_adapter_control_group_unified_hierarchy`, built and tested |
| `stack_lock_adapter_device_program_berkeley_packet_filter` | check | the crate `stack_lock_adapter_device_program_berkeley_packet_filter`, built and tested |
| `stack_lock_adapter_device_tree_system_file_system` | check | the crate `stack_lock_adapter_device_tree_system_file_system`, built and tested |
| `stack_lock_adapter_host_helper_elevation_program` | check | the crate `stack_lock_adapter_host_helper_elevation_program`, built and tested |
| `stack_lock_adapter_lock_record_file_system` | check | the crate `stack_lock_adapter_lock_record_file_system`, built and tested |
| `stack_lock_adapter_pause_standard_library` | check | the crate `stack_lock_adapter_pause_standard_library`, built and tested |
| `stack_lock_adapter_program_runner_spawner` | check | the crate `stack_lock_adapter_program_runner_spawner`, built and tested |
| `stack_lock_adapter_task_table_process_file_system` | check | the crate `stack_lock_adapter_task_table_process_file_system`, built and tested |
| `stack_lock_application` | check | the crate `stack_lock_application`, built and tested |
| `stack_lock_core_demand` | check | the crate `stack_lock_core_demand`, built and tested |
| `stack_lock_core_device_address` | check | the crate `stack_lock_core_device_address`, built and tested |
| `stack_lock_core_device_file` | check | the crate `stack_lock_core_device_file`, built and tested |
| `stack_lock_core_graphics_lock` | check | the crate `stack_lock_core_graphics_lock`, built and tested |
| `stack_lock_core_graphics_user` | check | the crate `stack_lock_core_graphics_user`, built and tested |
| `stack_lock_core_name` | check | the crate `stack_lock_core_name`, built and tested |
| `stack_process_adapter_spawner_standard_library` | check | the crate `stack_process_adapter_spawner_standard_library`, built and tested |
| `stack_process_adapter_spawner_transcript` | check | the crate `stack_process_adapter_spawner_transcript`, built and tested |
| `stack_process_core_invocation` | check | the crate `stack_process_core_invocation`, built and tested |
| `statix` | check | the lints of statix over Nix |
| `test` | check | a test file and a decision inside a main |
| `ubiquitous-language` | check | the books of the contexts against their vocabulary |
| `unsafe-code` | check | `unsafe` outside the one module that may use it |
| `unused-term` | check | the terms no production code spells |
| `verb` | check | the functions named for a thing that are no getters |
| `vocabulary` | package | the program of `crates/gates/vocabulary/gate` |
| `vocabulary-gates_coverage_adapter_file_system` | check | the identifiers of the crate `gates_coverage_adapter_file_system` against its languages |
| `vocabulary-gates_coverage_adapter_java_script_object_notation` | check | the identifiers of the crate `gates_coverage_adapter_java_script_object_notation` against its languages |
| `vocabulary-gates_coverage_adapter_process` | check | the identifiers of the crate `gates_coverage_adapter_process` against its languages |
| `vocabulary-gates_coverage_adapter_syntax` | check | the identifiers of the crate `gates_coverage_adapter_syntax` against its languages |
| `vocabulary-gates_coverage_application` | check | the identifiers of the crate `gates_coverage_application` against its languages |
| `vocabulary-gates_coverage_command_line` | check | the identifiers of the crate `gates_coverage_command_line` against its languages |
| `vocabulary-gates_coverage_core_compiler` | check | the identifiers of the crate `gates_coverage_core_compiler` against its languages |
| `vocabulary-gates_coverage_core_gaps` | check | the identifiers of the crate `gates_coverage_core_gaps` against its languages |
| `vocabulary-gates_coverage_core_invocation` | check | the identifiers of the crate `gates_coverage_core_invocation` against its languages |
| `vocabulary-gates_coverage_core_mapping_presence` | check | the identifiers of the crate `gates_coverage_core_mapping_presence` against its languages |
| `vocabulary-gates_coverage_core_profile_directory` | check | the identifiers of the crate `gates_coverage_core_profile_directory` against its languages |
| `vocabulary-gates_coverage_core_profile_tools` | check | the identifiers of the crate `gates_coverage_core_profile_tools` against its languages |
| `vocabulary-gates_coverage_core_region` | check | the identifiers of the crate `gates_coverage_core_region` against its languages |
| `vocabulary-gates_coverage_core_report` | check | the identifiers of the crate `gates_coverage_core_report` against its languages |
| `vocabulary-gates_coverage_core_source` | check | the identifiers of the crate `gates_coverage_core_source` against its languages |
| `vocabulary-gates_coverage_core_span` | check | the identifiers of the crate `gates_coverage_core_span` against its languages |
| `vocabulary-gates_coverage_core_test_boundaries` | check | the identifiers of the crate `gates_coverage_core_test_boundaries` against its languages |
| `vocabulary-gates_coverage_core_test_build` | check | the identifiers of the crate `gates_coverage_core_test_build` against its languages |
| `vocabulary-gates_vocabulary_gate` | check | the identifiers of the crate `gates_vocabulary_gate` against its languages |
| `vocabulary-gates_vocabulary_language_adapter_command_line` | check | the identifiers of the crate `gates_vocabulary_language_adapter_command_line` against its languages |
| `vocabulary-gates_vocabulary_language_adapter_file_system` | check | the identifiers of the crate `gates_vocabulary_language_adapter_file_system` against its languages |
| `vocabulary-gates_vocabulary_language_adapter_syntax` | check | the identifiers of the crate `gates_vocabulary_language_adapter_syntax` against its languages |
| `vocabulary-gates_vocabulary_language_adapter_toms_obvious_minimal_language` | check | the identifiers of the crate `gates_vocabulary_language_adapter_toms_obvious_minimal_language` against its languages |
| `vocabulary-gates_vocabulary_language_application` | check | the identifiers of the crate `gates_vocabulary_language_application` against its languages |
| `vocabulary-gates_vocabulary_language_core_books` | check | the identifiers of the crate `gates_vocabulary_language_core_books` against its languages |
| `vocabulary-gates_vocabulary_language_core_dependencies` | check | the identifiers of the crate `gates_vocabulary_language_core_dependencies` against its languages |
| `vocabulary-gates_vocabulary_language_core_foreign_name` | check | the identifiers of the crate `gates_vocabulary_language_core_foreign_name` against its languages |
| `vocabulary-gates_vocabulary_language_core_language` | check | the identifiers of the crate `gates_vocabulary_language_core_language` against its languages |
| `vocabulary-gates_vocabulary_language_core_languages` | check | the identifiers of the crate `gates_vocabulary_language_core_languages` against its languages |
| `vocabulary-products_baton_presentation_command_line_interface` | check | the identifiers of the crate `products_baton_presentation_command_line_interface` against its languages |
| `vocabulary-products_baton_presentation_host` | check | the identifiers of the crate `products_baton_presentation_host` against its languages |
| `vocabulary-stack_domain_driven_design_core` | check | the identifiers of the crate `stack_domain_driven_design_core` against its languages |
| `vocabulary-stack_lock_adapter_control_group_unified_hierarchy` | check | the identifiers of the crate `stack_lock_adapter_control_group_unified_hierarchy` against its languages |
| `vocabulary-stack_lock_adapter_device_program_berkeley_packet_filter` | check | the identifiers of the crate `stack_lock_adapter_device_program_berkeley_packet_filter` against its languages |
| `vocabulary-stack_lock_adapter_device_tree_system_file_system` | check | the identifiers of the crate `stack_lock_adapter_device_tree_system_file_system` against its languages |
| `vocabulary-stack_lock_adapter_host_helper_elevation_program` | check | the identifiers of the crate `stack_lock_adapter_host_helper_elevation_program` against its languages |
| `vocabulary-stack_lock_adapter_lock_record_file_system` | check | the identifiers of the crate `stack_lock_adapter_lock_record_file_system` against its languages |
| `vocabulary-stack_lock_adapter_pause_standard_library` | check | the identifiers of the crate `stack_lock_adapter_pause_standard_library` against its languages |
| `vocabulary-stack_lock_adapter_program_runner_spawner` | check | the identifiers of the crate `stack_lock_adapter_program_runner_spawner` against its languages |
| `vocabulary-stack_lock_adapter_task_table_process_file_system` | check | the identifiers of the crate `stack_lock_adapter_task_table_process_file_system` against its languages |
| `vocabulary-stack_lock_application` | check | the identifiers of the crate `stack_lock_application` against its languages |
| `vocabulary-stack_lock_core_demand` | check | the identifiers of the crate `stack_lock_core_demand` against its languages |
| `vocabulary-stack_lock_core_device_address` | check | the identifiers of the crate `stack_lock_core_device_address` against its languages |
| `vocabulary-stack_lock_core_device_file` | check | the identifiers of the crate `stack_lock_core_device_file` against its languages |
| `vocabulary-stack_lock_core_graphics_lock` | check | the identifiers of the crate `stack_lock_core_graphics_lock` against its languages |
| `vocabulary-stack_lock_core_graphics_user` | check | the identifiers of the crate `stack_lock_core_graphics_user` against its languages |
| `vocabulary-stack_lock_core_name` | check | the identifiers of the crate `stack_lock_core_name` against its languages |
| `vocabulary-stack_process_adapter_spawner_standard_library` | check | the identifiers of the crate `stack_process_adapter_spawner_standard_library` against its languages |
| `vocabulary-stack_process_adapter_spawner_transcript` | check | the identifiers of the crate `stack_process_adapter_spawner_transcript` against its languages |
| `vocabulary-stack_process_core_invocation` | check | the identifiers of the crate `stack_process_core_invocation` against its languages |
| `word` | check | the words `docs/STYLE.md` refuses in a name |
