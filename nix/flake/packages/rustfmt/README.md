# rustfmt

The formatter of Rust, `rustfmt` and `cargo-fmt`, built from the source of the toolchain of
`rust-toolchain.toml` with one patch, `assignment_rhs_break.patch`. The patch adds the option
`assignment_rhs_break`, and with that option at its default, `"Heuristic"`, this rustfmt formats
every file exactly as the rustfmt of the toolchain does.

With `assignment_rhs_break = "TrailingDelimiter"`, the right-hand side of a `let`, an assignment,
a `const` or a `static` that ends in a call, a method call, a tuple, a struct literal, an array or
a macro call stays on the line of its left-hand side and breaks inside its trailing delimiter,
where stock rustfmt would move it to the next line. The type of a struct field, a type alias or a
foreign `static` that ends in generic arguments, a tuple, a slice, a function pointer or a trait
object does the same after its `:` or `=`:

```rust
let embodiment = Embodiment::try_from((
    content,
    character,
    session,
    current_instant,
))?;
```

instead of

```rust
let embodiment =
    Embodiment::try_from((content, character, session, current_instant))?;
```

and

```rust
pub(super) package_supply: &'onset PackageSupply<
    ContentType,
    AssetsType,
    TypefaceType,
    TypesetterType,
>,
```

instead of

```rust
pub(super) package_supply:
    &'onset PackageSupply<ContentType, AssetsType, TypefaceType, TypesetterType>,
```

The type of a `let`, a `const`, a `static` and a parameter of a function never moves to the next
line in rustfmt, so it already breaks inside its delimiter without the option.

A call without arguments still moves to the next line, because breaking inside its empty
parentheses would leave `(` and `)` on two lines with nothing between them.

With `assignment_rhs_break = "SameLine"`, no line ends right after an `=`, a `=>` or a `:`. The
right-hand side of a `let`, an assignment, a `const` or a `static`, the type of a struct field, a
type alias or a `const`, the bounds of a where clause or of a trait, the value of a field of a
struct literal or a struct pattern, and the body of a match arm all start on the line of their
token, and the block of a match arm opens on the line of `=>` even behind a guard of several
lines. Each breaks only inside itself: before a binary operator, inside a delimiter, at a link of a
chain or in the block of a match arm. Where one cannot break at all, such as a long string literal,
a call without arguments or a long path, its line grows past `max_width`. rustfmt then tries it
again with one, two, four and more columns beyond the line until it fits, so the line grows by
about as little as it can, and a rewrite that breaks inside an empty pair of delimiters counts as
one that does not fit:

```rust
let maybe_moment_milliseconds = playback_milliseconds
    .checked_rem(duration_milliseconds_value);
let expectation = "detail level 1 is 4 by 4 pixels, but half of the level before it is 2 by 2";
```

instead of

```rust
let maybe_moment_milliseconds =
    playback_milliseconds.checked_rem(duration_milliseconds_value);
let expectation =
    "detail level 1 is 4 by 4 pixels, but half of the level before it is 2 by 2";
```

The left-hand side is formatted before its right-hand side and does not break to make room for
it, so where the left-hand side fills its line, as a long type of a `let` does, the right-hand side
lets that line grow past `max_width` even where breaking the left-hand side would have kept it
within.

rustfmt is a tool of the compiler: it links against the crates of rustc, so it builds only with
the very compiler it formats for. The package therefore builds it from the source tarball of that
release, `rustc-<version>-src.tar.gz`, whose dependencies are vendored, with the toolchain of
`rust-overlay` and its component `rustc-dev`, as nixpkgs builds its own rustfmt. The version comes
from `rust-toolchain.toml`, and the hash of the tarball pins that version, so a new toolchain needs
a new hash and a patch that still applies. The tests of rustfmt run under nextest, and every test
ends after 20 seconds.

The overlay `nix/flake/overlays` holds it as `rust-formatter`. The dev shell puts it before the
toolchain on `PATH` and names it in `RUSTFMT`, and the check `rustfmt` formats with it, both with
the options of `rustfmt.toml` at the root.

Its `rustfmt --version` prints `rustfmt 1.9.0` without the hash and the date of the commit, which
the build of rustfmt reads from git and the tarball does not carry.

## A new release of Rust

The patch carries over to a new release of `rust-toolchain.toml` like this:

1. Replace the hash of the tarball in `default.nix` with the one `nix store prefetch-file
   https://static.rust-lang.org/dist/rustc-<version>-src.tar.gz` prints.
2. Unpack `src/tools/rustfmt` of that tarball twice, apply the patch to one copy with
   `patch --strip=1`, and fix the hunks that no longer apply, in `src/expr.rs` around
   `choose_rhs` and `rewrite_field`, in `src/items.rs` where the kinds of types reach
   `rewrite_assign_rhs` and in `rewrite_static`, in `src/matches.rs` in `rewrite_match_body`, in
   `src/patterns.rs` where a `PatField` is rewritten, in `src/config/options.rs` and
   `src/config/mod.rs` beside `overflow_delimited_expr`, and in `Configurations.md`.
3. Write the patch anew with `git diff --no-index --no-prefix a/src/tools/rustfmt
   b/src/tools/rustfmt` from a directory that holds the unpatched copy at `a/src/tools/rustfmt` and
   the patched one at `b/src/tools/rustfmt`, so its paths are the ones the tarball has.
4. Build the package. Its build runs the tests of rustfmt, among them the pairs
   `tests/source/configs/assignment_rhs_break` and `tests/target/configs/assignment_rhs_break`.
