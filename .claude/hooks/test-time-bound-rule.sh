#!/usr/bin/env bash
payload=$(cat)
command_start='("command":[[:space:]]*"|[;&|(]|\\n|--command|timeout [0-9]+|exec)[[:space:]]*'
if ! grep --quiet --extended-regexp "${command_start}cargo( \+[a-z]+)? test( |\"|\\\\|\$)" <<<"$payload"; then
  exit 0
fi
echo 'That test command has no time bound, and a test that never ends fills the memory of the machine. Run tests with cargo nextest run --profile debug or through moon: .config/nextest.toml ends every test after 20 seconds.' >&2
exit 2
