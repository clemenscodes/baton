{
  fetchFromGitHub,
  languageServer,
  rustPlatform,
}:
let
  version = "2026-11-09";

  source = fetchFromGitHub {
    owner = "rust-lang";
    repo = "rust-analyzer";
    rev = version;
    hash = "sha256-PBg7irtsrwz2AyXG8sv4FKVMmNo2UBLcXvMDVmNBzl8=";
  };
in
languageServer.overrideAttrs {
  inherit version;
  src = source;
  cargoDeps = rustPlatform.fetchCargoVendor {
    src = source;
    hash = "sha256-cHPXHEzpv7/2FvXBpqGeOU2KGVrDLJl/Jp1TO/dgcKw=";
  };
}
