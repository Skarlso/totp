# TOTP CLI

A cli TOTP token generator based on RFC-6238.

# Security

This CLI never stores a password. It always asks for one. The password is used to manage multiple accounts and tokens via an AES encrypted account file located next to the binary.

# Commands

```bash
❯ totp help
totp 1.0
Gergely Brautigam
TOTP Token generator on the command line with AES encrypted account handling.

USAGE:
    totp [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         Adds a new account with a TOTP token.
    delete      Delete a given account.
    generate    Generate a new token for a given account.
    help        Prints this message or the help of the given subcommand(s)
```

# Example

First, create an account by using the provided BASE32 encoded TOTP identifier.

```bash
❯ totp add
account:
gmail_main
token:
MFZWIZQASDFFSFDSIJAAA=
Password:
```

Once this is done, you are ready to use the generator to generate a 6 digit token which changes every 30 seconds.

```bash
❯ totp generate -a gmail_main
Password:
364898
```

You can create an alias for that saying something like `gmail_main_token`.

# Tricks

On linux, this also adds the ability to directly pipe the resulting token to clipboard. Ready to be pasted into something else.

```bash
gmail_main_token|pbcopy
Password:
```

Then simply press {CTRL,CMD}+C and voila...

```bash
452987
```

# Compliance to RFC

This generator is in full compliance to the RFC described here: RFC-6238.
Note: With the expection that right now times, and methods are not configurable. That is in the ROADMAP.

# Unit Tests

I know, they are missing. And in the works. I'm still learning Rust.

# Contributions

Are welcomed.
