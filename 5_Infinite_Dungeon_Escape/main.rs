use std::hash::BuildHasherDefault;
use std::io; 
use std::io::Write;
use rand::Rng;

fn main(){

   let mut health = 100;
   let mut choice = String::new();
   loop{
     if health <= 0 {
            println!("GAME OVER! You died with {} HP", health);
            break;
     }
     print!("Enter Your Choice : \n\n");
     io::stdin().read_line(&mut choice).expect("Failed to take input\n\n");
  
     if choice.trim() == "W" {
        print!("W , you lost your 30 hp\n\n");
        health -= 30;
        choice.clear();
     }
     else if choice.trim() == "A" {
        print!("IDK why but bcoz of my mood swings you lost 50 hp \n\n");
        health -= 50;
        choice.clear();
     }
     else if choice.trim() == "S"
     {
        print!("You lost , Life is unfair \n\n");
        choice.clear();
        break;
     }
     else if choice.trim() == "D"{
        print!("I dont know why , but you won\n\n");
        choice.clear();
        health = 200;
     }
     else if choice.trim() == "Q"{
        print!("You are stuck in the Dungeon 4 Ever\n\n");
        choice.clear();
        break;
     }
     else{
        print!("Invalid Choice \n\n");
        choice.clear();
     }
   

     // Aage proceed hoga


   }

}
