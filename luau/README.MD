Rust bindings for [luau](https://github.com/luau-lang/luau) project

### Update
When updating the luau project, you need to re-generate bindings using the following commands:

#### Install [bindgen-cli](https://crates.io/crates/bindgen-cli)
```
cargo install bindgen-cli
```

#### Update the bindings
```shell
cd luau
bindgen luau/VM/include/lua.h -o src/lua.rs
```

#### Add for each rust file with bindings the prelude
```rust
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    dead_code,
    unused_imports,
    deprecated,
    clippy::all
)]
```