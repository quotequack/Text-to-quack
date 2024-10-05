use rand::Rng;
use fltk::{app, button::Button, frame::Frame, input::Input, menu::Choice, prelude::*, window::Window};
#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut trans = 0;
    let mut input = String::new();
    let mut app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Dropdown Menu Example");
    let mut dropdown = Choice::new(100, 50, 200, 40, "Translation");
    dropdown.add_choice("None|Discord|Plain");
    let mut btn = Button::new(160, 50, 80, 40, "Update");
    let mut inputfield = Input::new(100, 150, 200, 40, "Input:");
    let mut btn2 = Button::new(160, 150, 80, 40, "Submit");
    let mut btn3 = Button::new(0, 0, 40, 150, "Generate");
    let mut label = Frame::new(0, 151, 40, 150, "Output");

    btn.set_callback(move |_| {
        trans = dropdown.value();
        println!("{}", trans);
    });
    btn2.set_callback(move |_| {
        input = inputfield.value();
    });

    let five = String::from("quack");
    let four = String::from("quak");
    let three = String::from("qak");
    let two = String::from("qu");

    btn3.set_callback(move |_| {
        let length = input.len();
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
        let result = match trans {
            1 => format!("{}\n-# Translation: {}", p1, input),
            2 => format!("{}\n{}", p1, input),
            0 => format!("{}", p1),
            _ => {
                println!("Input 1 or 0");
                return;
            }
        };
        label.set_label(&result);
    });
}