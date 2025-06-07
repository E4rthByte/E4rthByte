build package *FLAGS:
    cargo clean --package {{package}}
    cargo build -p e4rthbyte {{FLAGS}}
    cargo build -p {{package}} {{FLAGS}}

run package *FLAGS: (build package FLAGS)
    cargo run -p {{package}} {{FLAGS}}