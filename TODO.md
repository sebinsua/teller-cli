Take a read of [this well-written rust code](https://www.reddit.com/r/rust/comments/2pmaqz/well_written_rust_code_to_read_and_learn_from/).

- [ ] Carefully [remove many of the `unwrap` statements](https://github.com/Manishearth/rust-clippy/issues/24) and clean up many of the deeply-nested matches in the usual ways (separate functions, `unwrap_*`, early returns, `let expected_value = match thing { ... }`.
- [ ] Data types from within TellerClient should be encapsulated depending on what they belong to.
- [ ] The remaining non-HTTP parts of `'client'` should perhaps be renamed to `'inform'` and receive the data from the API instead of creating it (the information that is applied, does not belong to the client).
- [ ] Move client to a separate crate `teller_api`.
