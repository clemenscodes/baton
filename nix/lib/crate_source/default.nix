{
  lib,
  self,
}:
crate:
let
  inherit (lib) any hasPrefix removePrefix;
  inherit (lib.sources) cleanSourceWith;
  members = (builtins.fromTOML (builtins.readFile "${self}/Cargo.toml")).workspace.members;
in
cleanSourceWith {
  inherit (crate) name;
  src = self;
  filter =
    path: type:
    let
      relativePath = removePrefix "${self}/" (toString path);
    in
    any (prefix: hasPrefix prefix relativePath || hasPrefix relativePath prefix) crate.paths
    || (type == "directory" && any (member: hasPrefix relativePath member) members)
    || any (member: relativePath == "${member}/Cargo.toml") members;
}
