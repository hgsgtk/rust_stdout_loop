use std::env;

fn main() {
    let expletive = env::args().nth(1).unwrap_or("y".into());
    loop {
        println!("{}", expletive);
    }
}
