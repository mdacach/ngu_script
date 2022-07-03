use std::{thread, time};

use rdev::{listen, Event, EventType, Key};

use crate::menu::Menu;

mod coords;
mod input;
mod inventory;
mod menu;

fn main() {
    thread::spawn(|| loop {
        menu::navigate(Menu::Inventory);
        inventory::merge_equips();
        inventory::boost_equips();
        for id in 0..24 {
            inventory::merge_slot(id);
            inventory::boost_slot(id);
        }
        menu::navigate(Menu::Adventure);
        println!("Checking on some monsters...");
        thread::sleep(time::Duration::from_secs(10));
    });

    if let Err(e) = listen(check_for_user_termination) {
        println!("Error listening to events: {:?}", e);
    }
}

fn check_for_user_termination(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::KeyZ) => {
            println!("Terminating due to user input.");
            // This hangs the working thread, but OK for now.
            std::process::exit(0);
        }
        EventType::KeyPress(other_key) => {
            println!("{:?} pressed.", other_key);
        }
        EventType::ButtonPress(button) => {
            println!("{:?} pressed.", button);
        }
        EventType::KeyRelease(other_key) => {
            println!("{:?} released.", other_key);
        }
        EventType::ButtonRelease(button) => {
            println!("{:?} released.", button);
        }
        _ => (),
    }
}
