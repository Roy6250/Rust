// By default variables are immutable in Rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {

    println!("Hello, world!");
    

    let secret_number= rand::thread_rng().gen_range(1, 100);
    println!("The secret number: {}", secret_number);

   
    loop{

        println!("Please input your guess");

         // mut Keyword makes the variable mutable
        let mut guess= String::new(); 


        //Taking input
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // let guess:u32= guess.trim().parse().expect("Please type a number");
        let guess:u32= match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) => continue,
        };

        // Here we are declaring "guess" variable once as string and other time as integer
        // This is known as shadowing

        println!("You have guess: {}", guess);


        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","too small".red()),
            Ordering::Equal => {
                println!("{}","Guessed Correctly!!!".green());
                break;
            },
            Ordering::Greater => println!("{}","too big".red()),
        }

    }

    


}