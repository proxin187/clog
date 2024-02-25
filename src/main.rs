mod xlib;

use xlib::Display;

use std::process;

fn main() {
    let mut display = match Display::open() {
        Ok(display) => display,
        Err(err) => {
            println!("[ERROR] failed to open display: {}", err.to_string());
            process::exit(1);
        },
    };

    loop {
        display.grab_input();

        if let Some(event) = display.poll_event() {
            match unsafe { event.type_ } {
                x11::xlib::KeyPress => {
                    unsafe {
                        println!("key: {}", display.keycode_to_string(event.key.keycode as u8));
                    }
                },
                _ => {},
            }
        }
    }
}

