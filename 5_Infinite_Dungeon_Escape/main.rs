use std::io; 
use std::io::Write;
use rand::Rng;

fn main(){

   let mut health = 100;
   let mut choice = String::new();
   loop{
     
     print!("Enter Your Choice : \n\n");
     io::stdin().read_line(&mut choice).expect("Failed to take input\n\n");
  
     if choice == "W" {
        print!("W , you lost your 30 hp\n\n");
        choice.clear();
     }
     else if choice == "A" {
        print!("IDK why but bcoz of my mood swings you lost 50 hp \n\n");
        health -= 50;
        choice.clear();
     }
     else if choice == "S"
     {
        print!("You lost , Life is unfair \n\n");
        health -= 100;
        choice.clear();
     }
     else if choice == "D"{
        print!("I dont know why , but you won\n\n");
        choice.clear();
        health = 200;
     }
     else if choice == "Q"{
        print!("You are stuck in the Dungeon 4 Ever\n\n");
        choice.clear();
        std::process::exit(0);
     }
   

     


   }

}
