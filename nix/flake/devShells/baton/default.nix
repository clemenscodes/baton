{
  cargo-llvm-cov,
  cargo-nextest,
  commitGate,
  deadnix,
  formatter,
  git,
  jq,
  mkShellNoCC,
  moon,
  nil,
  rust-formatter,
  rust-language-server,
  rustToolchain,
  statix,
  test-processes,
}:
mkShellNoCC {
  name = "baton";

  packages = [
    rust-formatter
    rust-language-server
    cargo-llvm-cov
    cargo-nextest
    deadnix
    formatter
    git
    jq
    moon
    nil
    rustToolchain
    statix
  ];

  RUSTFMT = "${rust-formatter}/bin/rustfmt";

  NATIVE_TEST_RUNNER = "${test-processes}/bin/test-processes";

  shellHook = ''
    git config hook.commit_gate.event commit-msg
    git config hook.commit_gate.command "${commitGate}/bin/commit_gate"
    moon sync
  '';
}
