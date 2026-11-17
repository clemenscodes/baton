{
  gawk,
  stdenvNoCC,
  source,
  vocabulary,
}:
stdenvNoCC.mkDerivation {
  name = "signature";
  src = source;
  nativeBuildInputs = [
    gawk
    vocabulary
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    echo '[SIGNATURE] signature reads the production code of every source under `crates`.'
    tool_vocabulary --tokens ./crates > tokens

    if ! awk --field-separator '\t' '
      function report_token(message) {
        printf "[SIGNATURE] %s:%s:%s: %s\n", $1, $2, $3, message
        finding_count += 1
      }
      function report_function(message) {
        printf "[SIGNATURE] %s:%s:%s: `fn %s` %s\n", function_path, function_line, function_column, function_name, message
        finding_count += 1
      }
      function finish_function() {
        if (function_declared) {
          if (return_text ~ /^(: : )?([a-z_]+ : : )*Result( <|$| $)/ || return_text ~ /Output = .*Result/) {
            if (function_name !~ /^try_/) {
              report_function("returns a Result and does not state so. Name it try_ and the caller reads the failure before the body.")
            }
          }
          if (return_text == "usize " && parameter_text == "& self " && function_name ~ /s$/ && function_name !~ /^count/) {
            report_function("is named for many things and returns one number. A count is named count, and the plural is the getter of the things.")
          }
        }
        function_state = 0
      }
      function start_function() {
        function_name = $5
        function_declared = ($4 == "declaration")
        function_path = $1
        function_line = $2
        function_column = $3
        function_state = 1
        angle_depth = 0
        parenthesis_depth = 0
        parameter_text = ""
        return_text = ""
      }
      function read_function(text) {
        if (function_state == 1) {
          if (parenthesis_depth == 0) {
            if (text == "<") {
              angle_depth += 1
            } else if (text == ">" && previous_text != "-") {
              angle_depth -= 1
            } else if (text == "(" && angle_depth == 0) {
              parenthesis_depth = 1
            }
            return
          }
          if (text == "(") {
            parenthesis_depth += 1
          } else if (text == ")") {
            parenthesis_depth -= 1
            if (parenthesis_depth == 0) {
              function_state = 2
              return
            }
          }
          parameter_text = parameter_text text " "
          return
        }
        if (function_state == 2) {
          if (text == "-") {
            function_state = 3
          } else if (text == "{" || text == ";" || text == "where") {
            finish_function()
          }
          return
        }
        if (function_state == 3) {
          function_state = 4
          return
        }
        if (text == "{" || text == ";" || text == "where") {
          finish_function()
          return
        }
        return_text = return_text text " "
      }
      function read_conversion(text) {
        if (text == "impl" && !implementation_header && conversion_depth == 0) {
          implementation_header = 1
          header_bracket_depth = 0
          header_conversion = 0
          header_for = 0
          return
        }
        if (implementation_header) {
          if (text == "<" || text == "[" || text == "(") {
            header_bracket_depth += 1
          } else if ((text == ">" && previous_text != "-") || text == "]" || text == ")") {
            header_bracket_depth -= 1
            if (header_bracket_depth < 0) {
              implementation_header = 0
            }
          } else if (header_bracket_depth == 0 && (text == "From" || text == "TryFrom") && !header_for) {
            header_conversion = 1
          } else if (header_bracket_depth == 0 && text == "for") {
            header_for = 1
          } else if (header_bracket_depth == 0 && text == "{") {
            implementation_header = 0
            if (header_conversion && header_for) {
              conversion_depth = brace_depth + 1
            }
          } else if (header_bracket_depth == 0 && text == ";") {
            implementation_header = 0
          }
        }
      }
      BEGIN {
        numeric_pattern = "^(usize|isize|u8|u16|u32|u64|u128|i8|i16|i32|i64|i128|f32|f64|char|bool)$"
      }
      {
        text = $5
        read_conversion(text)
        if (text == "{") {
          brace_depth += 1
        } else if (text == "}") {
          if (brace_depth == conversion_depth) {
            conversion_depth = 0
          }
          brace_depth -= 1
        }
        if (text == "use" && $4 == "identifier") {
          use_statement = 1
        }
        if (use_statement && text == ";") {
          use_statement = 0
        }
        if (previous_text == "as" && !use_statement && conversion_depth == 0 && previous_path == $1) {
          if (text ~ numeric_pattern || text == "*" || text == "_") {
            report_token("an `as` cast outside the body of a From or a TryFrom. Use From, Into or TryFrom.")
          }
        }
        if ($4 == "literal") {
          if (text ~ /(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)$/ || text ~ /^[0-9][0-9_]*(\.[0-9_]*)?(e[0-9+-]*)?(f32|f64)$/) {
            report_token("the number `" text "` wears a type. Annotate the variable instead.")
          }
          if (text ~ /_/) {
            report_token("the number `" text "` is broken up by underscores. A number is one number.")
          }
          hexadecimal_digits = text
          sub(/(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)$/, "", hexadecimal_digits)
          if (hexadecimal_digits ~ /^0x[0-9A-F_]*[a-f]/) {
            report_token("the number `" text "` is hexadecimal in lower case. Hexadecimal this repository forms is in upper case.")
          }
        }
        if (function_state > 0) {
          read_function(text)
        } else if (previous_text == "fn" && previous_path == $1 && text ~ /^[A-Za-z_]/) {
          start_function()
        }
        previous_text = text
        previous_path = $1
      }
      END {
        if (finding_count > 0) {
          printf "[SIGNATURE] %d signatures and numbers break a rule of docs/STYLE.md.\n", finding_count
          exit 1
        }
      }
    ' tokens; then
      exit 1
    fi
    echo "[SIGNATURE] every signature, every cast and every number of the production code keeps to docs/STYLE.md."
  '';
  installPhase = ''
    touch "$out"
  '';
}
