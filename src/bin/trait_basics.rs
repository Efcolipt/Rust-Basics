#![allow(unused)]

use std::fmt::format;

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait Test {
    fn test(&self, file_path: &str) -> String {
        format!("test {file_path}")
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {file_path}")
    }
}

impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("forge {file_path}")
    }
}

impl Test for Vyper {}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "1.2".to_string(),
    };

    let vy = Vyper {
        version: "0.4".to_string(),
    };

    println!("{:?}", compile(&sol, "file.sol"));
    println!("{:?}", compile(&vy, "file.vy"));

    println!("{:?}", sol.test("file.sol"));
    println!("{:?}", vy.test("file.vy"));
}
