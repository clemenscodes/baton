{
  findutils,
  gawk,
  self,
  stdenvNoCC,
}:
stdenvNoCC.mkDerivation {
  name = "comment";
  src = self;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  buildPhase = ''
    if ! find . -type f \( -name '*.nix' -or -name '*.sh' -or -name '*.py' \) -exec awk '
      FNR == 1 && /^#!/ { next }
      /^[[:space:]]*#/ { print FILENAME ":" FNR ":" $0; comment = 1 }
      END { exit comment }
    ' {} +; then
      echo "the lines above are comments, and code must not contain comments."
      exit 1
    fi
    if ! find . -type f -name '*.rs' -exec awk '
      FNR == 1 {
        inside_string = 0
        inside_raw_string = 0
      }
      {
        position = 1
        size = length($0)
        while (position <= size) {
          letter = substr($0, position, 1)
          remainder = substr($0, position)
          if (inside_string) {
            if (letter == "\\") {
              position += 2
              continue
            }
            if (letter == "\"") {
              inside_string = 0
            }
            position += 1
            continue
          }
          if (inside_raw_string) {
            if (index(remainder, raw_string_end) == 1) {
              inside_raw_string = 0
              position += length(raw_string_end)
              continue
            }
            position += 1
            continue
          }
          if (remainder ~ /^(\/\/|\/\*)/) {
            print FILENAME ":" FNR ":" $0
            comment = 1
            break
          }
          if (match(remainder, /^b?r#*"/) && (position == 1 || substr($0, position - 1, 1) !~ /[A-Za-z0-9_]/)) {
            raw_string_end = substr(remainder, 1, RLENGTH)
            sub(/^b?r/, "", raw_string_end)
            raw_string_end = "\"" substr(raw_string_end, 1, length(raw_string_end) - 1)
            inside_raw_string = 1
            position += RLENGTH
            continue
          }
          if (letter == "\"") {
            inside_string = 1
            position += 1
            continue
          }
          if (match(remainder, /^\047(\\[^\047]*|\\\047|[^\\\047])\047/)) {
            position += RLENGTH
            continue
          }
          position += 1
        }
      }
      END { exit comment }
    ' {} +; then
      echo "the lines above are comments of Rust, and a crate contains no comment, in its tests neither."
      exit 1
    fi
  '';
  installPhase = ''
    touch "$out"
  '';
}
