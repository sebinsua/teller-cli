# teller-cli ![Build Status](https://img.shields.io/travis/sebinsua/teller-cli.svg)
> Banking for your command line

The purpose of this command line tool is to provide a human-interface for your bank and not merely to be a one-to-one match with the underlying API.

### TODO

- [ ] Carefully [remove the `unwrap` statements](https://github.com/Manishearth/rust-clippy/issues/24) and clean up deep matches.
- [ ] Add some `error!`s for errors, instead of always panicking.
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
