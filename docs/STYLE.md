# Style

This page states how the code of baton is written.

## The first rule

**Every identifier says WHAT it is, never HOW it came to be.** This rule stands above every
other rule of this repository, by far, and it is not negotiable.

Every identifier and every term of a vocabulary contains a noun, the thing it names. A
participle, an adjective or an adverb stands only in front of the noun it qualifies and never
alone, and a word that only reports an act or a state names nothing. `whole`, `written` and
`own` are no names, and no vocabulary under `crates/gates/vocabulary/gate/src/domain/contexts` ever admits a
term without a noun, such as `Whole`, `Written`, `Own` or `Wide Whole`. Where no noun fits,
the name in the code is wrong and the code is renamed, and the word is never admitted.

The rules of naming below state it in detail: every identifier contains a noun, no value is
named for an act, and an adjective is allowed only in front of a noun. `CLAUDE.md` repeats it
for every agent, and the hook `.claude/hooks/vocabulary-noun-rule.sh` reminds every agent of
it whenever it touches the vocabulary.

## The other rules

These rules are binding, always. They are neither reminders nor preferences. Code
that breaks one of them is not finished. Where a rule names a gate, the gate is listed in
`docs/GATES.md`.

They bind every language of this repository, not only Rust. The examples are Rust,
because most of the code is, and the section "Every language" at the end states how each
rule applies to Nix and shell. They bind code. A document is prose and is free.

Beyond the rules of naming, types and errors, the style holds that the tree follows the
domain, functions are pure and short, every external dependency is a port, state lives in
the infrastructure, and the code is built for many threads. `docs/ARCHITECTURE.md` states
the architecture these rules serve.

## Names

Every identifier is English. Every identifier states its full meaning. No
abbreviations, no single letters, no shortened forms.

```rust
struct KeyValue { }
let guest_state = guest.state();
let hardware_address = tap.address;
```

An acronym is spelled out, always, wherever it appears.

```rust
struct DomainDrivenDesign { }
```

A proper name that is only ever spelled as letters is not an acronym and stays as it
is.

The axes of a coordinate in a space are the `x_axis`, the `y_axis` and the `z_axis`. A single
letter is no word of its own, so the letter of an axis stands only in front of `axis`, and a
position is never a `p`.

A name is what a reader receives instead of a comment, so it has to suffice on its
own.

**Every identifier contains a noun, at least one.** That is true of a type, a field, a
variable, a constant, a variant of an enum, a module, a crate, a free function, and of every
identifier of the closed language, a key, a setting, a structure, an enumeration, a variant
and the name of an adapter. An identifier without a noun is valid only as a method of a type,
because there the type is the noun, so `entry.term()` and `lexer.next()` are names and a
field `wipe`, a variant `Absent` and a setting `unreadable` are not. A word that is a noun
and also an adjective or a verb counts as a noun only where the identifier means the noun,
so `boot` in `Lifetime::boot` is the boot and `call` in `Step::call` is the call.

## No value is named for an act

No variable is ever named `written`, `taken`, `waiting` or any other participle alone. A
participle appears in a name only in front of the noun it qualifies.

**Every value is a noun for what it is, and never for what an action left behind.**
A method may be a verb in the present tense, `read_file`, `write_script`,
`parse_instruction`, because a method acts. A value never acts, so it is never named
for an act that touched it. `reads`, `refuses`, `writes`, `written`, `found`,
`given`, `wanted`, `shown`, `kept` and `settled` name nothing, because each of them
states only that something happened and leaves out what the value is. The deciding
question is whether the sentence "this value is a ..." can be completed with the name
alone. Where it needs a verb to be completed, the name is wrong.

```rust
let text = fs::read_to_string(path)?;
let finding_count = findings.len();
let addresses = location.try_addresses()?;
```

**This binds every language in this repository, not only Rust.** Nix and shell are code of this repository and answer to this page.

Nothing in this repository speaks. A program does not say, tell, speak, ask or
answer. It requests and it responds. A name states a meaning, a type constrains
a value, a test proves a behavior, and a gate refuses. People speak, ask and
answer.

Every word of an identifier belongs to the language of its context, or the identifier
is wrong. Nothing is banned, because every word means what it means, and a word whose
meaning does not occur in this project is simply the wrong word for what was named.

## Domain types instead of primitives

A primitive never represents a domain concept across a function boundary or in a
struct field. If a value has a meaning, it is a named type, and that type is what is
passed, stored and returned.

```rust
fn bind(&self, port: Port) -> Listener { }
struct Endpoint { host: Hostname, port: Port }
```

The primitive appears in exactly one location, inside the type that gives it
meaning, in its constructor and its accessor.

The test is whether a caller can pass the wrong number for the concept and it still
compiles. Where it can, the type is missing.

A `bool` that names a domain state is a primitive too. It becomes a two variant
enum, `DiskPresence::{Presence, Absence}` rather than `disk_exists: bool`. A `bool` stays
where its domain is exactly the domain of `bool`, as the value of the literal `true` or
`false` of the language, and as a flag on a clap `Args` struct, which is clap's interface.

## A type admits exactly its concept

A type admits exactly the values of its concept, and a constructor that can produce a value
outside it is wrong however convenient it is. A type may admit more inside than its concept
only where no code outside the type can name its fields or its variants and its one
constructor fails on every gap. So a text of the language keeps its parts in private modules,
and `Text::try_from` is the one way to a part and forms no empty characters and no two parts
of characters side by side.

A thing knows exactly as much as its task needs, and no more. A layer does its own work,
publishes the one result of that work, and keeps everything it needs for the work to itself.
Where a thing names a type, it needs that type for its own task, or the name is in the wrong
location.

An offset is kept where an error points, as Rust points at the name that is declared twice
and at the segment of a path that names nothing. A node whose every error points at one of
its parts keeps no offset of its own.

The exception is a primitive used in arithmetic. An `Offset` or an `Address` may stay
`usize`, because that keeps the code simple and avoids useless conversions wherever
arithmetic happens. No arithmetic is ever done on a `Hostname`, so it must be a
struct.

## Wrapping results and intermediate values

The result of a computation is a named struct, even with one field. It costs nothing,
and it is what lets the value implement `From`, `Display`, `IntoIterator` and
`FromStr`. This matters most for a `Vec` and for any foreign type, where wrapping
is the only way to implement a trait at all.

## No tuples

No plain tuples, no tuple structs, no newtype wrappers. A named struct with named
fields, always.

```rust
struct GuestMemoryRange { start: u64, length: u64 }
```

There is exactly one exception. The argument of `From` or `TryFrom` may be a tuple,
because a constructor takes exactly one argument and a value built from several values
has no other way in. It is destructured in the signature, so
the parts are named where they are read.

```rust
impl<SpellingType, DefinitionType> TryFrom<(SpellingType, DefinitionType)> for Term {
    fn try_from((spelling, definition): (SpellingType, DefinitionType)) -> Result<Self, Self::Error> { }
}
```

A tuple anywhere else, in a field, a return type, a variable or any other argument,
is the mistake this rule addresses. A tuple that a std or third party API returns,
such as the pair of `.enumerate()`, is destructured into named variables on the spot
and never stored, returned or built by hand.

## Files and modules

Every file of Rust is `lib.rs`, `main.rs` or `mod.rs`, and the build script of a crate is
`build.rs` beside its `Cargo.toml`, where Cargo looks for it. There is no other name. A concept
is a directory, and the concept lives in the `mod.rs` of that directory.

`lib.rs` is a module like every `mod.rs`: the root of a crate. A crate is a module that compiles and
links on its own, so the concept a crate is lives in its `lib.rs`, and the modules of the crate are
the children of that concept. No module inside a crate repeats the name of the crate.

Two siblings whose names begin with the same word are two parts of one concept, and that concept
is missing as their parent (owner, 2026-11-14). So `session_key` and `session_request` beside each
other are wrong; they are `session/key` and `session/request`. The same holds for crates: two
crates beside each other whose names begin with the same word stand in a directory named for that
word. Where a sibling already is that concept, as `frame` beside `frame_instant`, the others become
its children: a module becomes its child module, and a crate becomes a module of that crate, because
a crate never contains another crate. Where the shared
word is no noun, as `placed` in `placed_decal_sets`, the names are wrong and change, never the
rule: a directory is always named for a noun.

A `mod.rs` defines exactly one concept. One struct, or one enum, or one trait.
Its inherent impls, its trait impls and its tests belong in that same file. A
second struct means a second directory.

Every line of code about one field of a struct belongs in the module of the concept of that
field, and the module of the struct only composes its fields. A field with rules of its own,
such as a list that has to lie within a region, is a concept of its own with a module of its
own, never a bare `Vec` whose rules the owner of the field checks.

If two sibling modules share the first word of their name, that word is a module
and both belong inside it. Apply the rule again inside the result.

Every child of a directory is the same kind of thing as its siblings. Name what the
children are in a single noun phrase. If the sentence needs "and also", something
belongs one level down, inside a container named for what it is.

There is no module called `util`, `utils`, `helpers`, `common`, `misc` or
`shared`. Those names mark a concept that was not found.

## Crates

The code is split into many small crates, and new ones are created often. A concept
that more than one crate needs becomes a crate of its own, independent and isolated,
with one purpose and no dependency that purpose does not need. There are never too many
crates, only too few. A crate is its `src/main.rs` or its `src/lib.rs`, and a crate never
contains another crate, so crates that belong together are directories side by side below a
directory that is no crate, as the adapters of a port lie side by side below the directory of
that port, which `docs/ARCHITECTURE.md` states.

A crate is named for its full path under `crates`, every segment of the path joined by an
underscore, and leaves out none of them: `crates/stack/lock/adapter/pause/standard_library` is
`stack_lock_adapter_pause_standard_library`, `crates/stack/lock/core/graphics/lock` is `stack_lock_core_graphics_lock` and
`crates/gates/vocabulary/gate` is `gates_vocabulary_gate`. Its project of moon has the same name as its `id`.
The gate `crate-name` refuses every other name.

The root of a crate holds the directories `docs`, `assets`, `nix`, `src` and `tests` and no
other directory, beside its files such as `Cargo.toml`, `moon.yml`, `book.toml`, `README.md` and
`build.rs`. Data the crate reads, such as shaders, pages or scenarios, lies under `assets`.

A dependency is added with `cargo add`, which writes the manifest the way Cargo writes it,
and never by editing `Cargo.toml` by hand.

## The tree follows the domain

A subconcept of a concept is a directory inside that concept. This binds the directories
under `crates`, the crates, the modules inside a crate and every directory of the repository
alike. Where a concept is used by exactly one other concept, it lives inside that one, as
the section "Finding the module tree" finds it.

A directory, a crate and a module are named for what they are in the domain, a thing or a
concept, and never for an activity. `strip`, `parse`, `unpack`, `load` and `sync` are
activities and no names. `binary_stripper`, `overlay`, `entry` and `skill_tree` are names. An
agent noun such as `stripper` or `unpacker` names a thing that does the activity, so it is a
name.

The rule holds in every layer. A module of the infrastructure is named for the adapter it
is, such as `postgres` or `file_system`, and a module of the presentation for what it
presents, such as `account_page`, never for the request it serves.

Whether a word is an activity or a thing is a question no search answers. So every name of
a module, a crate and a directory is proposed and approved before it is created.

## Layers publish, modules do not

A layer publishes the types that leave it, one `pub(crate) use` per type, in the
`mod.rs` of the layer and nowhere else. That is the interface of the layer, and it is
the only re-export there is.

```rust
mod error;
mod language;
mod port;

pub(crate) use error::Error;
pub(crate) use language::Language;
pub(crate) use port::Reader;
```

Every other module keeps its children to itself, and when it does publish, it reaches
no further than its own child. A re-export that passes a type up through one module
after another is what this forbids, because after two of them nobody can tell where a
type lives, and after four the crate has a flat surface that a caller reaches with a
glob. There is no glob import anywhere, and no prelude.

**The layer is bound by that rule as well and reaches no further than its own child.**
`pub use inventory::items::item::Item` names where `Item` lives, and then hands the
caller `crate::Item`, which names nothing. The section after the next one sets out
what the layer publishes instead.

`lib.rs` follows the same rule as every other module: it publishes its own children and
reaches no further, and it passes no type of a grandchild up. What a library publishes leaves
the crate, so it publishes with `pub` rather than `pub(crate)`.

No crate passes an item of another crate along, the application included. An application
encapsulates its core: a presentation receives from the core only types of the application itself,
which translate the values of the core, an anti-corruption layer, so the core can change without
breaking a presentation. A `pub use` of an item of another crate is refused everywhere, as the owner
decided on 2026-11-13, and the gate `interface` refuses it.

A private `use` is ordinary Rust and is used freely. What is forbidden is passing
somebody else's item along, not importing one for the work of this module.

## Visibility belongs on the type, not on the module

A type receives the visibility it needs. A `pub(crate)` type inside a private module
is ordinary and correct, so a caller receives it, keeps it in a variable and calls its methods
without being able to name its path.

```rust
mod vocabulary;

pub(crate) struct Vocabulary { }
```

A module is opened only to keep its name in the path of the caller. That is the
entire test. When a caller writes `languages::Rust`, the name reveals which module
the type came from, and the module is opened so that it can. When a caller would write
`Rust` alone, the name reveals nothing, the module was opened to flatten a type out
of it, and that is the mistake.

Where a caller cannot name what it needs and no namespace belongs in its path, the
caller is in the wrong location. The compiler is showing that this code reaches into
the internals of somebody else, and the answer is to move it under whatever owns the
thing, or to let the owner publish it instead.

## A layer publishes concepts, not individual types

What leaves a layer is a concept, not the list of types that make up that concept. So
the layer opens each of its leading concepts as a module and lets the concept publish
its own types.

```rust
pub mod generation;
pub mod service;
pub mod system;

mod error;

pub use error::Error;
```

**The leading concept is a namespace, and the type at the head of it is named for that
concept.** `generation::Generation` and `service::Service` are correct and are not a
repetition. The path names the concept and the type names the thing, and the fact that
the two words are the same is precisely what reveals that this concept has one type at
its head.

The error is the only thing a layer publishes as a bare type, because it belongs to no
concept and leaves every one of them.

Inside a concept the rule above still applies. A module is opened only where its name
earns a position in the path. The exception is a type that lives two levels below the
concept, because a module may publish no further than its own child. Then the module
between them is opened, and the path grows by the one step the tree actually has.

## Visibility

Every type is private until something outside it fails to compile. Then that type
widens by exactly one step, `pub(super)` first, `pub(crate)` next, bare `pub` only
at the crate edge where another crate consumes it. A module is opened only under
the rule above.

## Fields and accessors

Struct fields are private. Read access is a getter, named for the thing it returns
and nothing more. Write access is a setter or a builder.

## Calls

Assign a struct to a variable before passing it.

```rust
let tap_device = TapDevice { index: 0, maximum_transmission_unit: 1500 };
launcher.attach(tap_device);
```

Assign any expression to a variable before passing it. A call receives plain
variables, never inline expressions, however short.

```rust
let virtual_base = guest.virtual_base(region);
let page_size = guest.page_size(region);
let translation = table.translate(virtual_base, page_size);
```

Where the compiler needs a type it cannot infer, the variable carries it as an annotation, and the
call carries no turbofish: `let host: Host<Composition> = Host::from(parts);`, never
`Host::<Composition>::from(parts)`. A turbofish stays only where no variable can take the
type, such as a call whose result is passed on at once, which the rule above already forbids.

## Variables are nouns, functions are verbs

**Every variable is a noun, because a variable is named for what it is.** A variable, a
field and an argument all answer the question of what this is, and the answer to that
question is a thing. A variable named with a verb is wrong, and a variable named with
an adjective is worse, because an adjective is not even a claim about a thing, only
half of one.

**Every function is a verb, because a function is named for what it does.** The only
exception is a getter, which is a noun and returns the thing it names, and that is
all of the exception.

**A verb acts on a noun, and a word that only asserts something about an argument is
not one.** `describes`, `covers`, `mapped` and `readable` name no work and no thing.
They appear where a function was made to answer yes or no and a word was invented for
the question. The answer is to return what was asked about and let the caller
read it, so `block(address)` answers with the memory block or with nothing, and the
invented word is gone along with the second read it was doing. A verb in the present
tense stays right where the subject really does the thing, so an instruction `loads`,
a path `compiles` and a line `measures`.

**A preposition is no name.** `at`, `of`, `over` and `behind` state where one thing is
in relation to another and never what it is. `rest` answers nothing either, because
the rest of what is the question the name has to answer.

**Replacing a refused word with another empty one is not a rename.** `where` in place
of `at` is an adverb in place of a preposition and states exactly as little. A rename
is finished when the sentence "this value is a ..." can be completed with the name, and
until then the new word is as empty as the refused one.

**An adjective is allowed only in front of a noun.** `decrypted_asset` names which
asset and is a name. `decrypted` alone names which what, and is not.

```rust
let text = report.to_string();
let stated_tables = header.tables();
let bare_name = candidate.chars().all(|letter| !letter.is_whitespace());
```

Where a value is the outcome of doing something, the noun for it is the doing.

**A name is never shadowed by a second value.** Two values under one name are two
things a reader has to keep apart without being told which is which, and the only
way out is to name the second one. Where no name comes to mind, the value has a
role in whatever is being computed and that role is its name, so a division has a
dividend, a divisor and a remainder rather than three shadows of `bytes`.

The words this repository refuses outright are `written`, `given`, `at`, `of`, `over`,
`wrote`, `number`, `found`, `one`, `other` and `later`. Each of them names nothing.
`found` reports only that a search ended. `one` and `other` report only that there are
two of something, and neither reports more inside a compound, so no name has one of
them in it either. That is true of a function, a variable, a field, an argument, a constant
and the name a closure or a pattern declares. The name of a test is a sentence and may
contain them.

A number is refused as a name of its own. It counts and never states what the value is,
so `let four = ...` is wrong and the two operands of a bit operation are the value and
the `mask` it meets. An ordinal before a noun stays, because there the number is the
only true statement about the thing.

A refusal in this code is an error. The type is `Error`, the field is `error`, and
every name and every test name follows, so a test that proves one of them ends in
`is_an_error`.

Nothing here speaks, so `say` and `speak` are refused in every form. A name
states a meaning, a type constrains a value, a test proves a behavior and a gate
refuses. And a pointer points, so `aim` is refused as well.

## Functions

A free function named `verb_noun` is a design mistake. The noun is a type and the
verb is a method on it.

```rust
impl Persona {
    fn load(&self, store: &Store) -> Result<Profile, Error> { }
}
```

A function whose name begins with `print` is a design mistake. Implement
`Display` and let the call site print.

The name of a function states exactly what that function does, so a reader who never
opens the body knows what it returns and what that was made from. A name that fits any
function fits none.

Only a getter returns an `Option`, and its name is `maybe_` and the noun it may yield, so
`maybe_type()` and `maybe_module()`. A function that computes never returns an `Option`. It
returns a `Result`, and its error states why the computation yields no value, so its name is
`try_` and its verb and noun, as `try_count_block_comment_bytes()`. `maybe_` never stands in
front of a verb, because the function then reads as one that performs its verb only perhaps.
A name a trait dictates stays as the trait declares it, so `try_from`, `from_str`, `next` and
`fmt` keep their names, and a test is a sentence and keeps its name.

A function is named for what a program does, never for what a person does. A program does
not take, hold, find or collect anything. It lexes, parses, resolves, declares, counts,
strips, decodes and compares, and its verb comes with the noun it acts on. A verb
alone is no term of a vocabulary, the term is the verb with its noun, as `Count Line Bytes`.

A `usize` that counts or locates is not wrapped, because it is arithmetic and no concept.

A name that is a noun is a getter and returns the thing it names. A name that is a
verb does work and the verb names which work. Nothing else is a function name, so
there is no `get_` prefix and no verb on something that only yields a field.

```rust
let term = entry.term();
let reason = ban.reason();
```

`of`, `get`, `make`, `build`, `run`, `do`, `handle`, `process`, `execute`, `apply`
and `perform` are banned outright. They name no work. Wherever one of them seems
fitting, the work has a real verb, and that verb is the name.

A name a trait dictates belongs to the trait, so `from`, `fmt`, `next` and
`into_iter` are named the way the trait declares them.

## Constructors

Every constructor is `From`, and where it can fail, `TryFrom`. There is no `new`,
no `of`, no `with_something` and no free constructor function. Rust already has the
trait for turning one value into another and this project uses it.

A constructor that needs several values takes one named struct made of them, and
that struct is what `From` receives.

```rust
struct Ban {
    word: Word,
    path: PathBuf,
    reason: Reason,
}

impl From<Ban> for Refusal {
    fn from(ban: Ban) -> Self { }
}
```

This is why every intermediary is a named struct. The struct is the argument list,
it states the meaning of what went in, and the conversion is the standard trait
rather than a name somebody invented.

A constructor that needs nothing is `From<()>`. Unit is the absence of an argument
rather than a tuple of values, so this is not the tuple the rule above forbids.

```rust
impl From<()> for Language {
    fn from(_: ()) -> Self { }
}
```

**Only `From` and `TryFrom` return `Self`, and only they take one.** Before the conversion the
value does not exist yet, so no other function can receive one or return one. The
computation before the conversion works on whatever the type is made of, and that is
what those functions take and return.

The one exception is a `const` that has to exist at compile time. `From` is not
`const`, so a type that is a constant has a `const fn new` beside its conversions,
and that is the only location the name `new` appears.

## Traits

The attribute `derive` names every trait the type mechanically supports, in the order
`Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`, `Copy`, `Default`.
`Copy` for anything cheaply bitwise copyable. `Hash` always accompanies `Eq`.
Where the attribute `derive` gives the wrong default, write the impl.

If a standard trait exists for what you are doing, implement it.

| Concept | Trait |
|---|---|
| Human readable text | `Display` |
| Infallible conversion | `From` |
| Fallible conversion | `TryFrom` |
| Zero or empty value | `Default` |
| Equality | `PartialEq`, `Eq` |
| Ordering | `PartialOrd`, `Ord` |
| Iteration | `Iterator`, `IntoIterator` |
| Parsing from text | `FromStr` |
| Errors | `core::error::Error`, which `std` publishes as `std::error::Error` |
| Indexing | `Index`, `IndexMut` |

Inside an `impl` block the type is `Self`, in return types, constructors,
associated calls and patterns alike.

## Errors

Every error is information, and destroying information is forbidden. A failure
contains the reason, the location and the context of what is broken. Turning it into a
default value, a `false`, a silent return or a discarded result makes the broken state
look like a success.

A `Result` is propagated with `?` or handled in a branch that converts or reports the
error. `let _ = ...` and `.ok()` on a `Result` are forbidden, and so are
`unwrap_or(false)`, `unwrap_or(true)` and `unwrap_or_default()` on one. Where a
missing file means absence rather than failure, `ErrorKind::NotFound` is matched
explicitly and every other kind is still an error.

A subprocess that exits with a status other than zero is a failure, and every status
is checked. Its standard error is inherited or forwarded, never sent to
`Stdio::null()`, because the tool states the reason it failed and that reason is the
information.

The only tolerated suppression is inside `Drop`, where `?` does not exist, and there
the error is still reported.

An error is called an error in every name, never a failure: a type is `Error` or names its
concept before `Error`, such as `WindowError`, and its module is `error`. The owner decided this
on 2026-11-13.

## Matches are exhaustive

A `match` names every variant of an enum of this repository, and no arm is the wildcard
`_`. When a variant is added, the compiler refuses every `match` that does not decide what
the new variant means, and that refusal is the point. A struct whose fields are all read is
destructured, `let Endpoint { host, port } = endpoint;`, so a field that is added later is
a compiler error wherever the struct is taken apart, and `..` stands only where a field is
deliberately left unread.

## Nothing is kept for later

The code states what is, now. There is no shim for an older caller, no re-export of an item
that moved, no `#[deprecated]` stub and no item kept in case it is needed again. Code that
nothing reaches is removed, and git keeps what it was.

## Numbers

No type suffix on a numeric literal. Annotate the variable instead.

```rust
let queue_count: u32 = 0;
let buffer: Vec<u8> = vec![0; size];
```

No underscore inside a numeric literal. A number is one number and is spelled as
one, so `0b00000101` and `1000`, never `0b0000_0101` or `1_000`.

No `as` cast outside the body of a `From` or `TryFrom` impl. Use `From`, `Into`
or `TryFrom`.

## Hexadecimal

Hexadecimal this repository forms is in upper case, and the `x` of a `0x` in front of
it stays lower case because the `x` is a marker and not a digit.

```
0x0000000000006A90
```

Hexadecimal another tool produces and reads back again stays exactly as that tool
produces it, which for `nix` and `sha256sum` means lower case. Input is accepted
leniently where output is strict, so a type that parses hexadecimal somebody else
produced accepts either case.

## Composition

When every field of a struct comes from another struct unchanged, embed that
struct as a named field rather than copying its fields.

## Purity

Deciding and doing are two functions. A decision takes what it needs as an
argument, returns what it produced, reads no clock and no file, keeps no state
between calls, and yields the same result every time. Doing what that decision states
touches the world and decides nothing.

Which program runs, with which arguments, and which file has to appear afterward
is a decision, and it is proven by a test with plain values. Spawning the program
and looking for the file is the doing, and it is a handful of lines with no branch
that a test would want to reach.

IO lives with the thing that owns it. `main.rs` parses the command line, delegates,
reports and sets the status. It only wires: it implements nothing, holds no `impl`, no
function besides `main` and no type of its own, and everything it composes is a type with
methods in the module tree of its crate.

The test is whether the decision can be called from a test with plain values and no
preparation. Where it cannot, the decision and the doing are still one function and
have to be cut apart.

**A function is pure and immutable.** It takes its inputs as owned values or shared
references, as one named struct where there is more than one input or where the input
means something, and only a trivial input such as a `usize` stays bare. It returns what it
computed and changes nothing it was handed. Its result depends on its inputs alone and is
the same every time.

A `&mut` parameter, a method on `&mut self`, a `static mut`, interior mutability and a
global are absent from the domain, because each of them lets a mutation cross the boundary
of a function. Where a value changes, the function returns the new value.

A local `let mut` inside a body is allowed, because its mutability stays inside the
function and the inputs and the output stay pure and deterministic. A value it names is
created in that body, mutated there and returned or dropped there, and it is never one of
the inputs.

## Ports

Every external dependency is reached through a port. The file system, the network, the
database, the clock, randomness, the environment and processes are reached only through a
trait the domain declares, and an adapter of the infrastructure implements it. A test of
the domain hands the domain a value or a fixed implementation of the port and never touches
the world, so it is deterministic and fast.

A port is a trait, never a closure. A type that needs many adapters takes one trait whose
associated types name them, split into one trait for every concern, and never one type
parameter for every adapter or for every function that builds one.

Real infrastructure is tested where it lives. An adapter that translates the domain into a
program is unit tested over the transcript spawner, and the one adapter that starts programs,
`stack_process_adapter_spawner_standard_library`, is tested end to end under its own runner.

## State and threads

The domain holds no state of its own between two calls. What a domain knows arrives as an
input and leaves as an output. Where state is unavoidable, it is held by an adapter behind a
port, and the domain computes the next state from the current one.

Every type of the domain is `Send` and `Sync`. `Rc` and `thread_local!` are absent from it.
A port a service awaits returns a future that is `Send`. Shared state in the infrastructure
is owned by exactly one adapter and reached through messages or through the database, never
through a lock a caller holds, so no deadlock and no contention over a lock can arise
between callers.

**An adapter that owns state is an actor.** It is two things. The actor is a task that owns
the state and the receiving end of a channel, and nothing else reaches the state. The handle
holds the sending end, implements the port, and is cloned freely, because the channel has
many senders and one receiver. The handle starts the task where the handle is constructed,
so the task owns everything it holds. A request that needs a response carries the sending
end of a `oneshot` channel, and the handle awaits the response on the other end.

Every channel between actors is bounded, so a slow actor pushes back on its senders instead
of letting its queue grow. No cycle of bounded channels connects actors, because each actor
of such a cycle can wait for the next to receive, and none ever does. Where two actors need
each other, the path back is a `oneshot` or a `try_send`, which never waits. The task ends
when the last handle is dropped, because its channel then yields no further message.

A send that fails is information like every other error. Where the receiver of a response
is gone, the requester stopped waiting, and the actor matches that case and reports it
rather than discarding it.

## The implementation is never visible from outside

A caller cannot tell whether a value keeps a `Vec`, a map or a computed field, and the type
can change that without any caller changing. Fields are private, and a type is reached
through its conversions, its getters, its methods and the standard traits it implements.

A value object that combines with another of its kind implements the operator or the
standard trait for it, such as `Add` for an amount of experience, `BitOr` for a set of
permissions, `FromIterator` for a collection and `Ord` for a rank, so composing two of them
reads as ordinary Rust and needs no function invented for it.

## One function, one task

A function prepares and then performs its one task. The preparation is whatever that
task needs, and it is what the lines above the last one are for. Where a
second task begins, a second function begins with it.

**A function aims at 25 lines.** The lines are those between the braces of its body. It is a
direction and no limit. A function that does one thing cleanly may have 50 lines or more,
such as a list of the definitions of a context. A longer function has a reason, and the
review asks for it. Where a function is long because it does a second task, the second task
is a second function, named for that task. The aim holds for the functions of the production
code; a function of a test module may be as long as its test needs.

**A module holds up to 500 lines.** The lines are those of a file `.rs` before its test module;
the test module does not count. One concept
of a domain rarely needs more. A longer module likely holds a concept of its own, such as the
forms of a statement beside the statement, and that concept moves into a module beneath it,
named for the concept. It is a direction and no limit, and a longer module has a reason the
review asks for.

**A function works on one value.** Where two different values appear under one
name, they are two things and the name hides it. That is a missing variable, not a
reason to fold them together.

**A parameter a function only passes on is forbidden.** Whoever takes it uses it.
Where a function needs an argument so that the one below it can have it, the
sequence belongs to whoever already has the argument.

**The sequence of steps belongs to whoever drives them.** A function that puts two
steps in order is a third step nobody asked for, and it is named for one of the two
while doing both.

**Two functions with the same body and the same signature are one operation**, and
at least one of the two names is untrue.

**A function whose body is one operator does not exist.** A name in front of an
operator the type already answers to adds a word and no meaning.

## Types name concepts, not containers for values

A value between two states is not a type. Where a fold needs a name for what it
passes along, the answer is the value it passes along, named for what it is.

A type invented for that purpose ends up with no body, because every operation it
would offer turns out to be an operation of what it is made of, or of what drives it.

Where a type really is missing, its word is usually already in the language of the
context.

## Tests

Tests come first. A behavior is expressed as a failing test, the test is made to
pass, and the code that resulted is what stays.

A test lives inline in the file of the thing it proves, in a `#[cfg(test)] mod
tests` at the bottom, and nowhere else. There is no test file. The directory `tests` of a
crate contains only `tests/assets`, the data its tests read, such as the fixtures under
`tests/assets/fixtures`.

A unit test starts no program and touches no infrastructure, and `nextest` runs nothing else. A test that starts a program, such as QEMU, swtpm or ssh, is an end-to-end test, and it runs
under its own runner, as `nix/flake/checks/README.md` states.

A test module may use `use super::*;`.

Two things keep an arrangement short, so a test does not swell and every test reads at
once as what it provides, what is done with it and what is then expected.

- **Fixtures** are the files under `tests/assets/fixtures`, the data a test reads.
- **Test factories** build the values an arrangement needs and nothing else. A factory is
  named for what it builds, such as `overlay_factory`, and it is reached only from tests.

Every function of a test module that is no test stands before the first test, in a
submodule of the test module: `fixture` holds the factories and whatever builds the data a
test reads, `helper` whatever else only tests need, such as running the program and reading
its output. A test calls them by their path, `fixture::overlay_factory()` or
`helper::standard_output(output)`, so the call says where the function lives.

The act and the assertion are written out in the test.

A string a test writes out that holds a quote or a line break is a raw string, `r#"…"#`,
written over as many lines as it holds, so that nothing in it is escaped. A string longer
than a line is bound to a variable first, like every other argument of a call, and the
assertion compares variables.

```rust
let character_text = String::from(
    r#"name = "Character"
kind = "class"
"#,
);
let expectation = Ok(character_text);
assert_eq!(text, expectation);
```

Every crate of baton and every tool is read through by its tests, every region,
every function and every line.

Every test is three blocks in this order, the arrangement, the act and the
assertion, separated by exactly one blank line and by nothing else. Within a block
the lines follow one another with no blank line between them, so the three blocks
are clear at a glance.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_is_kept_in_lower_case() {
        let spelling = "Kernel";

        let word = Word::from(spelling);

        let text = word.to_string();
        let expectation = "kernel";
        assert_eq!(text, expectation);
    }
}
```

## Finding the module tree

The rules above describe a good module tree. This section describes how to arrive at
one, because a tree is not designed once but found, by one question asked repeatedly.

> Which concept uses this concept?

Ask it of every module. If there is an answer, the module belongs inside that answer
and not where it was. Ask it again of the result, until the question has no answer
left anywhere.

Only two kinds of module remain at the root of a layer. The first is what leaves the
layer, because only the layer publishes and a module keeps its children to itself. The
second is what two independent concepts share, because it belongs to neither of them.
Everything else has a user, and its user is its home.

### Compiler errors as design feedback

Moving a concept inward makes it unnameable from outside, because a module keeps
its children to itself and only the layer publishes. That refusal is not an
obstacle to work around by moving the type back to the root. It is the same question,
asked a second time by the compiler.

There are two honest answers, and hoisting is neither of them. Either the type has
no business leaving its module and the caller was in the wrong location, or the type
should not exist. Removing the type usually makes the code smaller, and that is the reason
the refusal is worth listening to rather than working around.

## Comments

Crates contain no comments. Not doc comments, not line comments, not module prose.
A name states what a thing is, a type constrains what it is, and a test proves what
it does. Where none of these conveys the meaning, the design is wrong, and the design
is what changes. What a crate does for a reader lives in its `README.md` and its
own book.

The `README.md` of a crate is its overview and nothing more: what the crate is for, its layer,
the ports it declares or fills, how it is built and tested, and the chapters of its book. It
fits on a screen. Everything else is a chapter of the book under `docs`, one page for every
concept, and the pages follow the module tree as the sources do: the module `scene::terrain`,
whose source is `src/scene/terrain/mod.rs`, is described by `docs/scene/terrain/README.md`.
`docs/SUMMARY.md` lists them as that tree. A page holds up to 500 lines; a longer page holds a
concept of its own, and that concept becomes a page beneath it.

## What a search cannot check

A rule that a search can check has already been checked, by the gate that runs the
search. What remains is what only a human reviewer finds, and that is naming, in its
entirety.

Every name is read as the sentence it forms. A name that would fit any variable fits
none, so `done`, `of`, `one` and `made` are not names but gaps where a name is
missing, and they deserve no less attention than a wrong type. A count is named for what it counts and a path for what it leads to.

The test is whether somebody who reads one line knows what it is and what it does
without a second line.

## Lints

No `#[allow(...)]` and no `#![allow(...)]` anywhere, for any lint, and that includes the
code a macro of this repository generates. A lint that fires is the tool pointing out that
the code is wrong, so the code changes.

There is no exception. The workspace forbids `unsafe_code` in every crate, and no module
allows anything.

`clippy::module_inception` is fixed by not nesting. `clippy::too_many_arguments`
is fixed by composing the arguments into a named struct. `dead_code` is fixed by
removing the item or wiring it in.

If a lint genuinely cannot be satisfied, stop and ask.

## Every language

Every rule above binds every language. This section states what each rule means where a
language has no struct, no trait and no module of Rust.

### Nix

- A Nix file is a module, `<name>/default.nix`, never `<name>.nix`. The structure of the
  outputs of the flake is the structure of the directories, so an output lives at
  `nix/flake/<output>`.
- Every derivation is a real derivation, called through `callPackage`. `runCommand` is not
  used. A shell script is `writeShellApplication`.
- Every input is an argument, so every derivation can be overridden. Nothing that matters
  to evaluation is fixed inside a `let`.
- `inherit` is used wherever it applies, and `with` is never used.
- An attribute, an argument and a binding of a `let` are nouns, named as a variable of Rust
  is named.
- A Nix file contains no comment. What a derivation is for is in the `README.md` beside
  what it builds.

### Shell

- A variable is a noun for what it holds, and a function is a verb with its noun.
- Control flow is `if`. `&&` and `||` are never used for control flow.
- An error is never destroyed. No `2>/dev/null`, no `|| true`, no status left unchecked.
  Every program prints the reason it failed.
- An option is written in its long form where the program has one, so `--recursive` and
  not `-r`.
- Every expansion is quoted, and shellcheck refuses what is not.
- A script contains no comment. What it is for is in the `README.md` beside it.

### Markdown

- A Markdown file is named in upper snake case, such as `README.md`, `UBIQUITOUS_LANGUAGE.md`
  and `WALKING_SKELETON.md`.

