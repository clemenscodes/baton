# shell

The check `shell`. It runs shellcheck over every script under `.claude` and `.vscode`, a file whose
first line is a shebang of a shell, and refuses what shellcheck refuses, and a tree of no script,
because then the filter of the paths is wrong.
