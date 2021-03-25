fn main() {
    let path = std::env::args().nth(1).unwrap();
    let path = path[2..].replace('\\', "/");
    print!("{}", path);
}
