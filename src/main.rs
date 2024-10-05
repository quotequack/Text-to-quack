use enigo::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use rand::Rng;
use fltk::{app, button::Button, frame::Frame, input::Input, menu::Choice, prelude::*, window::Window};

fn main() {
    let trans = Rc::new(RefCell::new(2));
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Dropdown Menu Example");
    let mut dropdown = Choice::new(100, 40, 200, 30, "Translation:");
    dropdown.add_choice("None|Discord|Plain");
    let mut btn = Button::new(320, 40, 60, 30, "Update");
    let inputfield = Input::new(100, 100, 200, 30, "Input:");
    let mut btn2 = Button::new(320, 100, 60, 30, "Submit");

    let mut btn3 = Button::new(100, 160, 80, 40, "Generate");
    let mut label = Frame::new(200, 160, 180, 40, "Output");
    let input = Rc::new(RefCell::new(String::new()));

    let trans_clone = Rc::clone(&trans);
    btn.set_callback(move |_| {
        let trans_value: i32 = *trans_clone.borrow();
        *std::cell::RefCell::<i32>::borrow_mut(&trans_clone) = dropdown.value();
        println!("{}", trans_value);
    });
    let input_clone = Rc::clone(&input); // Clone Rc for btn2
    btn2.set_callback(move |_| {
        *std::cell::RefCell::<String>::borrow_mut(&input_clone) = inputfield.value();
    });

    let five = String::from("quack");
    let four = String::from("quak");
    let three = String::from("qak");
    let two = String::from("qu");

    let input_clone = Rc::clone(&input);
    let trans_clone = Rc::clone(&trans);
    btn3.set_callback(move |_| {
        let input_value = input_clone.borrow();
        let length = input_value.len();
        let trans_value = *trans_clone.borrow();
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


            clp = p1.chars().count();
        }
        let result = match trans_value {
            1 => format!("{}\n-# Translation: {}", p1, input_value),
            2 => format!("{}\n{}", p1, input_value),
            0 => format!("{}", p1),
            _ => {
                println!("Input 1 or 0");
                return;
            }
        };
        println!("{}", result);
        label.set_label(&result);
        thread::sleep(Duration::from_secs(3));
        typewriter(&result);
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}

fn typewriter(string_to_type: &str) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.delay(30);
    enigo.text(string_to_type);
}