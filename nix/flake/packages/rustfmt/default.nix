{
  fetchurl,
  lib,
  makeRustPlatform,
  makeWrapper,
  rust-bin,
  rustVersion ? (lib.importTOML ../../../../rust-toolchain.toml).toolchain.channel,
  rustcDevelopmentToolchain ? rust-bin.stable.${rustVersion}.minimal.override {
    extensions = [ "rustc-dev" ];
  },
  rustcDevelopmentPlatform ? makeRustPlatform {
    cargo = rustcDevelopmentToolchain;
    rustc = rustcDevelopmentToolchain;
  },
}:
rustcDevelopmentPlatform.buildRustPackage {
  pname = "rustfmt";
  inherit (rustcDevelopmentToolchain) version;

  src = fetchurl {
    url = "https://static.rust-lang.org/dist/rustc-${rustcDevelopmentToolchain.version}-src.tar.gz";
    hash = "sha256-3J+LkXsyRE1sesQ8wbQJAT06mmMzOLtgwUza4dFe5lo=";
  };

  patches = [ ./assignment_rhs_break.patch ];

  cargoVendorDir = "vendor";
  buildAndTestSubdir = "src/tools/rustfmt";
  dontUpdateAutotoolsGnuConfigScripts = true;

  nativeBuildInputs = [ makeWrapper ];

  useNextest = true;

  env = {
    RUSTC_BOOTSTRAP = 1;
    CFG_RELEASE = rustcDevelopmentToolchain.version;
    CFG_RELEASE_CHANNEL = "stable";
    RUSTFLAGS = "-C link-arg=-Wl,-rpath,${rustcDevelopmentToolchain}/lib";
  };

  preCheck = ''
    mkdir --parents .config
    printf '[profile.default]\nslow-timeout = { period = "10s", terminate-after = 2 }\n' > .config/nextest.toml
  '';

  postInstall = ''
    wrapProgram "$out/bin/cargo-fmt" --suffix PATH : ${lib.makeBinPath [ rustcDevelopmentToolchain ]}
  '';

  meta.mainProgram = "rustfmt";
}
