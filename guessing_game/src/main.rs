/*
Number guessing game from rust tutorial
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

Author: Josh Dowling
Created: 3/1/2023
Last updated: 10/1/2023 
*/

use std::io; //Define namespace?
use rand::Rng; //Maybe these are more like including libraries 
use std::cmp::Ordering; 

fn main() {
    //Print opening message 
    println!("Guess the number!");

    //Generate secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);  //thread rng is a function of the rand namespace so use ::
    //println!("The secret number is: {}", secret_number);

    let mut count = 0; 

    //Infinite Loop
    loop {
        println!("Please enter your number: "); 
        //create a mutable variable string called guess
        //What are the gray highlights? 
        let mut guess = String::new(); //how does the ::new work? - similar to string.new from c (class methods)? - :: is for a type or module on left and . is for a value or (object in C terms)

        //take input from user with IO stream
        io::stdin() //this is broken up into 3 lines to make it easier to read
            .read_line(&mut guess) //read line from stdin and pass guess as a mutable reference
            .expect("Failed to read line"); //fail condition 
    
        let guess: u32 = match guess.trim().parse() { //convert to number, trim removes new line characters and whitespace and parse allows the conversion of a string to another type 
            Ok(num) => num, //parse will return a result which was handled by except which crashes the program
            Err(_) => { //now its handled by a match statement which continues if an err is returned 
                println!("Please enter a number!");
                continue;
            }
        }; 
    
        println!("You guessed: {}", guess); //display guess 

        count += 1; 

        //compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                println!("Guess counter = {}", count);
                break; 
            }
        }
        println!("Guess counter = {}", count);
    }
}

//Added for testing purposes