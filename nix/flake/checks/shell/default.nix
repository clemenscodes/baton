{
  findutils,
  shellcheck,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "shell";
  src = source;
  nativeBuildInputs = [
    findutils
    shellcheck
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    find . -type f -print0 | sort --zero-terminated > files
    script_count=0
    finding_count=0
    while IFS= read -r -d "" file; do
      shebang=$(head --lines 1 "$file")
      case "$shebang" in
        '#!/'*sh*)
          script_count=$((script_count + 1))
          if ! shellcheck "$file"; then
            finding_count=$((finding_count + 1))
          fi
          ;;
      esac
    done < files
    if [ "$script_count" = 0 ]; then
      echo '[SHELL] no script was found, so the path filter is wrong.'
      exit 1
    fi
    if [ "$finding_count" != 0 ]; then
      echo "[SHELL] shellcheck read $script_count scripts and turned $finding_count of them away, as it states above."
      exit 1
    fi
    echo "[SHELL] shellcheck read $script_count scripts and found nothing."
  '';
  installPhase = ''
    touch "$out"
  '';
}
