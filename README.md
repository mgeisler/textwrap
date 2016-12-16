# Textwrap

Textwrap is a small Rust crate for word wrapping strings. You can use
it to format strings for display in commandline applications.

Strings are wrapped based on their [displayed width][unicode-width],
not their size in bytes. This means that characters with accents and
other non-ASCII characters are handled correctly.

[unicode-width]: https://unicode-rs.github.io/unicode-width/unicode_width/index.html
