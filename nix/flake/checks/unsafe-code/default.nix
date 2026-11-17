{
  findutils,
  gawk,
  gnugrep,
  self,
  stdenvNoCC,
}:
stdenvNoCC.mkDerivation {
  name = "unsafe-code";
  src = self;
  nativeBuildInputs = [
    findutils
    gawk
    gnugrep
  ];
  buildPhase = ''
    refusal=0
    if ! find crates -type f -name '*.rs' -not -path 'crates/stack/engine/adapter/gpu/vulkan/src/vulkan/*' -exec awk '
      {
        line = $0
        gsub(/r#+"[^"]*"#+/, "", line)
        gsub(/"([^"\\]|\\.)*"/, "", line)
        if (line ~ /(^|[^A-Za-z0-9_])unsafe([^A-Za-z0-9_]|$)/) { print FILENAME ":" FNR ":" $0; found = 1 }
      }
      END { exit found }
    ' {} +; then
      echo "the lines above use unsafe outside crates/stack/engine/adapter/gpu/vulkan/src/vulkan, the module that may."
      refusal=1
    fi
    if find crates -type f -name Cargo.toml -not -path 'crates/stack/engine/adapter/gpu/vulkan/Cargo.toml' -exec grep --with-filename --line-number --word-regexp unsafe_code {} +; then
      echo "the lines above set the lint unsafe_code outside the manifest of the workspace and of stack_engine_adapter_gpu_vulkan."
      refusal=1
    fi
    if ! grep --quiet --line-regexp 'unsafe_code = "forbid"' Cargo.toml; then
      echo "the manifest of the workspace must forbid unsafe_code."
      refusal=1
    fi
    if [ "$refusal" -ne 0 ]; then
      exit 1
    fi
  '';
  installPhase = ''
    touch "$out"
  '';
}
