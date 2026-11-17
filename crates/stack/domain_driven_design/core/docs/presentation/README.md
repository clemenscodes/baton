# Presentation

A component of the presentation publishes a `View`, the contract its callers build and pass,
and keeps its `Model` to itself. A model is converted from its view, `Model: From<&View>`, and
a `Presentation` names the `Model` it presents. View, model and presentation all belong to the
presentation layer.
