# narwhalapi C library

`libnarwhalapi` is the C library for the [Narwhal API](https://narwhalapi.com): economics
(including the equity risk premium), foreign exchange, calendars, stocks, funds, crypto,
company calendars, company filings, palm oil and spot prices, and webhook delivery. It is a thin C ABI over the
Rust client (`rust/narwhalapi-ffi`). The opening comment of `c/include/narwhalapi.h` holds
the ownership rules and the return values of exports without a status.

Build from source (needs a Rust toolchain). Releases are tags: build from the tag, for
example `v0.2.0`, not from `main`.

```sh
make -C c pc          # builds rust/target/release/libnarwhalapi.{a,dylib,so} and build/pkgconfig
export PKG_CONFIG_PATH="$PWD/c/build/pkgconfig"
cc -std=c99 c/examples/quickstart.c $(pkg-config --cflags --libs narwhalapi) -o quickstart
NARWHAL_API_KEY=... ./quickstart
```

Static linking: on macOS, `pkg-config --static --libs narwhalapi` still links the dylib,
because the linker prefers `libnarwhalapi.dylib` when both files share a directory. To link
statically, name the archive instead of `-lnarwhalapi`:

```sh
cc app.c $(pkg-config --cflags narwhalapi) \
  "$(pkg-config --variable=libdir narwhalapi)/libnarwhalapi.a" -liconv   # Linux: -lpthread -ldl -lm
```

The shared library's install name is `@rpath/libnarwhalapi.dylib` (macOS) and its soname is
`libnarwhalapi.so.0` (Linux), so installed programs find it through the rpath in `Libs`.

Every `*_params` struct and `nw_config` starts with `size`: set it to `sizeof` the struct.
Call `nw_version_check(NW_VERSION_MAJOR, NW_VERSION_MINOR)` at startup.

Decimals are exact wire strings (`const char *`); never convert them to floating point.
Operations that are not yet in the live API are declared only with `-DNW_UNSTABLE`; there
are none today.

MIT licensed. [Narwhal API](https://narwhalapi.com).
