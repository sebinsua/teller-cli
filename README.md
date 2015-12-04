# teller-cli

The purpose of this command line tool is to provide a human-interface for your bank and not merely to be a one-to-one match with the underlying API.

### Styling recommendations that I will try to follow

- Map over results or chaining monadic-like operations.
  See: https://news.ycombinator.com/item?id=7792456
- Prefer `try!` to tedious result matching to pass errors back.
  See: https://github.com/rust-lang/rust/blob/master/src/doc/style/errors/ergonomics.md
  `-> Result<(), IoError>`
- Obstruction type errors (e.g. `FileNotFound`) should use `Result<T, E>`.
- The `Option` type should not be used for "obstructed" operations; it should only be used when a `None` return value could be considered a "successful" execution of the operation.
- Do not create `Result`-producing and `panic`king versions of functions. Use just the former.
- Unit tests should live in a tests submodule at the bottom of the module they test, marked with `[cfg(test)]`.
- Think about guarantees is an essential part of designing Rust code. For example, `std::str` guarantees that an underlying buffer is valid UTF-8, while a `std::path::Path` guarantees no interior nulls. In general: prefer static enforcement of guarantees over dynamic enforcement as (1) bugs can be caught at compile-time, and (2) there is no runtime cost.
- If a constructor is becoming complex, then consider creating a builder with a basic constructor and a set of methods returning a `&'a mut` instance of `self` with a final terminal method that returns the `struct` that we were really wanting to build.
  See:
  https://github.com/rust-lang/rust/blob/master/src/doc/style/ownership/builders.md
- Create constructors `pub fn new()` and with sensible defaults for passive structs. This can then be overridden, like so `Config { color: Red, .. Config::new() }`.
- Use custom types to imbue meaning instead of core types.
- Newtypes are a zero-cost abstraction: they introduce a new, distinct name for an existing type, with no runtime overhead when converting between the two types.
  Newtypes can be defined like:
  ```
  struct Miles(pub f64);
  struct Kilometers(pub f64);
  ```
- Unless you wish to commit to a representation make the fields private. Also making a field public means it cannot be constrained with an invariant.
- Prefer immutable bindings, and consider making a binding immutable immediately after it has finished being mutated.
- Mirror the module hierarchy in the directory structure.
- Headers should be ordered: `extern crate`, external `use`, local `use`, `pub use`, `mod`, `pub mod`.
- Place modules in their own file.
  See:
  https://github.com/rust-lang/rust/blob/master/src/doc/style/features/modules.md
- Prefer borrowing arguments rather than transferring ownership, unless ownership is actually needed.
  ```
  fn foo(b: &Bar) { ... } // OK
  fn foo(b: &mut Bar) { ... } // OK
  fn foo(b: Bar) { ... } // BAD
  ```
- It's better to expect an enum than a series of mutually-exclusive `bool`s as this is static enforcement. If that's not possible however, you can use something like `debug_assert!` which can be switched off in production.
- Return useful intermediate results rather than throwing them away.
- You can yield back ownership if there is an error. For example `fn from_utf8_owned(vv: Vec<u8>) -> Result<String, Vec<u8>>`.
- `snake_case` for values, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constant or static variables, and short lowercase `'a` for lifetimes. Avoid module name redundant prefixes.
- Conversions marked `as_` or `into_` typically decrease abstraction, while those marked `to_` usually stay at the same level of abstraction but do some work to change their representation.

### TODO

- [x] Modularise the config code.
- [x] Modularise the requests code into `TellerClientService`.
- [ ] Modularise the inquirer'esque code.
- [ ] Refactor the config, client, and inquirer.
- [ ] If there is no config ask a question and save a config containing the answer. Set `is_first_time`.
- [ ] Logic to print the list of accounts to help with naming them.
- [ ] If `is_first_time` then fetch the list of accounts and ask questions to alias each of these as their `last_4_digits`.
- [ ] Implement `teller show balance current`.
- [ ] If `show balance <account_alias>` is not an alias check to see if it is a real account; also consider defaults.
- [ ] Write `README.md`.
- [ ] Write unit tests.
- [ ] Add shields.
- [ ] Move the style guidelines into my personal notes, once I no longer need to quickly refer to them.

## FAQ

#### Compiling gives `openssl/hmac.h` not found error

Ensure that both Homebrew and `openssl` are installed, and then [try running `brew link --force openssl`](https://github.com/sfackler/rust-openssl/issues/255).

This relates to the following error:

```
--- stderr
src/openssl_shim.c:1:10: fatal error: 'openssl/hmac.h' file not found
#include <openssl/hmac.h>
```
