{
  diffutils,
  findutils,
  lib,
  stdenvNoCC,
  crateDirectories,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "ubiquitous-language";
  src = source;
  nativeBuildInputs = [
    diffutils
    findutils
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    find crates -path '*/docs/UBIQUITOUS_LANGUAGE.md' | sort > books.txt
    while read -r book; do
      cp "$book" "$book.original"
    done < books.txt

    tool_vocabulary --book > report.txt
    cat report.txt

    sed --quiet 's/^\[UBIQUITOUS\] `\(.*\/docs\/UBIQUITOUS_LANGUAGE\.md\)` now contains [0-9]* term(s)\.$/\1/p' report.txt | sort > edition.txt
    refusal_count=0
    if ! diff books.txt edition.txt > difference.txt; then
      echo "[UBIQUITOUS] tool_vocabulary prints a book that the repository does not have, or the repository has a book that tool_vocabulary does not print."
      cat difference.txt
      refusal_count="$((refusal_count + 1))"
    fi
    while read -r book; do
      if ! diff --unified "$book.original" "$book" > difference.txt; then
        printf '[UBIQUITOUS] `%s` does not match the vocabulary it is printed from.\n' "$book"
        cat difference.txt
        refusal_count="$((refusal_count + 1))"
      fi
    done < books.txt
    printf '%s\n' ${lib.escapeShellArgs crateDirectories} > directories.txt
    find crates/gates/vocabulary/gate/src/domain/contexts -name mod.rs | sort > modules.txt
    while read -r module; do
      if ! grep --quiet 'DIRECTORY_NAME' "$module"; then
        continue
      fi
      context="$(dirname "$module" | cut --delimiter='/' --fields=8-)"
      directory="crates/$context"
      if grep --quiet --fixed-strings --line-regexp "$directory" directories.txt; then
        if [ ! -f "$directory/docs/UBIQUITOUS_LANGUAGE.md" ]; then
          printf '[UBIQUITOUS] `%s` has a context but no book `%s`.\n' "$directory" "$directory/docs/UBIQUITOUS_LANGUAGE.md"
          refusal_count="$((refusal_count + 1))"
        fi
      else
        printf '[UBIQUITOUS] the context `%s` does not correspond to any crate of the flake.\n' "$context"
        refusal_count="$((refusal_count + 1))"
      fi
    done < modules.txt

    if [ "$refusal_count" != 0 ]; then
      echo "[UBIQUITOUS] a book contains exactly what the vocabulary states, and nothing else."
      echo '[UBIQUITOUS] run `nix run .#vocabulary -- --book` to print them all again.'
      exit 1
    fi
    printf '[UBIQUITOUS] each of the %s books matches its vocabulary.\n' "$(wc --lines < books.txt)"
  '';
  installPhase = ''
    touch "$out"
  '';
}
