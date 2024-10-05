use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use fltk::{app, button::Button, frame::Frame, input::Input, menu::Choice, prelude::*, window::Window};

fn main() {
    let mut trans = 0;
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Dropdown Menu Example");

    // Dropdown and Update Button
    let mut dropdown = Choice::new(100, 40, 200, 30, "Translation:");
    dropdown.add_choice("None|Discord|Plain");
    let mut btn = Button::new(320, 40, 60, 30, "Update");

    // Input Field and Submit Button
    let inputfield = Input::new(100, 100, 200, 30, "Input:");
    let mut btn2 = Button::new(320, 100, 60, 30, "Submit");

    // Generate Button and Output Label
    let mut btn3 = Button::new(100, 160, 80, 40, "Generate");
    let mut label = Frame::new(200, 160, 180, 40, "Output");

    // Wrap input in Rc<RefCell> to allow shared, mutable access
    let input = Rc::new(RefCell::new(String::new()));

    btn.set_callback(move |_| {
        trans = dropdown.value();
        println!("{}", trans);
    });

    let input_clone = Rc::clone(&input); // Clone Rc for btn2
    btn2.set_callback(move |_| {
        *input_clone.borrow_mut() = inputfield.value();
    });

    let five = String::from("quack");
    let four = String::from("quak");
    let three = String::from("qak");
    let two = String::from("qu");

    let input_clone = Rc::clone(&input); // Clone Rc for btn3
    btn3.set_callback(move |_| {
        let input_value = input_clone.borrow(); // Borrow input value immutably
        let length = input_value.len();
        if length == 0 {
            println!("Empty input. Exiting...");
            return;
        }

        let mut clp = 0;
        let mut p1 = String::new();

        // Adjusted loop to handle full input
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

            clp = p1.chars().count();  // Track current character length
        }

        let result = match trans {
            1 => format!("{}\n-# Translation: {}", p1, input_value),
            2 => format!("{}\n{}", p1, input_value),
            0 => format!("{}", p1),
            _ => {
                println!("Input 1 or 0");
                return;
            }
        };

        label.set_label(&result);  // Set label to the full result
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
