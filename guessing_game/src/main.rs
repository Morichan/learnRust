extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // $B?t;z$rEv$F$F$_$F!*(B
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // $BM=A[CM$rF~NO$7$F$h(B
        println!("Please input your guess.");

        let mut guess = String::new();

        // $B9T$NFI<h$j$K<:GT$7$?$h(B
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // $B$"$J$?$NM=A[CM$O$3$l(B: guess
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"), // $B>.$5$9$.$k!*(B
            Ordering::Greater => println!("Too big!"), // $BBg$-$9$.$k!*(B
            Ordering::Equal   => {
                println!("You win!"); // $B$"$J$?$N>!$A$@!*(B
                break;
            }
        }
    }
}
