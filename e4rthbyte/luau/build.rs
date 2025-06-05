fn main() {
    let mut builder = base_builder();

    builder
        .include("luau/Compiler/include")
        .include("luau/VM/include")
        .include("luau/Common/include")
        .include("luau/Ast/include");

    builder
        .define("LUACODE_API", r#"extern "C""#);

    builder
        .files_by_glob_pattern("luau/Compiler/src/*.cpp")
        .files_by_glob_pattern("luau/VM/src/*.cpp")
        .files_by_glob_pattern("luau/Common/src/*.cpp")
        .files_by_glob_pattern("luau/Ast/src/*.cpp");

    builder.compile("luau");
}

fn base_builder() -> cc::Build {
    let mut builder = cc::Build::new();

    if builder.get_compiler().is_like_msvc() {
        builder.flag_if_supported("/std:c++17");
    } else {
        builder.flag_if_supported("-std=c++17");
    }
    
    builder
        .cpp(true)
        .warnings(false)
        .flag_if_supported("/std=c++17")
        .include("luau/Common/include");
    
    builder
}

trait BuildExt {
    fn files_by_glob_pattern<S>(&mut self, pattern: S) -> &mut Self
    where S: AsRef<str>;
}

impl BuildExt for cc::Build {
    fn files_by_glob_pattern<S>(&mut self, pattern: S) -> &mut Self
    where
        S: AsRef<str>
    {
        for entry in glob::glob(pattern.as_ref()).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {self.file(path);}
                Err(err) => println!("cargo:warning=Failed to read path: {}", err)
            }
        }
        
        self
    }
}