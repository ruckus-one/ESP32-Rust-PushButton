use anyhow::Result;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::{
    sync::{Arc, Mutex}, thread, time::Duration
};

mod push_button;
use push_button::{Button, ButtonState};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;

    let btn = Button::new(peripherals.pins.gpio0, Arc::new(Mutex::new(|state: ButtonState| {
        match state {
            ButtonState::Pressed => {
                println!("Pressed");
            },
            ButtonState::Released => {
                println!("Released");
            },
        }
    })));

    let _guard0 = btn.spawn_thread();

    loop {
        thread::sleep(Duration::from_millis(100));
    }
}
