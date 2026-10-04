# Rocinante patch to Sled 0.34.7

This directory is the upstream `sled` crate version 0.34.7, copied from the
official crates.io package. The source package checksum recorded by Cargo is
`7f96b4737c2ce5987354855aed3797279def4ebf734436c6aa4552cf8e169935`.
The upstream package identity (`sled` 0.34.7), repository, authors, and MIT OR
Apache-2.0 licensing are preserved.

Rocinante uses this copy only in the isolated legacy-data migration utility.
The patch removes `fxhash` and uses the standard library's randomized map and
set hasher for in-memory bookkeeping. It also moves `parking_lot` from 0.11.2
to 0.12.1, avoiding the unmaintained `instant` dependency. Neither change
alters Sled's on-disk format. The migration utility reads a copied store and
never includes this package in application dependency graphs. An explicit
allow for `unused_qualifications` accounts for the stricter Rust 1.99 lint on
unchanged upstream source; the other upstream lint denials remain enabled.
Rust 1.99 deprecation notices for the pinned implementation, one cfg-specific
unused import, and an upstream redundant semicolon are allowed only within the
vendored Sled crate. Dead code for its unsupported optional features and
platform-specific helpers is also allowed only in that crate; strict clippy
checks on the migration code remain enabled.

The migrator pins this format version and treats Sled's reserved
`__sled__default` tree name as the default keyspace, which Sled also exposes
through `Db` dereferencing. The separate tree-kind column preserves named
trees byte-for-byte, including a named tree with an empty name.
