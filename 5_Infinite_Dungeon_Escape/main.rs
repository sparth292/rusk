// Now this is the Plan

// Now i need to do Something which will assign the actions randomly to option keys and then i can compare it to a switch case so this is what i am going to implement: 

// I  will have a list called keys which will have these options
// W
// A
// S
// D 

// and 

// I have another list called actions with these elements

// lose30 , 
// add 10 , 
// instant win , 
// add 20 ,
// lose 20 ,
// instant lose 

// So what i want to do here is basically 
// randomize this whole thing
// like in first selection round it will be 

// W - lose30 
// A - add10
// S - instant lose 
// D - instant win 

// on second selection 

// W - instant win 
// A - add 10 
// S - add 20 
// D - lose 10

use std::hash::BuildHasherDefault;
use std::io; 
use std::io::Write;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main(){
   let keys = !vec[
      "W",
      "A",
      "S",
      "D",
   ];
   let actions = !vec[
      "lose10",
      "lose20",
      "lose30",
      "add10",
      "add20",
      "add30",
      "instantWin",
      "instantLose",
   ];
   
   let mut health = 100;
   let mut rng = rand::thread_rng();

   println!("Welcome to the Infinite Dungeon Escape!");
   println!("Rules:");
   println!("You have 100 health.");  
   println!("You can move with W, A, S, D.");
   println!("You can see your health with H.");
   println!("You can quit with Q.");
   println!("Good luck!");
   println!("");

   loop{
      if(health <= 0){
         println!("You have lost the game!");
         break;
      }
   }
   
}
