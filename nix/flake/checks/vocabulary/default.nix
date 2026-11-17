{
  stdenvNoCC,
  directory,
  name,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "vocabulary-${name}";
  src = source;
  nativeBuildInputs = [ vocabulary ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    echo "[VOCABULARY] tool_vocabulary checks every source under \`${directory}\`."
    tool_vocabulary "./${directory}"
  '';
  installPhase = ''
    touch "$out"
  '';
}
