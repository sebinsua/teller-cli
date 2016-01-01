Take a read of [this well-written rust code](https://www.reddit.com/r/rust/comments/2pmaqz/well_written_rust_code_to_read_and_learn_from/).

- [ ] Data types from within TellerClient should be encapsulated depending on what they belong to.
- [ ] Non-HTTP parts of `'client'` should perhaps be renamed to `'inform'` and
      receive the data from the API instead of creating it. Information applied
      does not belong to the client.
- [ ] Move client to a separate crate `teller_api`.
