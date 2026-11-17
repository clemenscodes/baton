{
  lib,
  self,
}:
crate:
let
  inherit (lib)
    attrNames
    attrValues
    concatMap
    concatStringsSep
    foldl'
    genericClosure
    init
    isAttrs
    optional
    splitString
    ;
  workspaceDependencies =
    (builtins.fromTOML (builtins.readFile "${self}/Cargo.toml")).workspace.dependencies;
  canonicalDirectory =
    baseDirectory: relativePath:
    concatStringsSep "/" (
      foldl' (
        segments: segment:
        if segment == ".." then
          init segments
        else if segment == "." || segment == "" then
          segments
        else
          segments ++ [ segment ]
      ) [ ] (splitString "/" "${baseDirectory}/${relativePath}")
    );
  dependencyDirectories =
    directory: dependencyTable:
    concatMap (
      dependencyName:
      let
        specification = dependencyTable.${dependencyName};
        workspaceSpecification = workspaceDependencies.${dependencyName} or { };
      in
      if !isAttrs specification then
        [ ]
      else if specification.workspace or false then
        optional (workspaceSpecification ? path) (canonicalDirectory "" workspaceSpecification.path)
      else
        optional (specification ? path) (canonicalDirectory directory specification.path)
    ) (attrNames dependencyTable);
  tableDirectories =
    tableNames: directory:
    let
      manifest = builtins.fromTOML (builtins.readFile "${self}/${directory}/Cargo.toml");
      platformTables = attrValues (manifest.target or { });
    in
    concatMap (
      table: concatMap (tableName: dependencyDirectories directory (table.${tableName} or { })) tableNames
    ) ([ manifest ] ++ platformTables);
  closureItem = key: { inherit key; };
  crateDirectories = map (item: item.key) (genericClosure {
    startSet = map closureItem (
      [ crate.directory ] ++ tableDirectories [ "dev-dependencies" ] crate.directory
    );
    operator =
      item:
      map closureItem (
        tableDirectories [
          "dependencies"
          "build-dependencies"
        ] item.key
      );
  });
  compilationPaths = directory: (import "${self}/${directory}/nix").compilationPaths or [ ];
in
[
  "Cargo.toml"
  "Cargo.lock"
  ".config/nextest.toml"
]
++ concatMap (
  directory:
  [
    "${directory}/Cargo.toml"
    "${directory}/src"
  ]
  ++ compilationPaths directory
) crateDirectories
++ crate.testPaths or [ ]
