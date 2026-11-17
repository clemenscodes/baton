{
  formatter,
  self,
  stdenvNoCC,
}:
stdenvNoCC.mkDerivation {
  name = "format";
  src = self;
  nativeBuildInputs = [ formatter ];
  buildPhase = ''
    treefmt --ci --tree-root .
  '';
  installPhase = ''
    touch "$out"
  '';
}
