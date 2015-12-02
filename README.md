# teller-cli

The purpose of this command line tool is to provide a human-interface for your bank and not merely to be a one-to-one match with the underlying API.

### TODO

- [ ] Come up with a good `docopt`s.
- [ ] Refactor to use `try!` and `get!`.
- [ ] `account` will probably be `(current|business|savings)`.
- [ ] Work out the best way of modularising and refactoring the config code.
- [ ] Refactor question to answer.
- [ ] If there is no config ask a question and save a config containing the answer. Set `is_first_time`.
- [ ] Make a request to get the list of accounts.
- [ ] Print the list of accounts in a table.
- [ ] If `is_first_time` then fetch the list of accounts and ask questions to alias each of these as their `last_4_digits`.
- [ ] Implement `teller show current --balance`.
- [ ] If `show <account_alias>` is not an alias check to see if it is a real account.
- [ ] Refactor `TellerService`.
- [ ] Write `README.md`.
- [ ] Write unit tests.
- [ ] Add shields.

## FAQ

#### Compiling gives `openssl/hmac.h` not found error

Ensure that both Homebrew and `openssl` are installed, and then [try running `brew link --force openssl`](https://github.com/sfackler/rust-openssl/issues/255).

This relates to the following error:

```
--- stderr
src/openssl_shim.c:1:10: fatal error: 'openssl/hmac.h' file not found
#include <openssl/hmac.h>
```
