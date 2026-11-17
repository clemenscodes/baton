{
  findutils,
  gawk,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "crate-name";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    find crates -name Cargo.toml -printf '%h\n' | sort > crate_directories
    if [ ! -s crate_directories ]; then
      echo '[CRATE-NAME] no manifest was found under `crates`, so the path filter is wrong.'
      exit 1
    fi
    finding_count=0
    while IFS= read -r crate_directory; do
      expected_name="''${crate_directory#crates/}"
      expected_name="''${expected_name//\//_}"
      package_name="$(awk '/^\[package\]/{inside_package=1; next} /^\[/{inside_package=0} inside_package && /^name *=/{gsub(/[ "]/, ""); sub(/^name=/, ""); print; exit}' "$crate_directory/Cargo.toml")"
      project_identifier="$(awk '/^id:/{gsub(/[ \047"]/, ""); sub(/^id:/, ""); print; exit}' "$crate_directory/moon.yml")"
      if [ "$package_name" != "$expected_name" ]; then
        echo "[CRATE-NAME] the crate $crate_directory is named $package_name, and its path names it $expected_name."
        finding_count=$((finding_count + 1))
      fi
      if [ "$project_identifier" != "$expected_name" ]; then
        echo "[CRATE-NAME] the project of moon of $crate_directory is $project_identifier, and its path names it $expected_name."
        finding_count=$((finding_count + 1))
      fi
    done < crate_directories
    if [ "$finding_count" != 0 ]; then
      exit 1
    fi
  '';
  installPhase = ''
    touch "$out"
  '';
}
