# The gate

The gate `gate::Gate` answers the query `gate::ReportJudgment` through its
`SynchronousQueryHandler`. The query names where the report is: `ReportJudgment::File` at a path,
which the reader opens, or `ReportJudgment::Input` on an input, such as the standard input. The gate
decodes the report, reads every source it names, parses the test boundaries of each and forms the
gaps of the report outside them, every region no test reaches outside a test module and outside a
`src/main.rs`.

It answers with a `gate::Judgment`. Its `outcome` is `Outcome::Success` where no gap remains and
`Outcome::Error` otherwise, and it reads as the success text or as a line for every gap. A report
whose file cannot be read is `Error::ReportExport`, a report the decoder refuses `Error::Report`, a
source that cannot be read `Error::Source` and a source that is no Rust `Error::Syntax`.

The judgment only reads, so it is a query.
