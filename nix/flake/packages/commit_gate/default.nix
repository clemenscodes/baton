{
  coreutils,
  git,
  gnugrep,
  writeShellApplication,
}:
writeShellApplication {
  name = "commit_gate";
  runtimeInputs = [
    coreutils
    git
    gnugrep
  ];
  text = ''
    message_file="$1"
    pattern='^(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test)(\([a-z0-9_-]+\))?!?: [^ ].*$'
    message="$(git stripspace --strip-comments < "$message_file")"
    subject="$(head --lines=1 <<<"$message")"
    refusal=0
    if ! grep --quiet --extended-regexp "$pattern" <<<"$subject"; then
      echo "[COMMIT] the subject is not a conventional commit: $subject"
      refusal=1
    fi
    trailers="$(git interpret-trailers --parse <<<"$message")"
    if [ -n "$trailers" ]; then
      echo "[COMMIT] the message must not contain a trailer, but has: $trailers"
      refusal=1
    fi
    exit "$refusal"
  '';
}
