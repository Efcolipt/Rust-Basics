#![allow(unused)]

trait Compiler {
    fn name(&self) -> String;
}

trait Language {
    fn compile(&self, file_path: &str) -> String;
}

trait CompiledLanguage: Language + Compiler {
    fn exec(&self, file_path: &str) {
        let name = self.name();
        println!("name: {name}");

        let cmd = self.compile(file_path);
        println!("cmd: {cmd}")
    }
}

struct Rust;

impl Compiler for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}

impl Language for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}

impl CompiledLanguage for Rust {}

fn main() {
    // Super trait
    let rust = Rust;

    rust.exec("file_path");
}
