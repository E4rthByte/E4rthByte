run package:
    cargo clean --package {{package}}
    cargo build -p e4rthbyte
    cargo run -p {{package}}