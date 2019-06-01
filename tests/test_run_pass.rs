use std::process::Command;

fn run(filepath: &str, expect: &str) {
    // compile
    Command::new("sh")
        .arg("-c")
        .arg(format!("./target/debug/rexparser \"{}\"", filepath))
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

#[test]
fn test_single_num() {
    let filepath = "./tests/resources/test_single_num.c";
    run(filepath, "1");
}

#[test]
fn test_add() {
    let filepath = "./tests/resources/test_add.c";
    run(filepath, "3");
}
