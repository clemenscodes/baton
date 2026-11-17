{
  self,
  statix,
  stdenvNoCC,
}:
stdenvNoCC.mkDerivation {
  name = "statix";
  src = self;
  nativeBuildInputs = [ statix ];
  buildPhase = ''
    statix check .
  '';
  installPhase = ''
    touch "$out"
  '';
}
