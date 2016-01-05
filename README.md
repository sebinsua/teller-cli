# teller-cli ![Build Status](https://img.shields.io/travis/sebinsua/teller-cli.svg)
> Banking for your command line

This tool provides useful ways of interrogating your bank through your command line, and is not merely meant to be a one-to-one match with underlying APIs.

It uses [Teller](http://teller.io) behind-the-scenes to interact with your UK bank, so you will need to have an account there. Want an account? [@stevegraham can hook you up!](https://twitter.com/stevegraham)

**:point_up_2: Heads up!** (1) This is my first [Rust](https://www.rust-lang.org/) project, (2) the interface is in flux while I try to make it human-like without ending up redundant, and (3) there are [no tests yet](https://github.com/sebinsua/teller-cli/issues/1)! This is soon to change with the release of [v1.0.0](https://github.com/sebinsua/teller-cli/issues?q=is%3Aopen+is%3Aissue+milestone%3Av1.0.0).

## Usage

`teller show balance current` will show you your current account's balance.

`teller --help` for more commands.

*e.g.*

![Instructions](http://i.imgur.com/cvZRwev.png)

## Why?

#### Notifications with [`terminal-notifier`](https://github.com/julienXX/terminal-notifier)

`teller show balance current | terminal-notifier -title "Current Account Balance"`

![Notifications](http://i.imgur.com/RxCSig9.png)

#### Alert when :moneybag: low

```sh
#!/bin/sh

CURRENT_BALANCE=`teller show balance current --hide-currency`;
MIN_BALANCE=1000.00;

if (( $(bc <<< "$CURRENT_BALANCE < $MIN_BALANCE") ))
then
  echo "Your current balance has fallen below £$MIN_BALANCE" | terminal-notifier -title "💰 Alert" -subtitle "Current Balance is £$CURRENT_BALANCE";
fi
```

![Alerts](http://i.imgur.com/OXU5uyv.png)

#### :coffee: How much money do I spend at [Nanna's](http://www.nannasn1.com/)?

```
> teller list transactions current | grep "NANNA'S"
27   2015-11-12  NANNA'S             -2.70
60   2015-10-28  NANNA'S             -2.40
68   2015-10-26  NANNA'S             -5.40
101  2015-10-09  NANNA'S             -2.70
203  2015-07-17  NANNA'S             -4.60
206  2015-07-16  NANNA'S             -9.90
208  2015-07-16  NANNA'S             -9.30
209  2015-07-16  NANNA'S             -0.10
```

Hopefully Teller will add support for querying transactions soon.

#### Am I saving money with a chart :chart_with_upwards_trend: with [`spark`](https://github.com/holman/spark)

```
> teller list balances business --interval=monthly --timeframe=year --output=spark | spark
▁▁▁▂▃▂▃▄▄▅▆█
```

#### Have I spent more money this month than I normally do?

```sh
#!/bin/sh

CURRENT_OUTGOING=`teller show outgoing current --hide-currency | sed 's/^-//'`;
OUTGOINGS=`teller list outgoings current --output=spark`;
SUM_OUTGOING=`echo "$OUTGOINGS" | sed 's/ /+/g' | bc -l | sed 's/^-//'`;
COUNT_OUTGOING=`echo "$OUTGOINGS" | wc -w | xargs`;
AVERAGE_OUTGOING=`bc <<< "scale=2; $SUM_OUTGOING / $COUNT_OUTGOING"`;

if (( $(bc <<< "$CURRENT_OUTGOING > $AVERAGE_OUTGOING") ))
then
  DIFFERENCE_OUTGOING=`bc <<< "scale=2; $CURRENT_OUTGOING - $AVERAGE_OUTGOING"`;
  echo "You've spent £$DIFFERENCE_OUTGOING more than normal." | terminal-notifier -title "💰 Spending Alert" -subtitle "Current Outgoing is £$CURRENT_OUTGOING";
fi
```

#### Show your current balance and last transaction in the OSX Menu Bar with [Bitbar](https://github.com/matryer/bitbar)

`show-current-balance.1h.sh`
```sh
#!/bin/sh
export PATH='/usr/local/bin:/usr/bin/:$PATH';                    

CURRENT_BALANCE=`teller show balance current --hide-currency`;
LAST_TRANSACTION=`teller list transactions | tail -n 1 | pcregrep -o1 "[0-9]+[ ]+(.*)"`;

echo "£$CURRENT_BALANCE";
echo "---";
echo "$LAST_TRANSACTION";
```

![Current Balance in OSX Menu Bar](http://i.imgur.com/BzkazSB.png)

## Installation

### From release

```
> curl -L https://github.com/sebinsua/teller-cli/releases/download/v0.0.5/teller > /usr/local/bin/teller && chmod +x /usr/local/bin/teller
```

### From source

First `git clone` and then:

```
> cargo build --release && cp ./target/release/teller /usr/local/bin && chmod +x /usr/local/bin/teller
```

## FAQ

#### Compiling gives `openssl/hmac.h` not found error

Ensure that both [Homebrew](https://github.com/Homebrew/homebrew) and `openssl` are installed, and then [try running `brew link --force openssl`](https://github.com/sfackler/rust-openssl/issues/255).

This relates to the following error:

```
--- stderr
src/openssl_shim.c:1:10: fatal error: 'openssl/hmac.h' file not found
#include <openssl/hmac.h>
```
