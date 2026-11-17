{
  findutils,
  gawk,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "participle";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    nouns="Beginning Being Binding Ceiling Encoding Ending Feed Finding Heading Computing Listing Meaning Naming Need Packing Padding Recording Red Ring Seed Setting Speed Spelling"
    irregular_participles="Written Given Taken Found Kept Made Shown Done Gone Known Seen Built Sent Spent Lost Left Brought Bought Thought Told Sold"
    find crates/gates/vocabulary/gate/src/domain/contexts -name mod.rs | sort > sources.txt
    : > terms.txt
    : > participles.txt
    : > unread_contexts.txt
    while read -r source_file; do
      awk '
        BEGIN { RS = "^$" }
        {
          term_count = patsplit($0, constructors, /TermText::new\(/)
          spelling_count = patsplit($0, spellings, /let spelling: &\[&str\] = &\[[^]]*\]/)
          head_count = 0
          for (spelling_index = 1; spelling_index <= spelling_count; spelling_index++) {
            quoted_word_count = patsplit(spellings[spelling_index], quoted_words, /"[^"]+"/)
            if (quoted_word_count > 0) {
              last_quoted_word = quoted_words[quoted_word_count]
              last_word = substr(last_quoted_word, 2, length(last_quoted_word) - 2)
              word_count = split(last_word, words, " ")
              print words[word_count]
              head_count++
            }
          }
          if (head_count != term_count) {
            printf "%s builds %d terms, but this gate can read the head of only %d of them.\n", FILENAME, term_count, head_count >> "unread_contexts.txt"
          }
        }
      ' "$source_file" >> terms.txt
    done < sources.txt
    if [ -s unread_contexts.txt ]; then
      echo "[PARTICIPLE] a context has a term whose spelling this gate cannot read."
      echo '[PARTICIPLE] every term must bind its spelling first, as `let spelling: &[&str] = &[...];`.'
      echo "[PARTICIPLE] otherwise a term this gate cannot read would pass unchecked."
      cat unread_contexts.txt
      exit 1
    fi
    while read -r head; do
      if [[ " $irregular_participles " == *" $head "* ]]; then
        printf '%s\n' "$head" >> participles.txt
        continue
      fi
      if [[ "$head" != *ed && "$head" != *ing ]]; then
        continue
      fi
      if [[ " $nouns " == *" $head "* ]]; then
        continue
      fi
      printf '%s\n' "$head" >> participles.txt
    done < terms.txt
    if [ -s participles.txt ]; then
      echo "[PARTICIPLE] a vocabulary has a term that is a participle."
      echo "[PARTICIPLE] every identifier of a crate is built from the terms of its context."
      echo "[PARTICIPLE] a participle admitted here would be a legal name in that crate from then on."
      echo "[PARTICIPLE] only the last word of a term is checked, because it is the head of the term."
      echo "[PARTICIPLE] if the head really is a noun, add it to the nouns of this gate."
      cat participles.txt
      exit 1
    fi
    printf '[PARTICIPLE] none of the %s terms of the contexts is a participle.\n' "$(wc --lines < terms.txt)"
  '';
  installPhase = ''
    touch "$out"
  '';
}
