# The word

A word is checked against a language in its own case and in its own plural, so
`Entry` covers `entries`, `Name` covers `NAMES` and `Resolve` covers `resolves`.
`Word::compose_stems` is where that is decided, by the endings English gives a plural, and
it folds nothing else together. Only the last word of a term may stand in the plural,
so `Treasure Chest` covers `treasure_chests` and not `treasures_chest`.
