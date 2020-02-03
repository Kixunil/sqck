Sequoia check key
=================

Minimalistic PGP key verificator.

About
-----

This tool does only one job and does it well: it verifies and re-encodes a PGP key given on it's input. This is useful to avoid exploiting full-featured GPG setups using hypothetical vulnerabilities in
complex C implementation.

This is especially valuable in Qubes if you use advanced PGP setups or for software building.

Building
--------

**Please review the code! It's only 38 lines (including blank lines) long. Seriously!**

Because of the security requirements of this tool it's written in [Rust](https://rust-lang.org) using [Sequoia](https://sequoia-pgp.org) library.
Thus, you need Rust compiler and `cargo` in order to build it.
See [Rustup](https://rustup.rs) if your distribution doesn't have the appropriate packages.

`apt install cargo` should work on Debian 10.

Usage
-----

```bash
sqck "$FINGERPRINT" < untrusted-key.gpg > trusted-key.gpg || exit 1
# You may want to use this safety measure
chmod 000 untrusted-key.gpg || exit 1
gpg --import trusted-key.gpg
```

That's it. No other features/options/whatever. Spaces in `$FINGERPRINT` are ignored.

**Important: for maximum security, do NOT read untrusted-key.gpg by other applications
even after verification!** The idea is that the key is parsed and encoded again by `sequoia` library,
so that transferring any exploit is extremely unlikely. As another sanity check, no output is produced if the fingerprint doesn't match.

Contributions
-------------

I will be *very* careful merging any contributions.

* Correct security vulnerability fixes will be merged immediately.
* Code refactoring that decreases the amount of code (without moving it away) will likely be merged.
* Code adding features will not be merged unless it fixes a vulnerability.
* Refactoring increasing the code size will NOT be merged.
* New correct integration tests will likely be merged.

License
-------

MITNFA
