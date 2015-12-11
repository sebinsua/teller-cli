# teller-cli ![Build Status](https://img.shields.io/travis/sebinsua/teller-cli.svg)
> Banking for your command line

The purpose of this command line tool is to provide a human-interface for your bank and not merely to be a one-to-one match with the underlying API.

### TODO

- [ ] Look at inquirer and use it to create a question/answer vector structure.
- [ ] `init_config` should `list_accounts` and `ask_questions` (to store the `links.self` of each in the config aliases, before passing the config onwards).
- [ ] Carefully [remove the `unwrap` statements](https://github.com/Manishearth/rust-clippy/issues/24).
- [ ] Write unit tests.
- [ ] Refactor the config.
- [ ] Refactor the client.
- [ ] Refactor the inquirer.
- [ ] Write `README.md`.

## FAQ

#### Compiling gives `openssl/hmac.h` not found error

Ensure that both Homebrew and `openssl` are installed, and then [try running `brew link --force openssl`](https://github.com/sfackler/rust-openssl/issues/255).

This relates to the following error:

```
--- stderr
src/openssl_shim.c:1:10: fatal error: 'openssl/hmac.h' file not found
#include <openssl/hmac.h>
```
