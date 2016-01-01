Take a read of [this well-written rust code](https://www.reddit.com/r/rust/comments/2pmaqz/well_written_rust_code_to_read_and_learn_from/).

- [x] Commands should all use `info!` to tell us what they're doing.
- [x] `*_command`s should map rather than match for concision, and then convert success or error to 0 or 1 (with an `error!`) on unwrapping.
- [ ] `get_config` should belong to `config/*` and have a `configure_cli` function passed into it.
- [ ] `get_account_id` should be a method on the `Config` object. `get_account_alias_for_id` should also be a method of this.
- [ ] We should not always be storing code within the `mod.rs`. Vice versa.
- [ ] Can the CLI `NAME` be set in the usage automatically?
- [ ] Refactor `'client'`:
  - [ ] Create a struct that receives an `authToken` on instantiation and implements basic methods to fetch data - each of these should use some kind of underlying `auth_request` method. This is instead of everything receiving `&Config` (a class that belongs to another module). Move to a separate crate `teller_api`.
  - [ ] The remaining non-HTTP parts of `'client'` should perhaps be renamed to `'inform'` and receive the data from the API instead of creating it (the information that is applied, does not belong to the client).
- [ ] `get_config` should not be concerned with execution of `configure_cli` itself. This will mean you will attempt to detect whether `cmd_init` was picked first, separately to the rest of the other commands prior to `get_config` being executed.
- [ ] `get_config` should belong to the `'config'` module.
- [ ] `ask_questions_for_config` should use `inquirer::ask_questions` and some kind of `find_answers*` function to get the answers out for the config.
- [ ] Carefully [remove many of the `unwrap` statements](https://github.com/Manishearth/rust-clippy/issues/24) and clean up many of the deeply-nested matches in the usual ways (separate functions, early returns, `let expected_value = match thing { ... }`.
- [ ] Use `rustfmt`.
