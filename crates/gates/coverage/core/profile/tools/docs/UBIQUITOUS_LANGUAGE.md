# The ubiquitous language

| Word | What it means |
|---|---|
| Compose Export Invocation | To compose the invocation that exports the coverage of the executables of a test build for the sources of its package. |
| Compose Inspection Invocation | To compose the invocation that lists the section headers of every executable of a test build. |
| Compose Merge Invocation | To compose the invocation that merges the profiles of a measurement into its profile data. |
| Compose Tool Path | To compose the path of a profile tool in the tool directory, with the suffix of executables of the host. |
| Data | The files and the functions of one binary of an export, whose regions count only in the files it keeps. |
| Empty Export | The export without data, the report of tests whose executables carry no mapping section and so no region. |
| Export | The text that llvm-cov export writes in the Java Script Object Notation, holding the data of every binary of the tests, which a measurement writes into its report file, and the error of the adapter Java Script Object Notation for a text that is no export, which carries the error of serde_json. |
| Export Tool Name | The name llvm-cov of the tool that exports coverage. |
| File | A file of an export, named by its filename, whose regions the data keeps, and the error of a code region whose file index its function lacks, which names that file index and the file count of that function; and the file of syn, the items of a source as syn parses them, whose modules the test module visitor visits. |
| Gates Coverage Core Profile Tools | The crate gates_coverage_core_profile_tools, the crate of the domain of the coverage that holds the profile tools, which compose the invocations of the tools of LLVM beside rustc. |
| Index | The position of a value in a list, counted from zero, such as the index of the separator word or of an executable. |
| Inspection | The listing of the section headers of the executables of a test build, which shows whether they carry a coverage mapping. |
| Inspection Tool Name | The name llvm-objdump of the tool that lists the section headers of an executable. |
| Library | The library of the compiler for its host, whose directory the compiler names on request. |
| Library Directory | The directory of the library of the compiler for its host. |
| Library Directory Absence | An output of the compiler that names no library directory. |
| Library Directory Encoding | An output of the compiler that is no text of UTF-8. |
| Merge | The merge of the profiles of a measurement into its profile data. |
| Merge Tool Name | The name llvm-profdata of the tool that merges profiles. |
| Object Flag | The flag -object, which names every executable of a test after the first to the tool that exports coverage. |
| Paths | The path of every source a report keeps a span of, each path once and in order. |
| Profile | A raw profile that an instrumented test writes, with the counts of the regions it reached. |
| Profile Data | The profiles of a measurement merged into one file. |
| Profile Data Flag | The flag -o, in front of the path of the profile data that the merge writes. |
| Profile Data Path | The path of the profile data in the profile directory. |
| Profile Tool | A tool of LLVM beside the library of the compiler that merges the profiles of a measurement or exports their coverage; the profile tools share the path of the profile data. |
| Section Header Flag | The flag --section-headers, which asks the inspection tool for the section headers of every executable it names. |
| Text | A string as it was read or as it is written: the text of a source, the success text and the text of a path. |
| Tool | A program of LLVM beside the library of the compiler, the one that merges profiles or the one that exports coverage. |
| Tool Directory | The directory of the profile tools, beside the library directory. |
| Tool Directory Name | The name bin of the tool directory. |
| Tool File Name | The name of the file of a tool: its name and the suffix of the executables of the host. |
