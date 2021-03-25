fn main() {
    let now = chrono::Local::now();
    print!("{}", now.format("%Y-%m-%d %H:%M"))
}
