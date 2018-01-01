extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // 数字を当ててみて！
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // 予想値を入力してよ
        println!("Please input your guess.");

        let mut guess = String::new();

        // 行の読取りに失敗したよ
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // あなたの予想値はこれ: guess
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"), // 小さすぎる！
            Ordering::Greater => println!("Too big!"), // 大きすぎる！
            Ordering::Equal   => {
                println!("You win!"); // あなたの勝ちだ！
                break;
            }
        }
    }
}
