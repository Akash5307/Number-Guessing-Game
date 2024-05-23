use std::io;
use rand::Rng;
use colored::*;
fn main() {
    
    let secret_num=rand::thread_rng().gen_range(1,101);
    // println!("Secret Number is {}\n",secret_num);
    println!("{}","!!!Welcome to number Guessing Game !!!".yellow());
    loop{
        println!("{}","Please input your guess :".blue());
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("Failed to take input");

        let guess:u32=guess.trim().parse().expect("Please type a number");
        if guess==secret_num{
            print!("{}"," !!! You Won !!! \n".green());
            break;
        }else if guess<secret_num{
            print!("{}","Number is too small (:*:)\n".red());
        }else{
            print!("{}","Number is too large (:*:)\n".red());
        }
    }
}
