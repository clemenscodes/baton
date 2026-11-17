# The outcome

The exit status is one when anything was an error and zero when nothing was.

The judgment and the edition write every error to the error stream, flush their streams, and
yield `Outcome::Error`, because the status of a process that did not do its work is the same
whichever of them stopped it.
