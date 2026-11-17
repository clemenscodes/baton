{
  findutils,
  gawk,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "verb";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    : > refusals.txt
    : > functions.txt
    find crates/gates/vocabulary/gate/src/domain/contexts -name mod.rs | sort > contexts.txt
    while read -r context; do
      directory="$(sed --quiet 's/.*DIRECTORY_NAME: &.static str = "\([^"]*\)";.*/\1/p' "$context")"
      if [ -z "$directory" ]; then
        continue
      fi
      if [ ! -d "crates/$directory" ]; then
        continue
      fi
      awk '
        /let spelling: &\[&str\] = / {
          spelling_count++
          spelling_text = $0
          while (spelling_text !~ /\];/ && (getline spelling_line) > 0) {
            spelling_text = spelling_text " " spelling_line
          }
          gsub(/[[:space:]]+/, " ", spelling_text)
          if (!match(spelling_text, /\[ ?("[A-Z][a-z0-9]*"(, "[A-Z][a-z0-9]*")*),? ?\]/, term)) {
            next
          }
          read_count++
          word = tolower(term[1])
          gsub(/"/, "", word)
          gsub(/, /, "_", word)
          getline definition
          while (definition !~ /"/ && (getline next_line) > 0) {
            definition = next_line
          }
          sub(/^[^"]*"/, "", definition)
          sub(/".*$/, "", definition)
          if (definition ~ /^To /) {
            if (word ~ /_/) print word "\tTo"
          } else if (word !~ /_/) {
            print word "\t" definition
          }
        }
        END {
          if (spelling_count != read_count) {
            printf "%s builds %d terms, but this gate can read the spelling of only %d of them.\n", FILENAME, spelling_count, read_count > "/dev/stderr"
            exit 1
          }
        }
      ' "$context" > nouns.tsv
      find "crates/$directory" -name '*.rs' -not -path '*/target/*' | sort > sources.txt
      while read -r source_file; do
        awk --assign source_file="$source_file" '
          FNR == NR {
            split($0, entry, "\t")
            if (entry[2] == "To") {
              phrase[entry[1]] = 1
            } else {
              noun[entry[1]] = entry[2]
            }
            next
          }
          function stem_word(word) {
            if (word in noun) return word
            if (word ~ /ies$/ && (substr(word, 1, length(word) - 3) "y") in noun) {
              return substr(word, 1, length(word) - 3) "y"
            }
            if (word ~ /es$/ && substr(word, 1, length(word) - 2) in noun) {
              return substr(word, 1, length(word) - 2)
            }
            if (word ~ /s$/ && substr(word, 1, length(word) - 1) in noun) {
              return substr(word, 1, length(word) - 1)
            }
            return ""
          }
          function judge_function(   words, head, stem, verb_phrase, term, line_index, getter) {
            split(name, words, "_")
            head = words[1] == "try" ? words[2] : words[1]
            stem = name
            sub(/^try_/, "", stem)
            for (verb_phrase in phrase) {
              if (stem == verb_phrase || index(stem, verb_phrase "_") == 1) return
            }
            term = stem_word(head)
            if (term == "") return
            getter = body_line_count == 1
            if (!getter && body_lines[1] ~ /^[[:space:]]*match self \{$/) {
              getter = 1
              for (line_index = 2; line_index <= body_line_count; line_index++) {
                if (body_lines[line_index] !~ /=>|^[[:space:]]*\}$/) getter = 0
              }
            }
            if (getter) return
            printf "%s:%d: `fn %s`\t%s\t%s\t%s\n", source_file, header_line, name, head, term, noun[term]
          }
          /^#\[cfg\(test\)\]$/ {
            test_module = 1
          }
          test_module {
            next
          }
          state == "body" {
            if ($0 == indentation "}") {
              judge_function()
              state = ""
            } else if ($0 !~ /^[[:space:]]*$/) {
              body_lines[++body_line_count] = $0
            }
            next
          }
          state == "signature" {
            if ($0 ~ /;[[:space:]]*$/ && $0 !~ /\{/) {
              state = ""
            } else if ($0 ~ /\{[[:space:]]*$/) {
              state = "body"
              if ($0 ~ /-> bool \{/) {
                name = ""
              }
            }
            next
          }
          match($0, /^([[:space:]]*)(pub(\([a-z]+\))?[[:space:]]+)?(const[[:space:]]+)?fn[[:space:]]+([a-z_0-9]+)/, header) {
            if (previous_line ~ /#\[test\]/) {
              previous_line = $0
              next
            }
            indentation = header[1]
            name = header[5]
            header_line = FNR
            body_line_count = 0
            delete body_lines
            if ($0 ~ /-> bool \{/) {
              name = ""
            }
            if ($0 ~ /\{.*\}[[:space:]]*$/) {
              body_line_count = 1
              judge_function()
            } else if ($0 ~ /\{[[:space:]]*$/) {
              state = "body"
            } else if ($0 !~ /;[[:space:]]*$/) {
              state = "signature"
            }
          }
          $0 !~ /^[[:space:]]*$/ { previous_line = $0 }
        ' nouns.tsv "$source_file" >> refusals.txt
        awk '/fn / { function_line_count++ } END { print function_line_count + 0 }' "$source_file" >> functions.txt
      done < sources.txt
    done < contexts.txt
    if [ -s refusals.txt ]; then
      while IFS=$'\t' read -r function head term definition; do
        echo "[VERB] $function is named for a thing, but it is not a getter."
        echo "[VERB] \`$head\` is the thing \`$term\` in the book of its crate: $definition"
      done < refusals.txt
      echo "[VERB] a function is named for what it does."
      echo '[VERB] only a getter, whose body is one line or a `match self` whose arms are values, is named for the thing it returns.'
      echo "[VERB] each function above computes something, so its name needs a verb."
      echo '[VERB] put the verb in front of the thing, such as `lex_`, `parse_`, `resolve_`, `count_`, `decode_`, `compare_` or `compose_`.'
      echo '[VERB] define the verb in the context of the crate under `crates/gates/vocabulary/gate/src/domain/contexts`.'
      echo '[VERB] see "Variables are nouns, functions are verbs" in `docs/STYLE.md`.'
      exit 1
    fi
    printf '[VERB] %s lines with `fn` were checked, and every function named for a thing of its book is a getter.\n' "$(awk '{ function_line_count += $1 } END { print function_line_count }' functions.txt)"
  '';
  installPhase = ''
    touch "$out"
  '';
}
