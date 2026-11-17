{
  craneLibrary,
  name,
  source,
  targetTriple,
}:
let
  workspace = craneLibrary.crateNameFromCargoToml { cargoToml = "${source}/Cargo.toml"; };
in
craneLibrary.mkCargoDerivation {
  inherit (workspace) version;
  pname = "${name}-${targetTriple}";
  src = source;
  strictDeps = true;
  cargoArtifacts = null;
  doInstallCargoArtifacts = false;
  buildPhaseCargoCommand = "cargo check --release --locked --target ${targetTriple} --package ${name}";
  installPhaseCommand = ''
    mkdir --parents "$out"
  '';
}
