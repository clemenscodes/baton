{
  findutils,
  gawk,
  writeShellApplication,
}:
writeShellApplication {
  name = "dependency_table";
  excludeShellChecks = [ "SC2016" ];
  runtimeInputs = [
    findutils
    gawk
  ];
  text = ''
    find crates -name Cargo.toml -print0 | sort --zero-terminated > manifests
    xargs --null awk '
      function select_alias(line_text) {
        sub(/[ .=].*$/, "", line_text)
        return line_text
      }
      FNR == 1 {
        section = ""
        directory = FILENAME
        sub(/\/?Cargo\.toml$/, "", directory)
      }
      /^\[/ {
        section = $0
        next
      }
      FILENAME == "Cargo.toml" && section == "[workspace.dependencies]" && match($0, /path = "[^"]*"/) {
        path_text = substr($0, RSTART + 8, RLENGTH - 9)
        workspace_paths[select_alias($0)] = path_text
        next
      }
      FILENAME != "Cargo.toml" && /^[A-Za-z0-9_-]/ {
        alias = select_alias($0)
        scope = ""
        if (section == "[dependencies]" || section ~ /^\[target\..*\.dependencies\]$/) {
          scope = "production"
        }
        if (section == "[dev-dependencies]" || section ~ /^\[target\..*\.dev-dependencies\]$/) {
          scope = "development"
        }
        if (scope != "" && (alias in workspace_paths)) {
          printf "%s\t%s\t%s\n", directory, scope, workspace_paths[alias]
        }
      }
    ' Cargo.toml < manifests | sort --unique
  '';
}
