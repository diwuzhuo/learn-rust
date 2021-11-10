use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let secret_word = rand::thread_rng().gen_range(1..101);
    println!("Generate word is {}", secret_word);

    loop {
        let mut guess_word = String::new();
        println!("input the word:");
        io::stdin().read_line(&mut guess_word)
            .expect("Failed to read_line");
        let guess_word : u32 = match guess_word.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input error!");
                continue;
            }
        };
        println!("Your guess word is {}", guess_word);
        
        match guess_word.cmp(&secret_word) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }    
    }
}
