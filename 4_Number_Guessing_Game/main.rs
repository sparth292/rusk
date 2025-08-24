    use std::io;
    use rand::Rng;
    fn main() {
    
        let mut randNumber = rand::thread_rng().gen_range(1..=100); 
        println!("Number Guessing Game ! \n");
        println!("{}",randNumber);
        
        loop{
        
        println!("Enter your guess");
        let mut guess = String::new();
        
        std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

        
        if randNumber.to_string() == guess.trim() {

        println!(" You Guessed the correct number! ");
        break;

        }

        }
        
    }
