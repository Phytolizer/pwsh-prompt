fn main() {
    let path = std::path::PathBuf::from(std::env::args().nth(1).unwrap());
    let git_output = std::process::Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(&path)
        .output()
        .unwrap();
    if git_output.status.success() {
        print!("{}", String::from_utf8(git_output.stdout).unwrap());
    }
}
