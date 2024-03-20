use std::process::Command;

fn main() {
    polkit();
}

fn polkit() {
    let which = Command::new("which")
        .arg("aa-caller")
        .output()
        .expect("failed to execute process");
    if !which.status.success() {
        panic!("{}", String::from_utf8(which.stderr).unwrap());
    }
    let bin_path = String::from_utf8(which.stdout).unwrap();
    Command::new("sed")
    .arg(format!("'s/@BIN_DIR@/{}/g'", bin_path.replace("/", r"\/")))
    .arg("resources/pkexec.policy.in")
    .arg(">")
    .arg("/usr/share/polkit-1/actions/com.github.jack-ullery.AppAnvil.pkexec.policy");
}
