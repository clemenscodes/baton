{{#include ../README.md}}

## The concept

`TestBoundaries` holds a `test_boundary::TestBoundary` for every test module of a source, which opens on the line of its attribute `#[cfg(test)]` and closes on the line of the brace that closes the module. Each test boundary and the test boundaries as a whole are a `Specification` of a span: they accept a span outside every test module and refuse one inside a test module, combined with `&`, and only a region whose span they accept is judged.
