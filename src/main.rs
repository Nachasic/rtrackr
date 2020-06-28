mod xorg;
mod window;
mod state;
mod tui;
mod event;

use xorg::*;
use std::{ 
    time,
    io::{ stdin }
};
use window::WindowInfo;
use state::AppState;
use event::*;
use tokio::*;

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
} 

// fn report_change(title: &String, name: &String, class: &String, uid: u64, afk_seconds: u64) {
//     clear_screen();
//     println!("Window title {:?}", title);
//     println!("Appliction name {:?}", name);
//     println!("Application class {:?}", class);
//     println!("Window UID {:?}", uid);
//     println!("Last action since {:?}", afk_seconds);
// }

// fn report_afk() {
//     clear_screen();
//     println!("AFK");
// }

fn update_window_info(state: &mut AppState, display: &Display, root_window: u64) -> Result<(), Box<dyn std::error::Error>> {
    let active_window_uid = XNetActiveWindow::get_as_property(&display, root_window)?;
    let title = XWMName::get_as_property(&display, active_window_uid)?;
    let (app_name, app_class) = XWMClass::get_as_property(&display, active_window_uid)?;

    let active_window = WindowInfo::build(active_window_uid)
        .with_title(title)
        .with_app_name(app_name)
        .with_app_class(app_class);

    state.updated_window_info(&active_window);
    Ok({})
}


async fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::new();

    let mut is_running = true;
    let mut cycle_start_time = time::SystemTime::now();
    let mut time_elapsed = time::Duration::new(0, 0);
    let sleep_duration = time::Duration::new(1, 0);

    let display = Display::open().unwrap();
    let root_window = display.get_default_root_window();

    let events = Events::with_config(EventConfig::default());

    clear_screen();
    let mut tui = tui::Tui::new()?;

    while is_running {
        let keys = query_keyboard(&display);
        let mouse = query_mouse_pointer(&display, root_window);
        let current_time = time::SystemTime::now();

        time_elapsed = 
                current_time.duration_since(cycle_start_time).unwrap_or(
                    time::Duration::new(0, 0)
                );
        
        state.updated_keys(keys);
        state.updated_mouse_info(&mouse);

        if time_elapsed > sleep_duration {
            update_window_info(&mut state, &display, root_window)?;
            cycle_start_time = current_time;
        }

        if let Ok(event) = events.next() {
            match event {
                Event::Input(key) =>
                    match key {
                        Key::Ctrl('c') => is_running = false,
                        _ => {}
                    }
                Event::Tick => tui.draw()?,
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
}
