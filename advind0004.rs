use std::io::{self, Write};
fn main() {
    println!("Hello, world!");
    print!("Naciśnij Enter, aby zakończyć...");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}