Take a read of [this well-written rust code](https://www.reddit.com/r/rust/comments/2pmaqz/well_written_rust_code_to_read_and_learn_from/).

- [ ] `get_account_id` should be a method on the `Config` object. `get_account_alias_for_id` should also be a method of this.
- [ ] Create a struct that receives an `authToken` on instantiation and implements basic methods to fetch data - each of these should use some kind of underlying `auth_request` method. This is instead of everything receiving `&Config` (a class that belongs to another module). Move to a separate crate `teller_api`.
- [ ] The remaining non-HTTP parts of `'client'` should perhaps be renamed to `'inform'` and receive the data from the API instead of creating it (the information that is applied, does not belong to the client).
- [ ] Carefully [remove many of the `unwrap` statements](https://github.com/Manishearth/rust-clippy/issues/24) and clean up many of the deeply-nested matches in the usual ways (separate functions, `unwrap_*`, early returns, `let expected_value = match thing { ... }`.
- [ ] Use `rustfmt`.
