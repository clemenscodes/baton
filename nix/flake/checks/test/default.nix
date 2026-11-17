{
  endToEndCrates,
  findutils,
  gawk,
  gnugrep,
  lib,
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "test";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
    gnugrep
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    finding_count=0

    find ./crates -type f -path '*/tests/*' -not -path '*/tests/assets/*' | sort > test_files
    if [ -s test_files ]; then
      echo '[TEST] a test lives inline in the source it proves, and the directory `tests` holds only `tests/assets`.'
      cat test_files
      finding_count=$((finding_count + 1))
    fi

    report_unit_test_sources() {
      grep --recursive --files-with-matches --include='*.rs' --extended-regexp "$1" ./crates | sort > rule_sources || true
      while IFS= read -r rule_source; do
        end_to_end_presence=0
        for end_to_end_directory in ${
          lib.concatMapStringsSep " " (crate: crate.directory) endToEndCrates
        }; do
          case "$rule_source" in
            "./$end_to_end_directory"/*) end_to_end_presence=1 ;;
          esac
        done
        if [ "$end_to_end_presence" = 0 ]; then
          echo "[TEST] $rule_source: $2 Its crate names no \`endToEnd = true\`."
          finding_count=$((finding_count + 1))
        fi
      done < rule_sources
    }

    report_unit_test_sources 'process::Command|Command::new' 'a unit test starts no program.'
    report_unit_test_sources \
      '(^|[^[:alnum:]_:])(arboard|ash|ash_window|axum|cpal|gilrs|gpu_allocator|softbuffer|TcpListener|TcpStream|UdpSocket|ureq|winit|x11rb)::' \
      'a unit test touches no infrastructure, and this source names a library behind it.'

    for end_to_end_name in ${lib.concatMapStringsSep " " (crate: crate.name) endToEndCrates}; do
      if ! grep --quiet --fixed-strings "package(=$end_to_end_name)" .config/nextest.toml; then
        echo "[TEST] .config/nextest.toml: the default filter leaves \`$end_to_end_name\` in, and nextest runs pure unit tests alone."
        finding_count=$((finding_count + 1))
      fi
    done

    find ./crates -path '*/src/main.rs' -print0 | sort --zero-terminated > sources
    mapfile -d "" source_paths < sources
    if [ "''${#source_paths[@]}" = 0 ]; then
      echo '[TEST] no `src/main.rs` was found under `crates`, so the path filter is wrong.'
      exit 1
    fi
    echo "[TEST] test reads the production code of ''${#source_paths[@]} programs."
    tool_vocabulary --tokens "''${source_paths[@]}" > tokens

    if ! awk --field-separator '\t' '
      function report_decision(path, line, column, text) {
        printf "[TEST] %s:%s:%s: `%s` is a decision inside a main. No test reaches a main, so nothing decides there.\n", path, line, column, text
        finding_count += 1
      }
      $4 == "identifier" && $5 ~ /^(if|else|match|while|for|loop|unwrap_or)$/ {
        report_decision($1, $2, $3, $5)
      }
      $4 == "punctuation" && $5 == ";" && previous_text == "?" {
        report_decision(previous_path, previous_line, previous_column, "?")
      }
      {
        previous_text = $5
        previous_path = $1
        previous_line = $2
        previous_column = $3
      }
      END {
        if (finding_count > 0) {
          printf "[TEST] a main parses the command line, delegates, reports and sets the status.\n"
          exit 1
        }
      }
    ' tokens; then
      finding_count=$((finding_count + 1))
    fi

    if [ "$finding_count" != 0 ]; then
      exit 1
    fi
    echo "[TEST] every test lives in its source, and no main decides anything."
  '';
  installPhase = ''
    touch "$out"
  '';
}
