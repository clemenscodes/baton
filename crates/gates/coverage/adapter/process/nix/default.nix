{
  name = "gates_coverage_adapter_process";
  check =
    { test-processes }:
    {
      endToEnd = true;
      inputs = [ ];
      environment = {
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER = "${test-processes}/bin/test-processes";
      };
    };
}
