{
  findutils,
  gawk,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "sibling";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    find crates -name Cargo.toml -printf '%h\n' | sort > crate_roots.txt
    find crates -type d | sort > directories.txt
    awk '
      FNR == NR { crate_root[$0] = 1; next }
      {
        path = $0
        segment_count = split(path, segments, "/")
        excluded = 0
        prefix = segments[1]
        for (segment_index = 2; segment_index <= segment_count; segment_index++) {
          if ((prefix in crate_root) && segments[segment_index] ~ /^(docs|nix|assets|tests)$/) {
            excluded = 1
          }
          prefix = prefix "/" segments[segment_index]
        }
        if (excluded || segment_count < 2) {
          next
        }
        parent = segments[1]
        for (segment_index = 2; segment_index < segment_count; segment_index++) {
          parent = parent "/" segments[segment_index]
        }
        name = segments[segment_count]
        if ((parent in crate_root) && name ~ /^(docs|src|nix|assets|tests)$/) {
          next
        }
        printf "%s\t%s\t/\n", parent, name
      }
    ' crate_roots.txt directories.txt > siblings.txt
    find crates -path '*/src/*' -name '*.rs' | sort > sources.txt
    while read -r source_file; do
      awk -v source_file="$source_file" '
        function enter_scope(scope_name) {
          depth++
          scope[depth] = scope[depth - 1] "::" scope_name
        }
        {
          line = $0
          if (raw_string) {
            if (index(line, "\"#") > 0) {
              line = substr(line, index(line, "\"#") + 2)
              raw_string = 0
            } else {
              next
            }
          }
          while (match(line, /r#+"/)) {
            rest = substr(line, RSTART + RLENGTH)
            if (index(rest, "\"#") > 0) {
              line = substr(line, 1, RSTART - 1) substr(rest, index(rest, "\"#") + 2)
            } else {
              line = substr(line, 1, RSTART - 1)
              raw_string = 1
            }
          }
          gsub(/\\./, "", line)
          gsub(/"[^"]*"/, "\"\"", line)
          gsub(/\047[^\047]\047/, "", line)
          sub(/\/\/.*/, "", line)
          if (match(line, /^[ \t]*(pub(\([^)]*\))?[ \t]+)?mod[ \t]+[a-z0-9_]+[ \t]*[;{]/)) {
            declaration = substr(line, RSTART, RLENGTH)
            module_name = declaration
            sub(/^[ \t]*(pub(\([^)]*\))?[ \t]+)?mod[ \t]+/, "", module_name)
            sub(/[ \t]*[;{]$/, "", module_name)
            kind = (declaration ~ /\{$/) ? "inline" : "file"
            printf "%s%s\t%s\t%s\n", source_file, scope[depth], module_name, kind
            if (kind == "inline") {
              enter_scope(module_name)
              line = substr(line, RSTART + RLENGTH)
            }
          }
          opening_count = gsub(/\{/, "{", line)
          closing_count = gsub(/\}/, "}", line)
          for (brace_index = 0; brace_index < opening_count; brace_index++) {
            enter_scope("{}")
          }
          for (brace_index = 0; brace_index < closing_count; brace_index++) {
            if (depth > 0) {
              depth--
            }
          }
        }
      ' "$source_file"
    done < sources.txt > modules.txt
    awk -F '\t' '
      {
        if (FILENAME == "siblings.txt") {
          key = $1; kind = "/"
        } else {
          key = $1; kind = $3
        }
        if (key ~ /::\{\}/) {
          next
        }
        word = $2
        sub(/_.*/, "", word)
        group = key "\t" word
        if ((group, $2) in member) {
          next
        }
        member[group, $2] = kind
        names[group] = names[group] " " $2
        if (kind == "inline") {
          inline_presence[group] = 1
        }
        groups[group] = 1
      }
      END {
        for (group in groups) {
          name_count = split(substr(names[group], 2), group_names, " ")
          if (name_count < 2) {
            continue
          }
          if (index(group, "::") == 0 && index(group, ".rs") > 0 && !(group in inline_presence)) {
            continue
          }
          split(group, group_parts, "\t")
          separator = (index(group, ".rs") > 0) ? "::" : "/"
          for (name_index = 2; name_index <= name_count; name_index++) {
            printf "[SIBLING] %s: `%s` and `%s` share `%s`; make them `%s%s...`.\n", group_parts[1], group_names[1], group_names[name_index], group_parts[2], group_parts[2], separator
          }
        }
      }
    ' siblings.txt modules.txt | sort > findings.txt
    if [ -s findings.txt ]; then
      echo "[SIBLING] two siblings whose names begin with the same word miss the parent that word names, as docs/STYLE.md states in \"Files and modules\"."
      cat findings.txt
      printf '[SIBLING] %s pairs of siblings share their first word.\n' "$(wc --lines < findings.txt)"
      exit 1
    fi
    printf '[SIBLING] none of the %s directories and modules shares its first word with a sibling.\n' "$(cat siblings.txt modules.txt | wc --lines)"
  '';
  installPhase = ''
    touch "$out"
  '';
}
