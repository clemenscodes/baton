{
  coreutils,
  writeShellApplication,
}:
writeShellApplication {
  name = "test-processes";
  runtimeInputs = [ coreutils ];
  text = ''
    test_binary=$1
    shift
    "$test_binary" --list --format terse | while IFS= read -r test_line; do
      test_name=''${test_line%: test}
      if [ "$test_name" != "$test_line" ]; then
        timeout 300 "$test_binary" --exact "$test_name" "$@"
      fi
    done
  '';
}
