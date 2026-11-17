final: _previous: {
  test-processes = final.callPackage ../packages/test_processes { };

  rust-formatter = final.callPackage ../packages/rustfmt { };

  rust-language-server = final.callPackage ../packages/rust_analyzer {
    languageServer = final.rust-analyzer-unwrapped;
  };
}
