fn main() {
    println!("g: {}", std::env::var("GLOBAL").unwrap());
    println!("l: {}", std::env::var("LOCAL").unwrap());
}
