# teller-cli ![Build Status](https://img.shields.io/travis/sebinsua/teller-cli.svg)
> Banking for your command line

This tool provides useful ways of interrogating your bank through your command line, and is not merely meant to be a one-to-one match with underlying APIs.

It uses [Teller](http://teller.io) behind-the-scenes to interact with your UK bank, so you will need to have an account there.

**:point_up_2: Want an account?** [@stevegraham can hook you up!](https://twitter.com/stevegraham)

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

#### Keep track of your spending from the OSX Menu Bar with [BitBar](https://github.com/matryer/bitbar)

Create a [`track-spending.1h.sh`](https://github.com/matryer/bitbar-plugins/blob/master/Finance/teller-track-spending.1h.sh) within your plugins directory:
```sh
#!/bin/sh
#
# Teller.io Banking via the OSX menu bar
# Requires:
# - a Teller.io account
# - a UK Bank
# - teller-cli: https://github.com/sebinsua/teller-cli#from-release
# - pcregrep: `brew install pcre`
#
# <bitbar.title>teller-track-spending</bitbar.title>
# <bitbar.version>v1.3.1</bitbar.version>
# <bitbar.author>Seb Insua</bitbar.author>
# <bitbar.author.github>sebinsua</bitbar.author.github>
# <bitbar.desc>Track your spending from your menu bar</bitbar.desc>
# <bitbar.image>https://camo.githubusercontent.com/e0215e6736172334f62effff36ff8df1ab38fed1/687474703a2f2f692e696d6775722e636f6d2f627638545a4c652e706e67</bitbar.image>
# <bitbar.dependencies>teller-cli, pcregrep</bitbar.dependencies>
# <bitbar.abouturl>https://github.com/sebinsua/teller-cli</bitbar.abouturl>

export PATH="/usr/local/bin:/usr/bin/:$PATH";

SPENDING_LIMIT='3000.00'; # Change this to a suitable spending limit.

exit_if_zero() {
  RETURN_CODE=$1;
  ERROR_MESSAGE=$2;
  if [ "$ERROR_MESSAGE" = "" ]; then
    ERROR_MESSAGE="Offline";
  fi;
  if [ "$RETURN_CODE" -ne 0 ]; then
    echo "$ERROR_MESSAGE|color=#7e7e7e";
    exit 1;
  fi;
}

# If we're offline we shouldn't output junk in the menu bar.
curl --connect-timeout 5 www.google.com > /dev/null 2> /dev/null;
exit_if_zero $? "Offline";

CURRENT_OUTGOING=$(teller show outgoing current --hide-currency);
exit_if_zero $? "Error";

CURRENT_BALANCE=$(teller show balance current --hide-currency);
exit_if_zero $? "Error";

LAST_TRANSACTION=$(teller list transactions | tail -n 1 | pcregrep -o1 "[0-9]+[ ]+(.*)");
exit_if_zero $? "Error";

if [ "$(echo "$CURRENT_OUTGOING > $SPENDING_LIMIT" | bc)" -ne 0 ]; then
  OVERSPEND=$(echo "scale=2; $CURRENT_OUTGOING - $SPENDING_LIMIT" | bc);
  echo "🚨 £$OVERSPEND OVERSPENT|color=red";
else
  UNDERSPEND=$(echo "scale=2; $SPENDING_LIMIT - $CURRENT_OUTGOING" | bc);
  if [ "$(echo "$UNDERSPEND > ($SPENDING_LIMIT/2)" | bc)" -ne 0 ]; then
    echo "🏦 £$UNDERSPEND remaining|color=green";
  else
    echo "🏦 £$UNDERSPEND remaining|color=#ffbf00";
  fi;
fi;
echo "---";
echo "Current Account: £$CURRENT_BALANCE";
echo "Current Outgoing: £$CURRENT_OUTGOING";
echo "Last TX: $LAST_TRANSACTION";
```

![Tracking spending in the OSX Menu Bar](http://i.imgur.com/bv8TZLe.png)

## Installation

### From release

```
> curl -L https://github.com/sebinsua/teller-cli/releases/download/v0.0.7/teller > /usr/local/bin/teller && chmod +x /usr/local/bin/teller
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
