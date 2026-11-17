#!/usr/bin/env bash
payload=$(cat)
if ! grep --quiet --extended-regexp 'moon[^|;&"]* --force' <<<"$payload"; then
  exit 0
fi
echo 'moon runs with its cache here: never pass --force. A cached result is a result. If the cache looks wrong, change one file and watch the task run again.' >&2
exit 2
