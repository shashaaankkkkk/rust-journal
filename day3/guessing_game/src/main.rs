use std::io;

fn main() {
    println!("guessing game");
    println!("please enter your input");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("failed to get input");
    println!("your guess {guess}");
}
