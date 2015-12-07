# teller-cli ![Build Status](https://img.shields.io/travis/sebinsua/teller-cli.svg)
> Banking for your command line

The purpose of this command line tool is to provide a human-interface for your bank and not merely to be a one-to-one match with the underlying API.

### TODO

- [ ] By default with no arguments call `--help`.
- [ ] `teller [<verb=show>] <object> [<account-alias>]` rather than `teller show balance current`. `<account-alias>` should have a default.
- [ ] Comment out the config calling code. Comment out the client calling code.
- [ ] Setup a config file with some sensible defaults for the account aliases as well as the `auth_token`.
- [ ] Work out whether a particular command has been picked and then `println!`.
- [ ] For each of the picked commands:
      - [ ] list accounts: call client code with `list_accounts`.
      - [ ] show balance: call client code with `show_balance`.
      - [ ] `represent_list_accounts`.
      - [ ] `represent_show_balance`.
- [ ] Set `is_first_time` to `false` by default.
- [ ] Set `is_first_time` dependent on whether the config file exists or not.
- [ ] If `is_first_time` is `true` then: `init_config` and pass the `config` to the picked command, else: pass the `config` to the picked command.
- [ ] `init_config` should `list_accounts` and `ask_questions` (to store the `links.self` of each in the config aliases, before passing the config onwards).
- [ ] Refactor the config.
- [ ] Refactor the client.
- [ ] Refactor the inquirer.
- [ ] Write unit tests.
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
