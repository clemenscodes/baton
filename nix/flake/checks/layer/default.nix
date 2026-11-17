{
  dependencyTable,
  gawk,
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "layer";
  src = source;
  nativeBuildInputs = [
    dependencyTable
    gawk
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    echo '[LAYER] layer reads the production code of every source under `crates` and every manifest.'
    tool_vocabulary --tokens ./crates > tokens
    dependency_table > dependencies

    finding_count=0
    if ! awk --field-separator '\t' '
      function select_layers(path) {
        if (path ~ /^\.\/crates\/(stack|products)\/[a-z_]+\/core\//) {
          return " DomainLayer "
        }
        if (path ~ /^\.\/crates\/(stack|products)\/[a-z_]+\/application\//) {
          return " ApplicationLayer "
        }
        if (path ~ /^\.\/crates\/(stack|products)\/[a-z_]+\/adapter\//) {
          return " InfrastructureLayer "
        }
        if (path ~ /^\.\/crates\/(stack|products)\/[a-z_]+\/presentation\//) {
          return " PresentationLayer "
        }
        if (path ~ /^\.\/crates\/gates\/(coverage|vocabulary\/language)\/core\//) {
          return " DomainLayer "
        }
        if (path ~ /^\.\/crates\/gates\/(coverage|vocabulary\/language)\/application\//) {
          return " ApplicationLayer "
        }
        if (path ~ /^\.\/crates\/gates\/(coverage|vocabulary\/language)\/adapter\//) {
          return " InfrastructureLayer "
        }
        if (path ~ /^\.\/crates\/gates\/(coverage|vocabulary\/language)\/command_line\//) {
          return " PresentationLayer "
        }
        if (path ~ /^\.\/crates\/(stack|gates|gates\/vocabulary)\/[a-z_]*\/src\/domain\//) {
          return " DomainLayer "
        }
        if (path ~ /^\.\/crates\/(stack|gates|gates\/vocabulary)\/[a-z_]*\/src\/application\//) {
          return " ApplicationLayer "
        }
        if (path ~ /^\.\/crates\/(stack|gates|gates\/vocabulary)\/[a-z_]*\/src\/infrastructure\//) {
          return " InfrastructureLayer "
        }
        if (path ~ /^\.\/crates\/(stack|gates|gates\/vocabulary)\/[a-z_]*\/src\/presentation\//) {
          return " PresentationLayer "
        }
        return ""
      }
      {
        if (second_text == "type" && first_text == "Layer" && $5 == "=") {
          inside_binding = 1
        } else if (inside_binding && $5 == ";") {
          inside_binding = 0
          if ($1 !~ /^\.\/crates\/stack\/domain_driven_design\//) {
            layers = select_layers($1)
            if (layers == "") {
              printf "[LAYER] %s:%s:%s: no layer of docs/ARCHITECTURE.md covers this path, so `%s` cannot be judged.\n", $1, $2, $3, first_text
              finding_count += 1
            } else if (index(layers, " " first_text " ") == 0) {
              printf "[LAYER] %s:%s:%s: `%s` is no layer of this path, whose layers are%s.\n", $1, $2, $3, first_text, substr(layers, 1, length(layers) - 1)
              finding_count += 1
            }
          }
        }
        second_text = first_text
        first_text = $5
      }
      END {
        if (finding_count > 0) {
          printf "[LAYER] %d types name a layer the table of docs/ARCHITECTURE.md does not give their directory.\n", finding_count
          exit 1
        }
      }
    ' tokens; then
      finding_count=1
    fi

    if ! awk --field-separator '\t' '
      function report_dependency(reason) {
        printf "[LAYER] %s/Cargo.toml depends on %s, and %s\n", $1, $3, reason
        finding_count += 1
      }
      $2 == "production" {
        crate_directory = $1
        dependency = $3
        split(crate_directory, crate_segments, "/")
        family_core = "crates/" crate_segments[2] "/" crate_segments[3] "/core/"
        if (crate_directory ~ /^crates\/(stack|products)\/[a-z_]+\/core\// && dependency !~ /^crates\/stack\/domain_driven_design\/core$/ && index(dependency, family_core) != 1 && !(crate_directory ~ /^crates\/products\// && dependency ~ /^crates\/stack\/[a-z_]+\/core\//)) {
          report_dependency("a crate of a core reaches only the crates of its own core, stack_domain_driven_design_core and, for a product, the cores of the stack.")
        } else if (crate_directory ~ /^crates\/stack\// && dependency ~ /^crates\/products\//) {
          report_dependency("the stack knows no product.")
        } else if (crate_directory ~ /\/presentation\// && dependency ~ /\/core\// && dependency !~ /^crates\/stack\/domain_driven_design\/core$/) {
          report_dependency("a presentation reaches only the application of its family and the adapters it composes, never a crate of a core.")
        } else if (crate_directory ~ /^crates\/(gates\/coverage\/[a-z_\/]*|gates\/vocabulary\/language\/[a-z_\/]*|gates\/vocabulary\/gate|stack\/domain_driven_design\/core)$/ && dependency !~ /^crates\/(stack\/domain_driven_design\/core|gates\/vocabulary\/language\/[a-z_\/]*)$/ && (crate_directory !~ /^crates\/gates\/coverage\// || dependency !~ /^crates\/gates\/coverage\//)) {
          report_dependency("a tool stands outside baton.")
        }
      }
      END {
        if (finding_count > 0) {
          printf "[LAYER] %d dependencies point outward, and an inner layer never names an outer one.\n", finding_count
          exit 1
        }
      }
    ' dependencies; then
      finding_count=1
    fi

    find crates -name Cargo.toml -printf '%h\n' | sort > crate_directories
    while IFS= read -r crate_directory; do
      if [ ! -f "$crate_directory/moon.yml" ]; then
        continue
      fi
      program_count=0
      if [ -f "$crate_directory/src/main.rs" ]; then
        program_count=1
      fi
      tag_text=$(sed --quiet 's/^tags: \[\(.*\)\]$/\1/p' "$crate_directory/moon.yml" | tr --delete "' ")
      printf '%s\t%s\t%s\n' "$crate_directory" "$program_count" "$tag_text"
    done < crate_directories > crate_tags
    if ! awk --field-separator '\t' '
      function select_crate_layer(directory, program_count) {
        if (directory ~ /^crates\/(gates\/|stack\/domain_driven_design\/core$)/ && directory !~ /^crates\/gates\/(coverage|vocabulary\/language)\//) {
          return ""
        }
        if (directory ~ /\/adapter\//) {
          return "infrastructure"
        }
        if (program_count == 1) {
          return "presentation"
        }
        if (directory ~ /\/presentation(\/|$)/) {
          return "library"
        }
        if (directory ~ /\/core(\/|$)/) {
          return "domain"
        }
        if (directory ~ /\/application(\/|$)/) {
          return "application"
        }
        return "none"
      }
      {
        layer = select_crate_layer($1, $2)
        tag_count = split($3, tags, ",")
        named_layers = ""
        for (tag_index = 1; tag_index <= tag_count; tag_index += 1) {
          if (tags[tag_index] ~ /^(domain|application|infrastructure|presentation)$/) {
            named_layers = named_layers " " tags[tag_index]
          }
        }
        if (layer == "none") {
          printf "[LAYER] %s: no layer of docs/ARCHITECTURE.md covers this crate, so its tag of a layer cannot be judged.\n", $1
          finding_count += 1
        } else if (layer == "library") {
          printf "[LAYER] %s is a library under a presentation, and a library is never a presentation: it is an adapter under adapter.\n", $1
          finding_count += 1
        } else if (named_layers != (layer == "" ? "" : " " layer)) {
          printf "[LAYER] %s/moon.yml names the layer tags%s, and the path of the crate gives it %s.\n", $1, (named_layers == "" ? " none" : named_layers), (layer == "" ? "no layer tag" : "the tag " layer)
          finding_count += 1
        }
      }
      END {
        if (finding_count > 0) {
          printf "[LAYER] %d crates carry a tag of a layer their path does not give them, and moon enforces the layers through these tags.\n", finding_count
          exit 1
        }
      }
    ' crate_tags; then
      finding_count=1
    fi

    if [ "$finding_count" != 0 ]; then
      exit 1
    fi
    echo "[LAYER] every type names a layer of its directory, and no dependency points outward."
  '';
  installPhase = ''
    touch "$out"
  '';
}
