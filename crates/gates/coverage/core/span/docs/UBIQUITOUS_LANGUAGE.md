# The ubiquitous language

| Word | What it means |
|---|---|
| Column | The column of a position and the column of a syntax error, counted from 1. |
| Domain | The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser. |
| End | The position a span ends at. |
| First Column | The column 1, below which a position refuses its column. |
| First Line | The line 1, below which a position refuses its line and a test boundary its opening line. |
| Gates Coverage Core Span | The crate gates_coverage_core_span, the crate of the domain of the coverage that holds the span, a path with the position where it starts and the position where it ends. |
| Line | A line of a source, counted from 1: the line of a position, the line of a syntax error, and the opening line and the closing line of a test boundary; and the error of a test boundary whose opening line is less than 1 or comes after its closing line, which names its path, its opening line and its closing line. |
| Order | The error of a span whose end lies before its start, which names the path, the start and the end of that span. |
| Path | The path of a file or a directory: of a source or of a file of an export, as llvm-cov names it, of the source a test boundary lies in, of a report, a profile, a profile directory, a tool, a manifest or an executable; and the path of an attribute as syn parses it, cfg for the test attribute. |
| Position | A line and a column of a source, where a span starts or ends; the error of a position whose line or column is less than 1, which names that line and that column; and the error of the adapter Java Script Object Notation for an export region with such a position, which names the path of its file and carries the error of that position. |
| Span | Where a region lies in a source: the path of the source, the start and the end, which a span refuses where it lies before the start, and the error of the adapter Java Script Object Notation for an export region that is no span, which carries the error of that span; and where a token lies in its source as proc-macro2 locates it, such as the span of a hash mark, whose start lies on the opening line of a test module. |
| Start | The position a span starts at. |
| Text | A string as it was read or as it is written: the text of a source, the success text and the text of a path. |
