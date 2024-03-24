use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("\n \
            Choose command:\n \
            1 - start Guess number game;\n \
            2 - exit;\n \
        ");
        
        let mut command = String::new();
        
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        
        match command.trim().as_ref() {
            "1" => start_game(),
            "2" => break,
            _ => println!("Invalid command try again."),
        }
    }
}

fn start_game() {
    println!("Guess the number!");
        
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");  

    loop {
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YOU WIN!!!");
                break
            }
        }
    }
}
