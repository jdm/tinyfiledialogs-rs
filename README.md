# tinyfiledialogs-rs

This is a high-level Rust binding to the excellent [tinyfiledialogs library](https://sourceforge.net/projects/tinyfiledialogs/)
by Guillaume Vareille. The source for the C library is included in
the `libtinyfiledialogs` directory to facilitate an all-in-one package
when using the Rust bindings via Cargo. It is updated sporadically from
the original repository, which should be used as the primary source for
all non-Rust users of the library.

To use this library, add this to the `dependencies` section in `Cargo.toml`:
```
tinyfiledialogs = "3.0"
```

## Security Warning

tinyfiledialogs should only be used with trusted input. Using it with
untrusted input, for example as dialog title or message, can in the worst
case lead to execution of arbitrary commands.
