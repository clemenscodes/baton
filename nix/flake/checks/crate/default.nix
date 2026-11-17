{
  cargo-nextest,
  craneLibrary,
  endToEnd,
  environment,
  features ? [ ],
  inputs,
  lib,
  name,
  source,
}:
let
  workspace = craneLibrary.crateNameFromCargoToml { cargoToml = "${source}/Cargo.toml"; };
in
craneLibrary.buildPackage {
  inherit (workspace) version;
  pname = name;
  src = source;
  strictDeps = true;
  doCheck = true;
  preCheck = lib.optionalString endToEnd ''
    export HOME="$TMPDIR"
  '';
  nativeBuildInputs = lib.optional (!endToEnd) cargo-nextest;
  buildInputs = inputs;
  cargoExtraArgs =
    "--package ${name}"
    + lib.optionalString (features != [ ]) " --features ${lib.concatStringsSep "," features}";
  env = {
    NEXTEST_SHOW_PROGRESS = "counter";
  }
  // environment;
  cargoTestCommand =
    if endToEnd then
      "cargoWithProfile test --tests"
    else
      "cargo nextest run --cargo-profile release --profile release \${cargoArtifacts:+--no-tests=warn}";
  extraDummyScript = ''
    mkdir --parents "$out/.config"
    cp "${source}/.config/nextest.toml" "$out/.config/nextest.toml"
  '';
}
