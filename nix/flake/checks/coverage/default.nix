{
  arguments,
  cargo-llvm-cov,
  cargo-nextest,
  coverage,
  craneLibrary,
  endToEnd,
  environment,
  foreignDirectories,
  inputs,
  lib,
  name,
  source,
  tools,
}:
let
  foreignPattern = "/(" + lib.concatStringsSep "|" foreignDirectories + ")/";
  workspace = craneLibrary.crateNameFromCargoToml { cargoToml = "${source}/Cargo.toml"; };
  argumentText = lib.concatMapStrings (argument: " ${lib.escapeShellArg argument}") arguments;
  testCommand = if endToEnd then "test --tests" else "nextest --no-tests=warn";
in
craneLibrary.mkCargoDerivation {
  inherit (workspace) version;
  pname = "coverage-${name}";
  src = source;
  cargoArtifacts = null;
  strictDeps = true;
  doInstallCargoArtifacts = false;
  nativeBuildInputs = [
    cargo-llvm-cov
    cargo-nextest
    coverage
  ]
  ++ tools;
  buildInputs = inputs;
  env = {
    NEXTEST_SHOW_PROGRESS = "counter";
  }
  // environment;
  buildPhaseCargoCommand = ''
    export HOME="$TMPDIR"
    cargo llvm-cov ${testCommand} --locked --no-cfg-coverage --package "${name}" --ignore-filename-regex '${foreignPattern}' --json --output-path report.json${argumentText}
    tool_coverage < report.json
  '';
  installPhaseCommand = ''
    touch "$out"
  '';
}
