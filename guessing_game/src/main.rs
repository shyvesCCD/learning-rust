use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");


    loop {
        let mut guess = String::new(); // Utilizando mut deixamos a nossa variável mutável.
        // Variáveis em rust são por padrão imutáveis. 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small!"),
            Ordering::Greater => println!("Your guess is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
