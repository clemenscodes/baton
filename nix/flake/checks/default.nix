{
  callPackage,
  cargo-llvm-cov,
  coverage,
  craneLibrary,
  crateSource,
  crates,
  formatter,
  lib,
  outputNames,
  rustToolchain,
  self,
  vocabulary,
}:
let
  inherit (lib)
    any
    attrNames
    concatMap
    concatStringsSep
    filter
    hasPrefix
    hasSuffix
    listToAttrs
    mapAttrsToList
    nameValuePair
    optionalAttrs
    removePrefix
    ;
  inherit (lib.sources) cleanSourceWith;
  sourceTree =
    name: paths:
    cleanSourceWith {
      inherit name;
      src = self;
      filter =
        path: _type:
        let
          relativePath = removePrefix "${self}/" (toString path);
        in
        any (prefix: hasPrefix prefix relativePath || hasPrefix relativePath prefix) paths;
    };
  manifestTree =
    name:
    cleanSourceWith {
      inherit name;
      src = self;
      filter =
        path: type:
        let
          fileName = baseNameOf path;
        in
        type == "directory" || fileName == "Cargo.toml" || fileName == "moon.yml";
    };
  checkSetting =
    crate:
    if crate ? check then
      callPackage crate.check { }
    else
      {
        inputs = [ ];
        environment = { };
      };
  endToEndSetting = setting: setting.endToEnd or false;
  coverageSetting =
    crate:
    let
      setting = checkSetting crate;
      extension = {
        arguments = [ ];
        environment = { };
        tools = [ ];
      }
      // optionalAttrs (crate ? coverageCheck) (callPackage crate.coverageCheck { });
    in
    setting
    // extension
    // {
      environment = setting.environment // extension.environment;
    };
  contextCrates = filter (
    crate:
    builtins.pathExists "${self}/crates/gates/vocabulary/gate/src/domain/contexts/${removePrefix "crates/" crate.directory}"
  ) crates;
  crateChecks = listToAttrs (
    map (
      crate:
      nameValuePair crate.name (
        callPackage ./crate {
          inherit craneLibrary;
          inherit (crate) name;
          inherit (checkSetting crate) inputs environment;
          endToEnd = endToEndSetting (checkSetting crate);
          source = crateSource crate;
        }
      )
    ) crates
  );
  featureChecks = listToAttrs (
    concatMap (
      crate:
      map (
        feature:
        nameValuePair "feature-${feature}-${crate.name}" (
          callPackage ./crate {
            inherit craneLibrary;
            inherit (crate) name;
            inherit (checkSetting crate) inputs environment;
            endToEnd = endToEndSetting (checkSetting crate);
            features = [ feature ];
            source = crateSource crate;
          }
        )
      ) crate.features
    ) (filter (crate: crate ? features) crates)
  );
  compilationChecks = listToAttrs (
    concatMap (
      crate:
      mapAttrsToList (
        targetName: targetTriple:
        nameValuePair "compilation-${targetName}-${crate.name}" (
          callPackage ./compilation {
            inherit craneLibrary targetTriple;
            inherit (crate) name;
            source = crateSource crate;
          }
        )
      ) crate.compilationTargets
    ) (filter (crate: crate ? compilationTargets) crates)
  );
  vocabularyChecks = listToAttrs (
    map (
      crate:
      nameValuePair "vocabulary-${crate.name}" (
        callPackage ./vocabulary {
          inherit vocabulary;
          inherit (crate) name;
          inherit (crate) directory;
          source = sourceTree "vocabulary-${crate.name}" crate.paths;
        }
      )
    ) contextCrates
  );
  coverageChecks = listToAttrs (
    map (
      crate:
      nameValuePair "coverage-${crate.name}" (
        callPackage ./coverage {
          inherit cargo-llvm-cov coverage craneLibrary;
          inherit (crate) name;
          endToEnd = endToEndSetting (checkSetting crate);
          inherit (coverageSetting crate)
            arguments
            environment
            inputs
            tools
            ;
          source = crateSource crate;
          foreignDirectories = map (foreignCrate: foreignCrate.directory) (
            filter (foreignCrate: foreignCrate.name != crate.name) crates
          );
        }
      )
    ) crates
  );
  dependencyTable = callPackage ./dependency_table { };
  gates =
    crateChecks
    // featureChecks
    // compilationChecks
    // vocabularyChecks
    // coverageChecks
    // treeChecks;
  treeChecks = {
    format = callPackage ./format { inherit formatter self; };
    statix = callPackage ./statix { inherit self; };
    deadnix = callPackage ./deadnix { inherit self; };
    comment = callPackage ./comment { inherit self; };
    unsafe-code = callPackage ./unsafe-code { inherit self; };
    ubiquitous-language = callPackage ./ubiquitous-language {
      inherit vocabulary;
      crateDirectories = map (crate: crate.directory) crates;
      source = sourceTree "ubiquitous-language" [ "crates" ];
    };
    participle = callPackage ./participle {
      source = sourceTree "participle" [ "crates/gates/vocabulary/gate/src/domain/contexts" ];
    };
    sibling = callPackage ./sibling {
      source = sourceTree "sibling" [ "crates" ];
    };
    verb = callPackage ./verb {
      source = sourceTree "verb" [ "crates" ];
    };
    unused-term = callPackage ./unused-term {
      inherit vocabulary;
      source = sourceTree "unused-term" [
        "Cargo.toml"
        "crates"
      ];
    };
    rustfmt = callPackage ./rustfmt {
      inherit rustToolchain;
      source = sourceTree "rustfmt" [
        "Cargo.toml"
        "Cargo.lock"
        "crates"
        "rustfmt.toml"
      ];
    };
    word = callPackage ./word {
      inherit vocabulary;
      source = sourceTree "word" [
        ".claude"
        ".vscode"
        "crates"
        "flake.nix"
        "nix"
      ];
    };
    signature = callPackage ./signature {
      inherit vocabulary;
      source = sourceTree "signature" [ "crates" ];
    };
    interface = callPackage ./interface {
      inherit vocabulary;
      source = sourceTree "interface" [ "crates" ];
    };
    test = callPackage ./test {
      inherit vocabulary;
      endToEndCrates = filter (crate: endToEndSetting (checkSetting crate)) crates;
      source = sourceTree "test" [
        ".config/nextest.toml"
        "crates"
      ];
    };
    layer = callPackage ./layer {
      inherit dependencyTable vocabulary;
      source = sourceTree "layer" [
        "Cargo.toml"
        "crates"
      ];
    };
    adapter = callPackage ./adapter {
      inherit dependencyTable vocabulary;
      source = sourceTree "adapter" [
        "Cargo.toml"
        "crates"
      ];
    };
    dependencies = callPackage ./dependencies {
      inherit dependencyTable;
      source = manifestTree "dependencies";
    };
    crate-name = callPackage ./crate-name {
      source = manifestTree "crate-name";
    };
    pipeline = callPackage ./pipeline {
      names = concatStringsSep "\n" (attrNames gates) + "\n";
      source = sourceTree "pipeline" [
        ".moon"
        "crates"
        "moon.yml"
        "nix/flake/checks/default.nix"
      ];
    };
    outputs = callPackage ./outputs {
      names = concatStringsSep "\n" (map (name: "${name} check") (attrNames gates) ++ outputNames) + "\n";
      source = sourceTree "outputs" [ "docs/OUTPUTS.md" ];
    };
    book = callPackage ./book {
      source = cleanSourceWith {
        name = "book";
        src = self;
        filter =
          path: type:
          let
            fileName = baseNameOf path;
          in
          type == "directory" || fileName == "book.toml" || hasSuffix ".md" fileName;
      };
    };
    shell = callPackage ./shell {
      source = sourceTree "shell" [
        ".claude"
        ".vscode"
      ];
    };
  };
in
gates
