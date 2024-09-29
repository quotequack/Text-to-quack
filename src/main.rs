use std::io::{self, Write};
use rand::Rng;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Fail");

    let five = String::from("quack");
    let four = String::from("quak");
    let three = String::from("qak");
    let two = String::from("qu");

    let length = input.trim().len();
    if length == 0 {
        println!("Empty input. Exiting...");
        return;
    }

    let mut clp = 0;
    let mut p1 = String::new();
    
    while clp < length {
        let crn = rand::thread_rng().gen_range(1..=4);
        
        if crn == 1 {
            p1.push_str(&format!("{} ", five));
        } else if crn == 2 {
            p1.push_str(&format!("{} ", four));
        } else if crn == 3 {
            p1.push_str(&format!("{} ", three));
        } else if crn == 4 {
            p1.push_str(&format!("{} ", two));
        }

        clp = p1.len();
    }
    let result = String::from("{}\n-# Translation: {}, p1, input.trim");
    println!("{}", result);
    io::stdout().flush().unwrap();
}
