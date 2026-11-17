#!/usr/bin/env bash
payload=$(cat)
tool_name=$(grep -o '"tool_name" *: *"[^"]*"' <<<"$payload" | head -n 1)
case "$tool_name" in
*'"Bash"') touched_text=$payload ;;
*) touched_text=$(grep -o '"file_path" *: *"[^"]*"' <<<"$payload" | head -n 1) ;;
esac
if ! grep -q 'crates/gates/vocabulary/gate/src/domain/contexts' <<<"$touched_text"; then
  exit 0
fi
rule='THE MOST IMPORTANT RULE OF THIS REPOSITORY, NOT NEGOTIABLE: it is strictly forbidden to enter a term into the vocabulary that contains no noun, such as Whole, Written, Own or Wide Whole. Every term and every identifier states WHAT the thing is, never HOW it came to be. A participle, an adjective or an adverb never stands alone, only in front of the noun it qualifies, and no word that only reports an act or a state names anything. Check every term you add or change against \"The first rule\" and the rules of naming in docs/STYLE.md. Where no noun fits, the name in the code is wrong: rename the code, never admit the word.'
printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","additionalContext":"%s"}}\n' "$rule"
