{
  gawk,
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "interface";
  src = source;
  nativeBuildInputs = [
    gawk
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    echo '[INTERFACE] interface reads the production code of every source under `crates`.'
    tool_vocabulary --tokens ./crates > tokens

    if ! awk --field-separator '\t' '
      function report_statement(message) {
        printf "[INTERFACE] %s:%s:%s: `%s` %s\n", statement_path, statement_line, statement_column, statement_text, message
        finding_count += 1
      }
      function finish_statement(   depth_text, depth, child_text, item_text, item_list, item_count, position, item) {
        if (statement_text ~ /(^|::|\{|,)\*/) {
          report_statement("is a glob import, which flattens the module of somebody else into this one.")
        }
        if (statement_public) {
          depth_text = statement_text
          depth = gsub(/::/, "::", depth_text)
          if (depth != 1) {
            report_statement("reaches past the own child of this module.")
            return
          }
          child_text = statement_text
          sub(/::.*$/, "", child_text)
          public_count += 1
          public_paths[public_count] = statement_path
          public_lines[public_count] = statement_line
          public_columns[public_count] = statement_column
          public_texts[public_count] = statement_text
          public_children[public_count] = child_text
          item_text = statement_text
          sub(/^.*::/, "", item_text)
          gsub(/[{}]/, "", item_text)
          item_count = split(item_text, item_list, ",")
          for (position = 1; position <= item_count; position += 1) {
            item = item_list[position]
            sub(/ as .*$/, "", item)
            if (item != "") {
              published[statement_path, item] = 1
              export_count += 1
              export_paths[export_count] = statement_path
              export_lines[export_count] = statement_line
              export_columns[export_count] = statement_column
              export_children[export_count] = child_text
              export_items[export_count] = item
            }
          }
        }
      }
      {
        text = $5
        if (first_text == "mod" && first_kind == "identifier" && $4 == "declaration") {
          declared[$1, text] = 1
        }
        if (inside_statement) {
          if (text == ";") {
            inside_statement = 0
            finish_statement()
          } else if (text == "as") {
            statement_text = statement_text " as "
          } else {
            statement_text = statement_text text
          }
        } else if (text == "use" && $4 == "identifier") {
          inside_statement = 1
          statement_public = (first_text == "pub" || (first_text == ")" && fourth_text == "pub"))
          statement_path = $1
          statement_line = $2
          statement_column = $3
          statement_text = ""
        }
        fourth_text = third_text
        third_text = second_text
        second_text = first_text
        first_text = text
        first_kind = $4
      }
      END {
        for (position = 1; position <= public_count; position += 1) {
          if (!((public_paths[position], public_children[position]) in declared)) {
            printf "[INTERFACE] %s:%s:%s: `%s` passes on what `%s` holds, and this file declares no child module of that name. A crate publishes types of its own, and an application translates the values of its core into them instead of handing them on.\n", public_paths[position], public_lines[position], public_columns[position], public_texts[position], public_children[position]
            finding_count += 1
          }
        }
        for (position = 1; position <= export_count; position += 1) {
          path = export_paths[position]
          if (path !~ /\/lib\.rs$/) {
            directory = path
            sub(/[^\/]*$/, "", directory)
            module = directory export_children[position] "/mod.rs"
            if ((module, export_items[position]) in published) {
              printf "[INTERFACE] %s:%s:%s: publishes `%s`, which %s already published. Nothing publishes twice.\n", path, export_lines[position], export_columns[position], export_items[position], module
              finding_count += 1
            }
          }
        }
        if (finding_count > 0) {
          printf "[INTERFACE] %d imports break the interface of a layer: a layer publishes its own children once, passes on nothing another crate or module holds, and no import is a glob.\n", finding_count
          exit 1
        }
      }
    ' tokens; then
      exit 1
    fi
    echo "[INTERFACE] no production code imports a glob, reaches past its own child, passes on what another crate or module holds or publishes twice."
  '';
  installPhase = ''
    touch "$out"
  '';
}
