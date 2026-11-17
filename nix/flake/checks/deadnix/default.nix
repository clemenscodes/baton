{
  deadnix,
  self,
  stdenvNoCC,
}:
stdenvNoCC.mkDerivation {
  name = "deadnix";
  src = self;
  nativeBuildInputs = [ deadnix ];
  buildPhase = ''
    deadnix --fail .
  '';
  installPhase = ''
    touch "$out"
  '';
}
