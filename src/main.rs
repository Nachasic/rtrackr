mod event;
mod record_store;
mod state;
mod tui;
mod window_manager;
mod xorg;
mod classifier;
mod constants;

use event::*;
use state::AppState;
use std::time;
use window_manager::OSWindowManager;
use xorg::XORGWindowManager;
use crate::tui::*;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

fn update_window_info<T>(wm: &T, state: &mut AppState) -> Result<(), Box<dyn std::error::Error>>
where
    T: OSWindowManager,
{
    let active_window = wm.get_window_archetype();

    state.update_window_info(active_window)?;
    Ok({})
}

async fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::new()?;

    #[cfg(any(target_os = "linux"))]
    let wm = XORGWindowManager::default();

    let mut is_running = true;
    let mut cycle_start_time = time::SystemTime::now();
    let mut time_elapsed: time::Duration;
    let sleep_duration = time::Duration::new(1, 0);

    let events = Events::with_config(EventConfig::default());
    let mut tui = tui::Tui::new(&state)?;

    tui.clear()?;

    while is_running {
        let keys = wm.query_keyboard();
        let mouse = wm.query_mouse_pointer();
        let current_time = time::SystemTime::now();

        time_elapsed = current_time
            .duration_since(cycle_start_time)
            .unwrap_or(time::Duration::new(0, 0));

        state.update_keys(keys);
        state.update_mouse_info(&mouse);

        if time_elapsed > sleep_duration {
            update_window_info(&wm, &mut state)?;
            tui.tick(&state);
            cycle_start_time = current_time;
        }

        if let Ok(event) = events.next() {
            match event {
                Event::Input(key) => match key {
                    Key::Ctrl('c') => is_running = false,
                    _ => {}
                },
                Event::Tick => {
                    tui.draw()?;
                },
            }
        }
    }
    Ok({})
}

#[tokio::main]
async fn main() {
    match main_loop().await {
        Err(err) => println!("FATAL ERROR: {}", err),
        Ok(_) => {}
    };
    println!("Done, goodbye!");
}

