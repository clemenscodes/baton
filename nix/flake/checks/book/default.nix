{
  findutils,
  gnugrep,
  mdbook,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "book";
  src = source;
  nativeBuildInputs = [
    findutils
    gnugrep
    mdbook
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    find . -name book.toml | sort > books
    book_count=$(wc --lines < books)
    if [ "$book_count" = 0 ]; then
      echo '[BOOK] no book was found, so the path filter is wrong.'
      exit 1
    fi
    echo "[BOOK] mdbook reads $book_count books."

    while IFS= read -r manifest; do
      book=$(dirname "$manifest")
      pages="$book/$(sed --quiet 's/^src *= *"\(.*\)"/\1/p' "$manifest")"
      find "$pages" -name '*.md' | sort > first_listing
      html=$(mktemp --directory)
      if ! mdbook build "$book" --dest-dir "$html" > mdbook_output 2>&1; then
        cat mdbook_output
        echo "[BOOK] mdbook cannot build $book, as it states above."
        exit 1
      fi
      if grep --quiet ' WARN ' mdbook_output; then
        cat mdbook_output
        echo "[BOOK] mdbook warns about $book, as it states above, and a warning is an error."
        exit 1
      fi
      find "$pages" -name '*.md' | sort > second_listing
      if ! diff first_listing second_listing; then
        echo "[BOOK] $book names a page that does not exist, and mdbook made it up."
        exit 1
      fi
      grep --only-matching '(\./[^)]*\.md)' "$pages/SUMMARY.md" \
        | tr --delete '()' \
        | sed "s|^\./|$pages/|" \
        | sort > summary_pages
      find "$pages" -name '*.md' -not -name SUMMARY.md | sort > directory_pages
      if ! diff summary_pages directory_pages; then
        echo "[BOOK] $book leaves a page out of its summary, or names one twice."
        exit 1
      fi
    done < books
    echo "[BOOK] all $book_count books build, and every page is in its summary."
  '';
  installPhase = ''
    touch "$out"
  '';
}
