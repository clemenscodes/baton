# The report file

The module `file` holds the report file of a measurement, `ReportFile`: the path of the report and
the export that a measurement writes to it. The port `ProfileStore` of
`gates_coverage_core_profile_directory` writes it, and `gates_coverage_adapter_file_system` fills that
port. It does no IO itself.
