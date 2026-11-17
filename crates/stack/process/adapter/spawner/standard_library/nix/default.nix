{
  name = "stack_process_adapter_spawner_standard_library";
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
