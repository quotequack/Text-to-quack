use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Input starter string");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    println!("Input '1' for discord translation on, '2' for plain tranlation on and '0' for off");
    let mut trans: String = String::new();
    io::stdin().read_line(&mut trans).expect("Fail");

    let trans_num: u32 = match trans.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number");
            return;
        },
    };

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

    match trans_num {
        1 => println!("{}\nTranslation: {}", p1, input.trim()),
        2 => println!("{}\n{}", p1, input.trim()),
        0 => println!("{}", p1),
        _ => println!("Input 1 or 0"),
    }

    io::stdout().flush().unwrap();
}