{
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "unused-term";
  src = source;
  nativeBuildInputs = [ vocabulary ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    echo '[VOCABULARY] tool_vocabulary checks the terms of every context against every source under `crates`.'
    tool_vocabulary --terms ./crates
  '';
  installPhase = ''
    touch "$out"
  '';
}
