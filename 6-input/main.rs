use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    println!("buf = {}", buf);

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let num: i32 = buf.trim().parse().unwrap();
    println!("num = {}", num);
}
