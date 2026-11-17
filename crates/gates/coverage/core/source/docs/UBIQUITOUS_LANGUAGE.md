# The ubiquitous language

| Word | What it means |
|---|---|
| Domain | The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser. |
| Gates Coverage Core Source | The crate gates_coverage_core_source, the crate of the domain of the coverage that holds the source, a path with its text, and the port of the reader. |
| Line | A line of a source, counted from 1: the line of a position, the line of a syntax error, and the opening line and the closing line of a test boundary; and the error of a test boundary whose opening line is less than 1 or comes after its closing line, which names its path, its opening line and its closing line. |
| Open Report | To open the export of llvm-cov at the path the gate is given, which it then decodes instead of its standard input. |
| Path | The path of a file or a directory: of a source or of a file of an export, as llvm-cov names it, of the source a test boundary lies in, of a report, a profile, a profile directory, a tool, a manifest or an executable; and the path of an attribute as syn parses it, cfg for the test attribute. |
| Read Source | To read the text of the file of a path into a source of that path. |
| Reader | The port that reads a source by its path. |
| Source | A file of Rust whose spans a report holds: its path and its text, as the reader reads it and the parser parses it, and the error of the gate for a source that cannot be read, which names its path and the cause. |
| Text | A string as it was read or as it is written: the text of a source, the success text and the text of a path. |
