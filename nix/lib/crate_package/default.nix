{ craneLibrary }:
{
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
  doCheck = false;
  cargoExtraArgs = "--package ${name}";
}
