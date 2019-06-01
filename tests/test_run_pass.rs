use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn run(input: &str, expect: &str) {
    // compile
    Command::new("sh")
        .arg("-c")
        .arg(format!("./target/debug/rexparser \"{}\"", input))
        .status()
        .expect("process failed to execute");

    // run generated IR and get returned status code
    let status = Command::new("sh")
        .arg("-c")
        .arg("llvm-as compiled.ll; lli compiled.bc")
        .status()
        .expect("failed to execute process");

    println!("{:?} => {:?}", status.to_string(), expect);
    assert!(status.to_string() == String::from(format!("exit code: {}", expect)));
}

fn get_code(filename: &str) -> String {
    let filename = String::from("./tests/resources/") + filename;
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("somethig went wrong reading the file");
    contents
}

#[test]
fn test_single_num() {
    let code = get_code("test_single_num.c");
    run(&code, "1")
}
