Take a read of [this well-written rust code](https://www.reddit.com/r/rust/comments/2pmaqz/well_written_rust_code_to_read_and_learn_from/).

- [ ] Refactor `'client'`:
  - [ ] Create a struct that receives an `authToken` on instantiation and implements basic methods to fetch data - each of these should use some kind of underlying `auth_request` method. This is instead of everything receiving `&Config` (a class that belongs to another module). Move to a separate crate `teller_api`.
  - [ ] The remaining non-HTTP parts of `'client'` should perhaps be renamed to `'inform'` and receive the data from the API instead of creating it (the information that is applied, does not belong to the client).
- [x] Refactor `'inquirer'`:
  - [x] Into a module directory.
- [x] Refactor `'main'`:
  - [x] `struct`s and `impl Decodable`s should remain where they are as are command definition related.
  - [x] `pick_command` should be simpler:
    - [x] Only destructure args to get out what you need to match. 'Additional destructuring](http://rustbyexample.com/flow_control/match/destructuring/destructure_structures.html) should happen before the underlying command is called.
    - [ ] Only `get_config` once, if it is successful then pass the `&config` onto the commands, otherwise `configure_cli`. `get_config` should not be concerned with execution of `configure_cli` itself. This will mean you will attempt to detect whether `cmd_init` was picked first, separately to the rest of the other commands prior to `get_config` being executed.
    - [ ] `get_config` should belong to the `'config'` module.
  - [ ] `init_config` should become `ask_questions_for_config`. Behind the scenes it should use `inquirer::ask_questions` and some kind of `find_answers*` function to get the answers out for the config.
  - [x] Commands should be moved into a module `'command'`. This includes `configure_cli`, `list_accounts`, `show_balance`, `list_transactions`, etc.
  - [ ] Table writing and other response writing should live in `'represent'`. This includes `get_account_alias_for_id`, `represent_list_accounts`, `represent_show_balance`, `represent_list_transactions`, `represent_list_amounts`, `represent_list_balances`, `represent_list_outgoings`, `represent_list_incomings`, etc.
- [ ] Carefully [remove many of the `unwrap` statements](https://github.com/Manishearth/rust-clippy/issues/24) and clean up many of the deeply-nested matches in the usual ways (separate functions, early returns, `let expected_value = match thing { ... }`.
