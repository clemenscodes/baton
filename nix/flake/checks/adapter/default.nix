{
  dependencyTable,
  findutils,
  gawk,
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "adapter";
  src = source;
  nativeBuildInputs = [
    dependencyTable
    findutils
    gawk
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    tool_vocabulary --tokens ./crates > tokens
    dependency_table > dependencies
    find crates -name Cargo.toml -printf '%h\n' | sort > crate_directories
    find crates -path '*/src/main.rs' | sed 's|/src/main\.rs$||' | sort > composition_roots

    awk --field-separator '\t' '
      FILENAME == "crate_directories" {
        crate_directories[$1] = 1
        next
      }
      {
        if (second_text == "type" && first_text == "Layer" && $5 == "=") {
          inside_binding = 1
        } else if (inside_binding && $5 == ";") {
          inside_binding = 0
          directory = $1
          sub(/^\.\//, "", directory)
          while (directory != "" && directory != "." && !(directory in crate_directories)) {
            sub(/\/?[^\/]*$/, "", directory)
          }
          layers[directory] = layers[directory] " " first_text
        }
        second_text = first_text
        first_text = $5
      }
      END {
        for (directory in layers) {
          remainder = layers[directory]
          gsub(/ InfrastructureLayer/, "", remainder)
          if (remainder == "") {
            print directory
          }
        }
      }
    ' crate_directories tokens | sort > adapters
    adapter_count=$(wc --lines < adapters)
    if [ "$adapter_count" = 0 ]; then
      echo '[ADAPTER] no adapter was found, so the reading of the layers is wrong.'
      exit 1
    fi
    echo "[ADAPTER] $adapter_count crates are adapters, as every type of them names the layer of the infrastructure."

    if ! awk --field-separator '\t' '
      FILENAME == "adapters" {
        adapters[$1] = 1
        next
      }
      FILENAME == "composition_roots" {
        composition_roots[$1] = 1
        next
      }
      $2 == "production" && ($3 in adapters) && !($1 in composition_roots) && !($1 in adapters) {
        printf "[ADAPTER] %s/Cargo.toml reaches the adapter %s, and only the `main` that composes a program may name one.\n", $1, $3
        finding_count += 1
      }
      END {
        if (finding_count > 0) {
          printf "[ADAPTER] %d crates reach an adapter. Name the port instead, and let the program compose the adapter.\n", finding_count
          exit 1
        }
      }
    ' adapters composition_roots dependencies; then
      exit 1
    fi
    echo "[ADAPTER] no crate but a program reaches an adapter."
  '';
  installPhase = ''
    touch "$out"
  '';
}
