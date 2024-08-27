fn main() {
    let output = std::process::Command::new("typeshare")
        .arg(".")
        .arg("-l")
        .arg("typescript")
        .arg("-o")
        .arg("client/src/bindings.ts")
        .output()
        .expect("Failed to run typeshare");

    if !output.status.success() {
        panic!(
            "Typeshare failed: \n\n{:#?}\n\nDo you have it installed?",
            output
        );
    }
}
