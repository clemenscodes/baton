# The vocabulary

The port `vocabulary::Vocabulary` is how the application reaches the vocabulary it judges with. It
lists the contexts as a `vocabulary::ContextList` and the foreign languages as a
`vocabulary::ForeignLanguageList`, or the `Error` one of them cannot be formed with. The program of
the gate fills the port with its contexts, so the application knows no context itself.

The gate and the press compose their languages from the two lists when they are made, with
`Gate::try_from` and `Press::try_from`: the language of Rust, the test words, and every context
joined with the foreign languages it names. A vocabulary that cannot be listed, or languages that
cannot be composed, refuse the service with `Error::Vocabulary` or the error of the port.
