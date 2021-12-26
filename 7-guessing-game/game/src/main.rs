use rand::Rng;
use read_input::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("数字を当ててください!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("予想した数字を入力してください。");

        let guess: Result<u32, _> = input().try_get();

        let guess: u32 = match guess {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想は{}です。", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます!"),
            Ordering::Greater => println!("大きすぎます!"),
            Ordering::Equal => {
                println!("あなたの勝ちです!");
                break;
            }
        }
    }
}
