mod xorg;
mod window;

use xorg::*;
use std::{ thread, time };
use window::WindowInfo;

struct AppState {
    active_window_info: Option<WindowInfo>,
    last_mouse_position: (i32, i32),
    keys_were_pressed: bool,
    seconds_since_active: i32,
}

impl AppState {
    pub fn new() -> Self {
        Self{
            active_window_info: None,
            seconds_since_active: 0,
            last_mouse_position: (0, 0),
            keys_were_pressed: false,
        }
    }

    pub fn with_window_info(&mut self, info: &WindowInfo) -> &mut Self {
        self.active_window_info = Some(info.clone());
        self
    }

    pub fn updated_mouse_pos(&mut self, position: (i32, i32)) -> bool {
        if self.last_mouse_position != position {
            self.last_mouse_position = position;
            self.timer_reset();
            return true
        }
        false
    }

    pub fn updated_keys(&mut self, comb: Vec<u8>) -> bool {
        self.keys_were_pressed = comb.len() > 0;
        self.keys_were_pressed
    }

    pub fn tick (&mut self) {
        self.seconds_since_active += 1;
    }

    pub fn timer_reset(&mut self) {
        self.seconds_since_active = 0;
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
} 

fn report_change(info: &WindowInfo, afk_seconds: i32) {
    clear_screen();
    println!("Window title {:?}", info.title);
    println!("Appliction name {:?}", info.app_name);
    println!("Application class {:?}", info.app_class);
    println!("Window UID {:?}", info.uid);
    println!("Last action since {:?}", afk_seconds);
}

fn report_afk() {
    clear_screen();
    println!("AFK");
}

fn run(state: &mut AppState, display: &Display, root_window: u64) -> Result<(), Box<dyn std::error::Error>> {
    let active_window_uid = XNetActiveWindow::get_as_property(&display, root_window)?;
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
                state.timer_reset();
            }
        }
    }

    // Checking inputs
    let mouse = query_pointer(display, root_window);

    state.tick();
    state.updated_mouse_pos(mouse.coords);
    
    report_change(&active_window, state.seconds_since_active);
    Ok({})
}

fn main_loop() {
    let mut is_running = true;
    let sleep_duration = time::Duration::new(1, 0);
    let mut state = AppState::new();
    let display = Display::open().unwrap();
    let root_window = display.get_default_root_window();
    let mut cycle_start_time = time::SystemTime::now();
    let mut time_elapsed = time::Duration::new(0, 0);
    
    while is_running {
        let keys = query_keymap(&display);
        let current_time = time::SystemTime::now();

        time_elapsed = 
                current_time.duration_since(cycle_start_time).unwrap_or(
                    time::Duration::new(0, 0)
                );
        if state.updated_keys(keys) {
            state.timer_reset();
        }

        if time_elapsed > sleep_duration {
            match run(&mut state, &display, root_window) {
                Err(err) => {
                    println!("FATAL ERROR: {}", err);
                    is_running = false;
                },
                Ok(_) => { cycle_start_time = current_time },
            }
        }
    };
}

fn main() {
    main_loop();
    println!("Done, goodbye!");
}