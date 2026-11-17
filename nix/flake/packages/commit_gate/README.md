# commit_gate

The program of the hook `commit-msg`. It reads the message of the commit being made and
refuses it where its subject is no Conventional Commit of the types `build`, `chore`, `ci`,
`docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style` and `test`, and where it carries a
trailer. So a commit that breaks the rule is never made, and no gate has to read the history
afterward.

The dev shell installs it into the configuration of the repository as the hook
`commit_gate` of the event `commit-msg`. A hook of the configuration runs beside the hooks of
`core.hooksPath`, so the hooks of the host stay as they are.

```
commit_gate .git/COMMIT_EDITMSG
```
