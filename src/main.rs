mod xorg;
mod window;
mod state;
mod tui;
mod event;
mod window_manager;
mod record_store;

use std::{ 
    time,
};
use window::WindowInfo;
use state::AppState;
use event::*;
use window_manager::{ OSWindowManager };
use xorg::{ XORGWindowManager };

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn update_window_info<T>(wm: &T, state: &mut AppState) -> Result<(), Box<dyn std::error::Error>>
where T: OSWindowManager  {
    let active_window = wm.get_window_info()?;

    state.updated_window_info(&active_window);
    Ok({})
}


async fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::new();

    #[cfg(any(target_os = "linux"))]
    let wm = XORGWindowManager::default();

    let mut is_running = true;
    let mut cycle_start_time = time::SystemTime::now();
    let mut time_elapsed: time::Duration;
    let sleep_duration = time::Duration::new(1, 0);

    let events = Events::with_config(EventConfig::default());

    clear_screen();
    let mut tui = tui::Tui::new()?;

    while is_running {
        let keys = wm.query_keyboard();
        let mouse = wm.query_mouse_pointer();
        let current_time = time::SystemTime::now();

        time_elapsed = 
                current_time.duration_since(cycle_start_time).unwrap_or(
                    time::Duration::new(0, 0)
                );
        
        state.updated_keys(keys);
        state.updated_mouse_info(&mouse);

        if time_elapsed > sleep_duration {
            update_window_info(&wm, &mut state)?;
            cycle_start_time = current_time;
        }

        if let Ok(event) = events.next() {
            match event {
                Event::Input(key) =>
                    match key {
                        Key::Ctrl('c') => is_running = false,
                        _ => {}
                    }
                Event::Tick => tui.draw(&mut state)?,
            }
        }
    };
    Ok({})
}

#[tokio::main]
async fn main() {
    match main_loop().await {
        Err(err) => println!("FATAL ERROR: {}", err),
        Ok(_) => {}
    };
    println!("Done, goodbye!");
    clear_screen();
}
