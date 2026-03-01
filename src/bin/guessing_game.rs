use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Find the secret number!!");

    let secret_number = rand::rng().random_range(1..100);

    loop {
        println!("Please input one number!");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("Error");

        let guess : u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,  
        };
    
        println!("You typed this number: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Oh no try again this number is so tiny"),
            Ordering::Greater => println!("Oh no try again this number is so big"),
            Ordering::Equal => {
                println!("That's it you typed the correct number!");
                break;
            },
        }   
    }
}