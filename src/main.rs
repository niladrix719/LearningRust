fn main() {
    let s = "64";
    let num: i32 = s.trim().parse().unwrap();
    println!("{}",num * 2);
}
