{
  rust-formatter,
  rustToolchain,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "rustfmt";
  src = source;
  nativeBuildInputs = [
    rust-formatter
    rustToolchain
  ];
  RUSTFMT = "${rust-formatter}/bin/rustfmt";
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    export CARGO_HOME="$TMPDIR/cargo"
    cargo fmt --all --check
  '';
  installPhase = ''
    touch "$out"
  '';
}
