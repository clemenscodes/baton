# The ubiquitous language

| Word | What it means |
|---|---|
| Compose Profile Data Path | To compose the path of the profile data in the profile directory. |
| Compose Profile Variable | To compose the profile variable that names a profile in the profile directory for every process and binary. |
| Create Profile Directory | To create the profile directory of the process under the profile root, empty, replacing one an earlier process of the same number left. |
| Data | The files and the functions of one binary of an export, whose regions count only in the files it keeps. |
| Gates Coverage Core Profile Directory | The crate gates_coverage_core_profile_directory, the crate of the domain of the coverage that holds the profile directory and the port of the profile store. |
| List Profiles | To list the profiles of a profile directory in their order. |
| Profile | A raw profile that an instrumented test writes, with the counts of the regions it reached. |
| Profile Data | The profiles of a measurement merged into one file. |
| Profile Data Name | The name coverage.profdata of the profile data. |
| Profile Directory | The directory of one measurement into which its tests write their profiles; no other measurement writes there, and the measurement removes it when it ends. |
| Profile Name Pattern | The pattern %p-%m.profraw, which names the profile of every process and every binary apart. |
| Profile Root | The directory beside the report under which every measurement makes its profile directory. |
| Profile Store | The driven port that keeps the profiles of a measurement in a directory of their own and writes its report. |
| Profile Variable | The variable LLVM_PROFILE_FILE, which tells an instrumented program where it writes its profile. |
| Profile Variable Name | The name LLVM_PROFILE_FILE of the profile variable. |
| Remove Profile Directory | To remove a profile directory with its profiles and its profile data. |
| Write Report File | To write the export of a report file at its path, creating its directory. |
