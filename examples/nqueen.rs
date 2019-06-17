use std::process::Command;

fn run(filepath: &str) {
    // compile
    Command::new("sh")
        .arg("-c")
        .arg(format!("./target/debug/rexparser \"{}\"", filepath))
        .status()
        .expect("process failed to execute");

    // run generated IR and get returned status code
    let output = Command::new("sh")
        .arg("-c")
        .arg("llvm-link -S -o runnable.ll compiled.ll builtin.ll; llvm-as runnable.ll; lli runnable.bc")
        .output()
        .expect("failed to execute process");

    let stdout_string = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout_string);
}

fn main() {
    let filepath = "./tests/resources/test_nqueen.c";
    run(filepath);
}
