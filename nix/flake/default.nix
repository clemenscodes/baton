{ inputs, ... }:
{
  systems = [ "x86_64-linux" ];

  flake.nixosModules.default = import ./modules/baton { inherit (inputs) self; };

  perSystem =
    { system, ... }:
    let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.rust-overlay.overlays.default
          (import ./overlays)
        ];
      };

      inherit (inputs) self;

      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ../../rust-toolchain.toml;

      craneLibrary = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      crateSource = import ../lib/crate_source {
        inherit (pkgs) lib;
        inherit self;
      };

      cratePackage = import ../lib/crate_package { inherit craneLibrary; };

      cratePaths = import ../lib/crate_paths {
        inherit (pkgs) lib;
        inherit self;
      };

      crateDefinition =
        directory:
        let
          crate = import ../../crates/${directory}/nix // {
            directory = "crates/${directory}";
          };
        in
        crate // { paths = cratePaths crate; };

      crates = map crateDefinition [
        "stack/domain_driven_design/core"
        "products/baton/presentation/host"
        "products/baton/presentation/command_line_interface"
        "stack/lock/adapter/device/program/berkeley_packet_filter"
        "stack/lock/adapter/device/tree/system_file_system"
        "stack/lock/adapter/task_table/process_file_system"
        "stack/lock/adapter/control_group/unified_hierarchy"
        "stack/lock/adapter/lock_record/file_system"
        "stack/lock/adapter/host_helper/elevation_program"
        "stack/lock/adapter/program_runner/spawner"
        "stack/lock/adapter/pause/standard_library"
        "stack/lock/application"
        "stack/lock/core/graphics/lock"
        "stack/lock/core/demand"
        "stack/lock/core/graphics/user"
        "stack/lock/core/device/file"
        "stack/lock/core/device/address"
        "stack/lock/core/name"
        "stack/process/adapter/spawner/standard_library"
        "stack/process/adapter/spawner/transcript"
        "stack/process/core/invocation"
        "gates/coverage/adapter/file_system"
        "gates/coverage/adapter/java_script_object_notation"
        "gates/coverage/adapter/process"
        "gates/coverage/adapter/syntax"
        "gates/coverage/application"
        "gates/coverage/command_line"
        "gates/coverage/core/compiler"
        "gates/coverage/core/gaps"
        "gates/coverage/core/invocation"
        "gates/coverage/core/mapping_presence"
        "gates/coverage/core/profile/directory"
        "gates/coverage/core/profile/tools"
        "gates/coverage/core/region"
        "gates/coverage/core/report"
        "gates/coverage/core/source"
        "gates/coverage/core/span"
        "gates/coverage/core/test/boundaries"
        "gates/coverage/core/test/build"
        "gates/vocabulary/gate"
        "gates/vocabulary/language/adapter/command_line"
        "gates/vocabulary/language/adapter/file_system"
        "gates/vocabulary/language/adapter/syntax"
        "gates/vocabulary/language/adapter/toms_obvious_minimal_language"
        "gates/vocabulary/language/application"
        "gates/vocabulary/language/core/books"
        "gates/vocabulary/language/core/dependencies"
        "gates/vocabulary/language/core/foreign_name"
        "gates/vocabulary/language/core/language"
        "gates/vocabulary/language/core/languages"
      ];

      vocabularyCrate = crateDefinition "gates/vocabulary/gate";

      coverageCrate = crateDefinition "gates/coverage/command_line";

      batonCrate = crateDefinition "products/baton/presentation/command_line_interface";

      batonHostCrate = crateDefinition "products/baton/presentation/host";

      cargoLlvmCoverage = pkgs.cargo-llvm-cov.overrideAttrs { doCheck = false; };

      formatter = pkgs.nixfmt-tree;

      packages = {
        baton = pkgs.callPackage ./packages/baton {
          inherit cratePackage;
          inherit (batonCrate) name;
          source = crateSource batonCrate;
        };
        baton_host = pkgs.callPackage ./packages/baton_host {
          inherit cratePackage;
          inherit (batonHostCrate) name;
          source = crateSource batonHostCrate;
        };
        commit_gate = pkgs.callPackage ./packages/commit_gate { };
        vocabulary = pkgs.callPackage ./packages/vocabulary {
          inherit cratePackage;
          inherit (vocabularyCrate) name;
          source = crateSource vocabularyCrate;
        };
        coverage = pkgs.callPackage ./packages/coverage {
          inherit cratePackage;
          inherit (coverageCrate) name;
          source = crateSource coverageCrate;
        };
      };

      devShells = {
        default = pkgs.callPackage ./devShells/baton {
          inherit formatter rustToolchain;
          cargo-llvm-cov = cargoLlvmCoverage;
          commitGate = packages.commit_gate;
        };
      };

      checks = pkgs.callPackages ./checks {
        inherit
          craneLibrary
          crateSource
          crates
          formatter
          rustToolchain
          self
          ;
        inherit (packages) coverage vocabulary;
        outputNames =
          map (name: "${name} package") (pkgs.lib.attrNames packages)
          ++ map (name: "${name} shell") (pkgs.lib.attrNames devShells)
          ++ map (name: "${name} module") (pkgs.lib.attrNames self.nixosModules)
          ++ [ "formatter formatter" ];
        cargo-llvm-cov = cargoLlvmCoverage;
      };
    in
    {
      _module.args = { inherit pkgs; };

      inherit
        formatter
        packages
        devShells
        checks
        ;
    };
}
