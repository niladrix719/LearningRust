use std::io;

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("failed to read line");
    println!("{}",string);
}
