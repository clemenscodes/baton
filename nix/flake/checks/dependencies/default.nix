{
  dependencyTable,
  findutils,
  gawk,
  stdenvNoCC,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "dependencies";
  src = source;
  nativeBuildInputs = [
    dependencyTable
    findutils
    gawk
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    dependency_table > dependencies
    awk '
      /^members = \[/ {
        inside_members = 1
        next
      }
      inside_members && /^\]/ {
        exit
      }
      inside_members {
        gsub(/[ ",]/, "")
        print
      }
    ' Cargo.toml | sort > members
    find crates -name Cargo.toml -printf '%h\n' | sort > crate_directories
    if [ ! -s crate_directories ]; then
      echo '[DEPENDENCIES] no manifest was found under `crates`, so the path filter is wrong.'
      exit 1
    fi
    finding_count=0
    if ! diff members crate_directories > member_difference; then
      echo '[DEPENDENCIES] the members of the workspace are not the crates of the tree.'
      cat member_difference
      echo '[DEPENDENCIES] a line marked < is a member without a crate, a line marked > a crate that is no member.'
      finding_count=1
    fi

    while IFS= read -r crate_directory; do
      if [ ! -f "$crate_directory/moon.yml" ]; then
        echo "[DEPENDENCIES] the crate $crate_directory has no moon.yml, so moon knows nothing of it."
        finding_count=1
        continue
      fi
      awk --assign crate_directory="$crate_directory" --field-separator '\t' '
        $1 == crate_directory && $2 == "production" {
          production[$3] = 1
        }
        $1 == crate_directory && $2 == "development" {
          development[$3] = 1
        }
        END {
          for (directory in production) {
            print directory "\tproduction"
          }
          for (directory in development) {
            if (!(directory in production)) {
              print directory "\tdevelopment"
            }
          }
        }
      ' dependencies > cargo_directories
      : > cargo_dependencies
      while IFS="$(printf '\t')" read -r dependency_directory scope; do
        dependency_id=$(sed --quiet "s/^id: '\(.*\)'\$/\1/p" "$dependency_directory/moon.yml")
        printf '%s\t%s\n' "$dependency_id" "$scope" >> cargo_dependencies
      done < cargo_directories
      sort --unique cargo_dependencies --output cargo_dependencies
      awk '
        function close_entry() {
          if (entry != "") {
            print entry "\t" entry_scope
          }
          entry = ""
        }
        /^dependsOn:$/ {
          inside_list = 1
          next
        }
        inside_list && /^  - \047[^\047]*\047$/ {
          close_entry()
          entry = $2
          gsub(/\047/, "", entry)
          entry_scope = "production"
          next
        }
        inside_list && /^  - id: / {
          close_entry()
          entry = $3
          gsub(/\047/, "", entry)
          entry_scope = "production"
          next
        }
        inside_list && /^    scope: / {
          entry_scope = $2
          gsub(/\047/, "", entry_scope)
          next
        }
        inside_list {
          close_entry()
          inside_list = 0
        }
        END {
          close_entry()
        }
      ' "$crate_directory/moon.yml" | sort --unique > moon_dependencies
      if ! diff cargo_dependencies moon_dependencies > difference; then
        echo "[DEPENDENCIES] the dependsOn of $crate_directory/moon.yml is not what its Cargo.toml names."
        cat difference
        finding_count=1
      fi
    done < crate_directories

    find crates -name Cargo.toml -print0 | sort --zero-terminated | xargs --null awk '
      FNR == 1 {
        section = ""
        directory = FILENAME
        sub(/\/Cargo\.toml$/, "", directory)
      }
      /^\[/ {
        section = $0
        next
      }
      (section == "[dependencies]" || section ~ /^\[target\..*\.dependencies\]$/) && /^stack_game_application[ .=]/ && /"authority"/ {
        print directory "\t" directory "/Cargo.toml names stack_game_application with its feature authority"
      }
      section == "[features]" && /stack_game_application\??\/authority/ {
        print directory "\t" "a feature of " directory "/Cargo.toml enables the feature authority of stack_game_application"
      }
    ' | sort --unique > authority_enablers
    while IFS= read -r crate_directory; do
      if grep --quiet "^tags: .*'\(client\|client_game_adapter\)'" "$crate_directory/moon.yml"; then
        printf '%s\n' "$crate_directory"
      fi
    done < crate_directories > client_directories
    if ! awk --field-separator '\t' '
      FILENAME == ARGV[1] {
        enabler_reason[$1] = $2
        next
      }
      FILENAME == ARGV[2] && $2 == "production" {
        edge_count[$1] += 1
        edges[$1, edge_count[$1]] = $3
        next
      }
      FILENAME == ARGV[3] {
        clients[$1] = 1
      }
      function trace_authority(crate, path,   index_count, dependency, dependency_path) {
        if (crate in visited) {
          return ""
        }
        visited[crate] = 1
        if (crate in enabler_reason) {
          return path " (" enabler_reason[crate] ")"
        }
        for (index_count = 1; index_count <= edge_count[crate]; index_count += 1) {
          dependency = edges[crate, index_count]
          dependency_path = trace_authority(dependency, path " -> " dependency)
          if (dependency_path != "") {
            return dependency_path
          }
        }
        return ""
      }
      END {
        refusal_count = 0
        for (client in clients) {
          delete visited
          traced_path = trace_authority(client, client)
          if (traced_path != "") {
            print "[DEPENDENCIES] the client crate " client " compiles the authority of the game. The feature authority of stack_game_application reaches it through " traced_path "."
            refusal_count += 1
          }
        }
        exit (refusal_count > 0)
      }
    ' authority_enablers dependencies client_directories; then
      echo '[DEPENDENCIES] no crate tagged client or client_game_adapter may enable the feature authority of stack_game_application, directly or through another crate, because no program of a client may hold a rule of the authority.'
      finding_count=1
    fi

    if [ "$finding_count" != 0 ]; then
      echo '[DEPENDENCIES] a line marked < is a crate the Cargo.toml names that the dependsOn leaves out, so moon knows nothing of that edge: neither its cache nor the constraints of .moon/workspace.yml see it.'
      echo '[DEPENDENCIES] a line marked > is a crate the dependsOn names that the Cargo.toml never names.'
      exit 1
    fi
    crate_count=$(wc --lines < crate_directories)
    echo "[DEPENDENCIES] the dependsOn of all $crate_count crates names exactly the crates its Cargo.toml names."
    client_count=$(wc --lines < client_directories)
    echo "[DEPENDENCIES] none of the $client_count crates tagged client or client_game_adapter enables the feature authority of stack_game_application."
  '';
  installPhase = ''
    touch "$out"
  '';
}
