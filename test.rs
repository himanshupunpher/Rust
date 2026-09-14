extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Whats up Motherfuckers! Welcome to Guessing your ass number game");
    
    let secret_number = rand::thread_rng().gen_range(1, 11);

    let mut guess = String::new();

    println!("Enter a Number Bitch!!");

    io::stdin().read_line(&guess)
        .expect("What the fuck did you do nigga!!");

    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You Entered {} Asshole", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Your dick is small!! HAAAAAA HAAAAA"),
        Ordering::Greater => println!("Entered a bigger number Nigger try smaller"),
        Ordering::Equal => println!("Finally a Premium Nigga"),
    };
}
