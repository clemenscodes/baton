# The ubiquitous language

| Word | What it means |
|---|---|
| Gap | A span of a source that no test reaches, that lies outside a main path and in the production of that source. |
| Gaps | The gaps of a report in the order of their spans, selected out of the regions of the report by the test boundaries of its sources, which the outcome counts and the gate writes to the errors. |
| Gates Coverage Core Gaps | The crate gates_coverage_core_gaps, the crate of the domain of the coverage that holds the gaps, the regions outside the test modules that no test reaches. |
| Main Path | The path src/main.rs, whose spans are never a gap, because a main parses, delegates and decides nothing. |
