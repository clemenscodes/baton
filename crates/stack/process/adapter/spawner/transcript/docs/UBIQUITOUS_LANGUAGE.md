# The ubiquitous language

| Word | What it means |
|---|---|
| Completion Queue | The completions a recording spawner still hands out, in order. |
| End Request | A request to end a recorded program, kept with the invocation of that program. |
| Maybe Completion | The next completion of a recording spawner, where one is left, and none where the program is to be missing. |
| Poison | The lock of a thread that panicked while it held it, whose record is still read. |
| Record Invocation | To record an invocation and to take the next completion. |
| Recorded Program | The running program of the recording spawner, which completes at once with its completion. |
| Spawner | The transcript adapter of the port Spawner, which hands out the completions it was made with in order and succeeds once they are spent. |
| Stack Process Adapter Spawner Transcript | The crate stack_process_adapter_spawner_transcript, the adapter of the spawner that starts nothing and records every invocation, for the tests of the adapters that drive a program. |
