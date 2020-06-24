mod xorg;
mod window;

use xorg::*;
use std::{ thread, time };
use window::WindowInfo;

struct AppState {
    active_window_info: Option<WindowInfo>
}

impl AppState {
    pub fn new() -> Self {
        Self{
            active_window_info: None
        }
    }

    pub fn with_window_info(&mut self, info: &WindowInfo) -> &mut Self {
        self.active_window_info = Some(info.clone());
        self
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
} 

fn report_change(info: &WindowInfo) {
    clear_screen();
    println!("Window title {:?}", info.title);
    println!("Appliction name {:?}", info.app_name);
    println!("Application class {:?}", info.app_class);
    println!("Window UID {:?}", info.uid);
}

fn run(mut state: &mut AppState, display: &Display, root_window: u64) -> Result<(), Box<dyn std::error::Error>> {
    let active_window_uid = XNetActiveWindow::get_as_property(&display, root_window)?;

    if active_window_uid == 50331651 {
        println!("DANGER");
    }

    let title = XWMName::get_as_property(&display, active_window_uid)?;
    let (app_name, app_class) = XWMClass::get_as_property(&display, active_window_uid)?;

    let active_window = WindowInfo::build(active_window_uid)
        .with_title(title)
        .with_app_name(app_name)
        .with_app_class(app_class);

    match &mut state.active_window_info {
        None => { state.with_window_info(&active_window); },
        Some(previous_active_window) => {
            if previous_active_window != &active_window {
                state.with_window_info(&active_window);
                report_change(&active_window);
            }
        }
    }
    Ok({})
}

fn main_loop() {
    let mut is_running = true;
    let sleep_duration = time::Duration::new(1, 0);
    let mut state = AppState::new();
    let display = Display::open().unwrap();
    let root_window = display.get_default_root_window();

    while is_running {
        match run(&mut state, &display, root_window) {
            Err(err) => {
                println!("FATAL ERROR: {}", err);
                is_running = false;
            },
            Ok(_) => thread::sleep(sleep_duration),
        }
    };
}

fn main() {
    main_loop();
    println!("Done, goodbye!");
}