fn main() {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    println!("{}", s);
    println!("If this is insufficient, come up with a passphrase based on a easily-remebered sentence no-spaced.");
}
