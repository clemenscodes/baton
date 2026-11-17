{
  findutils,
  gawk,
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "word";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    echo '[WORD] word reads the names of every source under `crates`, of every manifest, of the Nix and of the shell.'
    tool_vocabulary --tokens ./crates > tokens

    find ./crates -name Cargo.toml -print0 | sort --zero-terminated > manifests
    xargs --null awk '
      FNR == 1 { section = "" }
      /^\[/ { section = $0 }
      section == "[package]" && /^name = "/ {
        name = $0
        gsub(/^name = "|"$/, "", name)
        printf "%s\t%d\t1\tmodule\t%s\n", FILENAME, FNR, name
      }
    ' < manifests >> tokens

    find . -type f -name '*.nix' -print0 | sort --zero-terminated > nix_sources
    xargs --null awk '
      function insert_name(category, name_text) {
        printf "%s\t%d\t1\t%s\t%s\n", FILENAME, FNR, category, name_text
      }
      function insert_shell_names(text) {
        if (match(text, /^[[:space:]]*(export[[:space:]]+|local[[:space:]]+|readonly[[:space:]]+)?[A-Za-z_][A-Za-z0-9_]*=/)) {
          name_text = substr(text, RSTART, RLENGTH - 1)
          sub(/^[[:space:]]*(export[[:space:]]+|local[[:space:]]+|readonly[[:space:]]+)?/, "", name_text)
          insert_name("value", name_text)
        }
        if (match(text, /^[[:space:]]*for[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]+in/)) {
          name_text = substr(text, RSTART, RLENGTH)
          sub(/^[[:space:]]*for[[:space:]]+/, "", name_text)
          sub(/[[:space:]]+in$/, "", name_text)
          insert_name("value", name_text)
        }
        if (match(text, /^[[:space:]]*[A-Za-z_][A-Za-z0-9_]*\(\)[[:space:]]*\{/)) {
          name_text = substr(text, RSTART, RLENGTH)
          sub(/^[[:space:]]*/, "", name_text)
          sub(/\(.*$/, "", name_text)
          insert_name("function", name_text)
        }
      }
      FNR == 1 { indented_string = 0 }
      {
        code = ""
        string_text = ""
        position = 1
        size = length($0)
        plain_string = 0
        while (position <= size) {
          letter = substr($0, position, 1)
          pair = substr($0, position, 2)
          if (indented_string) {
            if (pair == "\047\047") {
              following = substr($0, position + 2, 1)
              if (following == "$" || following == "\047" || following == "\\") {
                string_text = string_text substr($0, position + 2, 1)
                position += 3
                continue
              }
              indented_string = 0
              position += 2
              continue
            }
            string_text = string_text letter
            position += 1
            continue
          }
          if (plain_string) {
            if (letter == "\\") {
              position += 2
              continue
            }
            if (letter == "\"") {
              plain_string = 0
            }
            position += 1
            continue
          }
          if (pair == "\047\047") {
            indented_string = 1
            position += 2
            continue
          }
          if (letter == "\"") {
            plain_string = 1
            position += 1
            continue
          }
          code = code letter
          position += 1
        }
        if (string_text != "") {
          insert_shell_names(string_text)
        }
        if (match(code, /^[[:space:]]*[A-Za-z_][A-Za-z0-9_-]*[[:space:]]*[=,?]/) || match(code, /^[[:space:]]*[A-Za-z_][A-Za-z0-9_-]*:/)) {
          name_text = substr(code, RSTART, RLENGTH - 1)
          gsub(/[[:space:]]/, "", name_text)
          insert_name("value", name_text)
        }
      }
    ' < nix_sources >> tokens

    find . -type f -not -name '*.nix' -not -name '*.rs' -print0 | sort --zero-terminated > files
    while IFS= read -r -d "" file; do
      if head --lines 1 "$file" | grep --quiet --text '^#!/.*sh'; then
          awk '
            function insert_name(category, name_text) {
              printf "%s\t%d\t1\t%s\t%s\n", FILENAME, FNR, category, name_text
            }
            match($0, /^[[:space:]]*(export[[:space:]]+|local[[:space:]]+|readonly[[:space:]]+)?[A-Za-z_][A-Za-z0-9_]*=/) {
              name_text = substr($0, RSTART, RLENGTH - 1)
              sub(/^[[:space:]]*(export[[:space:]]+|local[[:space:]]+|readonly[[:space:]]+)?/, "", name_text)
              insert_name("value", name_text)
            }
            match($0, /^[[:space:]]*for[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]+in/) {
              name_text = substr($0, RSTART, RLENGTH)
              sub(/^[[:space:]]*for[[:space:]]+/, "", name_text)
              sub(/[[:space:]]+in$/, "", name_text)
              insert_name("value", name_text)
            }
            match($0, /^[[:space:]]*[A-Za-z_][A-Za-z0-9_]*\(\)[[:space:]]*\{/) {
              name_text = substr($0, RSTART, RLENGTH)
              sub(/^[[:space:]]*/, "", name_text)
              sub(/\(.*$/, "", name_text)
              insert_name("function", name_text)
            }
          ' "$file" >> tokens
      fi
    done < files

    if ! awk --field-separator '\t' '
      function insert_words(word_text, message,   word_list, word_count, position) {
        word_count = split(word_text, word_list, " ")
        for (position = 1; position <= word_count; position += 1) {
          messages[word_list[position]] = message
        }
      }
      function insert_set(target_set, word_text,   word_list, word_count, position) {
        word_count = split(word_text, word_list, " ")
        for (position = 1; position <= word_count; position += 1) {
          target_set[word_list[position]] = 1
        }
      }
      function report_name(message) {
        printf "[WORD] %s:%s:%s: `%s` %s\n", $1, $2, $3, $5, message
        finding_count += 1
      }
      function select_category(   name_text) {
        name_text = $5
        if ($4 != "declaration") {
          return $4
        }
        if (previous_text == "fn") {
          return "function"
        }
        if (previous_text == "mod") {
          return "module"
        }
        if (previous_text == "const" || previous_text == "static") {
          return "constant"
        }
        if (previous_text == "struct" || previous_text == "enum" || previous_text == "union" || previous_text == "trait" || previous_text == "type") {
          return "type"
        }
        if (name_text ~ /^[A-Z][A-Z0-9_]+$/) {
          return "constant"
        }
        if (name_text ~ /^[A-Z]/) {
          return "type"
        }
        return "value"
      }
      function judge_name(category,   snake_text, word_list, word_count, position, word, first_word, last_word) {
        snake_text = gensub(/([a-z0-9])([A-Z])/, "\\1_\\2", "g", $5)
        snake_text = gensub(/([A-Z]+)([A-Z][a-z])/, "\\1_\\2", "g", snake_text)
        snake_text = tolower(snake_text)
        gsub(/-/, "_", snake_text)
        word_count = split(snake_text, word_list, "_")
        for (position = 1; position <= word_count; position += 1) {
          word = word_list[position]
          if (word in messages) {
            report_name(messages[word])
            return
          }
          if ((word in refusal_words) && $1 !~ refusal_path) {
            report_name("holds a word of a refusal, and only a gate refuses. In the code a failure is an error.")
            return
          }
        }
        if (snake_text ~ /(^|_)(an?|the|one|every|each|its|their|no|unbroken|first|last)_runs?(_|$)/ || snake_text ~ /(^|_)runs?_of(_|$)/) {
          report_name("calls a thing a run. A process runs and nothing else does.")
          return
        }
        first_word = word_list[1]
        if (first_word == "try" && word_count > 1) {
          first_word = word_list[2]
        }
        last_word = word_list[word_count]
        if (category == "function") {
          if (first_word in banned_verbs) {
            report_name("begins with a verb that names no work. Name the real work of the function.")
            return
          }
          if (first_word in person_verbs) {
            report_name("is named for what a person does. Name what the program does.")
            return
          }
          if (first_word == "print") {
            report_name("begins with print. Implement Display and let the call site print.")
            return
          }
          if (snake_text == "new" || word_list[1] == "with") {
            report_name("is a constructor of its own. Every constructor is From or TryFrom.")
            return
          }
          if (snake_text in positions && snake_text != "start") {
            report_name("names a position and never a thing. State the first, the last or the end of what.")
            return
          }
          if (snake_text in empty_participles) {
            report_name("states only that something happened. Name the work or the thing.")
            return
          }
        }
        if (category == "module") {
          if (snake_text in stopgaps) {
            report_name("marks a concept nobody has found yet. Name the concept.")
            return
          }
          if (snake_text in activities) {
            report_name("is an activity and no thing. A module is named for what it is.")
            return
          }
        }
        if (category == "value" || category == "constant") {
          if (snake_text in empty_participles || snake_text in acts) {
            report_name("is named for an act and not for what the value is.")
            return
          }
          if (snake_text in stopgaps) {
            report_name("marks a concept nobody has found yet. Name what the value is.")
            return
          }
          if (snake_text in positions) {
            report_name("names a position and never a thing. State the first, the last, the start or the end of what.")
            return
          }
          if (snake_text in number_words) {
            report_name("is a number. It counts and never states what the value is.")
            return
          }
          if (last_word ~ /[a-z]ed$/ && !(last_word in participle_nouns)) {
            report_name("is a past tense alone. Name what the value is, not what was done to it.")
            return
          }
          if (last_word ~ /[a-z]ing$/ && !(last_word in participle_nouns)) {
            report_name("is named for the act that made it. Name what the value is.")
            return
          }
        }
      }
      BEGIN {
        refusal_path = "^\\./(crates/gates|nix/flake/checks|nix/flake/packages/commit_gate|\\.claude/hooks)/"
        insert_words("carry carries carried carrying", "holds a refused verb. State where a thing is and what it does.")
        insert_words("stand stands standing stood lie lies lying sit sits sitting sat", "holds a refused verb. State where a thing is and what it does.")
        insert_words("hold holds holding held", "holds a refused verb, wrong in every domain of this game. State what the thing is.")
        insert_words("say says said saying speak speaks spoke spoken speaking tell tells told telling ask asks asked asking", "holds a refused verb. Nothing here speaks: a program requests and responds.")
        insert_words("aim aims aimed aiming", "holds a refused verb. A pointer points, it never aims.")
        insert_words("nought noughts", "holds nought. Write zero.")
        insert_words("corpus", "holds corpus. Most names are better without it.")
        insert_words("place places", "holds place. A value that states where a thing is names that thing: an offset, an address, a position, a slot.")
        insert_words("whole", "holds whole. State the thing without it.")
        insert_words("reading readings", "holds reading, which is the act and never the thing. Name what came out of it.")
        insert_words("written given at of over wrote number numbers found one other later behind rest", "holds a word that names nothing. State what the value is.")
        insert_set(refusal_words, "refuse refuses refused refusing refusal refusals")
        insert_set(banned_verbs, "get make build run do handle process execute apply perform")
        insert_set(person_verbs, "take takes hold find collect")
        insert_set(empty_participles, "reads refuses writes taken waiting wanted shown kept settled made describes covers mapped readable done own")
        insert_set(acts, "read walked gathered folded trimmed satisfied finished annotated marked padded named opens kill resolve cut there here")
        insert_set(stopgaps, "util utils helper helpers common misc shared")
        insert_set(activities, "strip parse unpack load sync")
        insert_set(positions, "first last start end inside")
        insert_set(number_words, "two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety hundred thousand million")
        insert_set(participle_nouns, "beginning being binding ceiling encoding ending feed finding heading listing meaning naming need packing padding red ring seed setting speed spelling string king plumbing tooling shared extended sibling")
      }
      $4 == "declaration" || $4 == "module" || $4 == "value" || $4 == "function" {
        judge_name(select_category())
      }
      $5 != "mut" && $5 != "ref" {
        previous_text = $5
      }
      END {
        if (finding_count > 0) {
          printf "[WORD] %d names hold a word this repository does not admit, as docs/STYLE.md states.\n", finding_count
          printf "[WORD] the gate reads names and leaves prose alone: a document, a string, a definition and the sentence of a test.\n"
          exit 1
        }
      }
    ' tokens; then
      exit 1
    fi
    name_count=$(wc --lines < tokens)
    echo "[WORD] every name of $name_count tokens was read, and none of them holds a word this repository does not admit."
  '';
  installPhase = ''
    touch "$out"
  '';
}
