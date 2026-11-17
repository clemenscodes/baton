{
  findutils,
  gawk,
  stdenvNoCC,
  names,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "pipeline";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  inherit names;
  passAsFile = [ "names" ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    select_checks() {
      grep --only-matching 'checks\.x86_64-linux\.[^ '"'"']*' "$1" | sed 's|checks\.x86_64-linux\.||'
    }

    select_checks moon.yml > moon_checks
    select_checks .moon/tasks/crate.yml > crate_pattern
    find crates -name moon.yml | sort > projects
    while IFS= read -r project; do
      project_id=$(sed --quiet "s/^id: '\(.*\)'\$/\1/p" "$project")
      if grep --quiet "^tags: \[.*'crate'.*\]\$" "$project"; then
        sed "s|\$project|$project_id|" crate_pattern >> moon_checks
      fi
      if grep --quiet 'checks\.x86_64-linux\.' "$project"; then
        select_checks "$project" | sed "s|\$project|$project_id|" >> moon_checks
      fi
    done < projects
    sort --unique moon_checks --output moon_checks
    sort "$namesPath" > flake_checks

    finding_count=0
    if ! diff flake_checks moon_checks > difference; then
      echo '[PIPELINE] the flake and moon do not run the same checks.'
      cat difference
      echo '[PIPELINE] a line marked < is a check of the flake no task of moon builds, a line marked > a task naming a check the flake does not have.'
      finding_count=1
    fi

    if ! awk '
      /^  [a-zA-Z][a-zA-Z0-9\/._-]*:$/ {
        task = $1
        sub(/:$/, "", task)
        next
      }
      task != "ci" && /checks\.x86_64-linux\./ {
        tasks[task] = 1
      }
      task == "ci" && /^      - / {
        dependency = $2
        gsub(/\047/, "", dependency)
        ci_tasks[dependency] = 1
      }
      END {
        for (task in tasks) {
          if (!(task in ci_tasks)) {
            printf "[PIPELINE] the task `%s` of moon.yml is not in its `ci`, so that task never reaches it.\n", task
            finding_count += 1
          }
        }
        exit finding_count > 0
      }
    ' moon.yml; then
      finding_count=1
    fi

    awk '
      /^  [a-zA-Z][a-zA-Z0-9\/._-]*:$/ {
        check = ""
        next
      }
      /checks\.x86_64-linux\./ {
        match($0, /checks\.x86_64-linux\.[a-zA-Z0-9_-]+/)
        check = substr($0, RSTART + 20, RLENGTH - 20)
        next
      }
      check != "" && /^      - / {
        entry = $2
        gsub(/\047/, "", entry)
        sub(/^\//, "", entry)
        sub(/[\/*].*$/, "", entry)
        printf "%s\t%s\n", check, entry
      }
    ' moon.yml | sort --unique > task_inputs

    awk '
      /sourceTree "[^"]*"/ {
        match($0, /sourceTree "[^"]*"/)
        check = substr($0, RSTART + 12, RLENGTH - 13)
        remainder = substr($0, RSTART + RLENGTH)
        while (match(remainder, /"[^"]*"/)) {
          printf "%s\t%s\n", check, substr(remainder, RSTART + 1, RLENGTH - 2)
          remainder = substr(remainder, RSTART + RLENGTH)
        }
        inside_list = (remainder !~ /\]/ && $0 !~ /\]/)
        next
      }
      inside_list {
        remainder = $0
        while (match(remainder, /"[^"]*"/)) {
          printf "%s\t%s\n", check, substr(remainder, RSTART + 1, RLENGTH - 2)
          remainder = substr(remainder, RSTART + RLENGTH)
        }
        if ($0 ~ /\]/) {
          inside_list = 0
        }
      }
    ' nix/flake/checks/default.nix | sort --unique > gate_sources

    source_count=0
    while IFS=$'\t' read -r check path; do
      if grep --quiet "^$check"$'\t' task_inputs; then
        source_count=$((source_count + 1))
        root=''${path%%/*}
        if ! grep --quiet --line-regexp --fixed-strings "$check"$'\t'"$root" task_inputs; then
          if ! grep --quiet --line-regexp --fixed-strings "$check"$'\t' task_inputs; then
            echo "[PIPELINE] the gate $check reads $path and its task lists no $root, so moon answers from a cache that never saw the change."
            finding_count=1
          fi
        fi
      fi
    done < gate_sources

    if [ "$finding_count" != 0 ]; then
      exit 1
    fi
    echo "[PIPELINE] the flake and moon run the same checks, and all $source_count sources of the gates are inputs of their tasks."
  '';
  installPhase = ''
    touch "$out"
  '';
}
