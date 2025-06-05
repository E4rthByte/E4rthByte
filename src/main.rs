use luau::luacode::luau_compile;

fn main() {
    let code = r#"
    print("some")
    "#;
    dbg!(&luau_compile(code));
}
