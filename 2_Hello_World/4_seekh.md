# Kya Seekhne mila mujhe is directory se

## In hello_world directory 

create a main.rs file and write the code given in this folder
(Do the same for practice.rs)

Baadme Save the file and run these commands

rustc main.rs
./main

After running those commands you'll get an output as "Hello World" (Output obviously Depends on your code)

If Hello, world! did print, congratulations! 
You’ve officially written a Rust program. 
That makes you a Rust programmer Welcome to the club!

## Now let's Review our code in detail (main.rs)

Here’s the first piece of the puzzle:

Ye jo function hai ye har bar execution isse hi chalu hoga jaisa cpp me hota tha

fn main() {

}

The function body is wrapped in {}. 
Rust requires curly brackets around all function bodies. 
It’s good style to place the opening curly bracket on the same line as the function declaration, 
adding one space in between.

println!("Hello, world!");

This line does all the work in this little program: 
it prints text to the screen. There are three important details to notice here.
First, println! calls a Rust macro.
If it had called a function instead, it would be entered as println (without the !). 
Rust macros are a way to write code that generates code to extend Rust syntax,
and we’ll discuss them in more detail in Chapter 20. 
For now, you just need to know that using a ! means that you’re calling a macro 
instead of a normal function and that macros don’t always follow the same rules as functions.

If you don't know what rust macro is let me explain it to you in a simpler way.
It is kind of like a function, but it's more powerful because it works at the code generation level. 
So instead of just running like a function, it actually expands into code at compile time. 
This means you can do things that regular functions can't do, like generating repetitive code automatically, 
creating domain-specific syntax, or even modifying how input expressions are interpreted—basically, 
macros let you write code that writes code, which functions alone can’t do.

### Here's an example of a rust macro 

macro_rules! say_hola
{

() => {
  
  println!("Hola Amigo , Kaise ho theek ho");
  println!("Dies out of cringe after printing that line");

};

}

#### Google ki definition 
Rust macros are powerful metaprogramming tool that allows you to write code 
that generates other code during compilation

#### Abhi what is Metaprogramming in the context of rust 
(Woh google karle merepe jyada time nahi hai aur mujhe ye doc chhoti aur understandable rakhni hai)

