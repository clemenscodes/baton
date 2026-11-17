{
  stdenvNoCC,
  names,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "outputs";
  src = source;
  inherit names;
  passAsFile = [ "names" ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    sort "$namesPath" > flake_outputs
    grep --only-matching '^| `[a-zA-Z0-9_-]*` | [a-z]*' docs/OUTPUTS.md \
      | tr --delete '|`' \
      | awk '{ print $1, $2 }' \
      | sort > document_outputs
    if ! diff flake_outputs document_outputs; then
      echo '[OUTPUTS] the flake and docs/OUTPUTS.md do not agree.'
      echo '[OUTPUTS] a line marked < is an output nobody recorded, a line marked > a row of an output that is gone.'
      exit 1
    fi
    output_count=$(wc --lines < flake_outputs)
    echo "[OUTPUTS] docs/OUTPUTS.md states all $output_count outputs of the flake."
  '';
  installPhase = ''
    touch "$out"
  '';
}
