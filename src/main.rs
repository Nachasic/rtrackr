mod xorg;
use xorg::*;

fn run () -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::open().ok_or_else(|| "Failed to get display")?;
    let root_window = display.get_default_root_window();

    let active_window = XNetActiveWindow::get_as_property(&display, &root_window)?;
    let title = XWMName::get_as_property(&display, &active_window)?;
    let (app_name, app_class) = XWMClass::get_as_property(&display, &active_window)?;

    println!("Title {:?}", title);
    println!("Application name {:?}", app_name);
    println!("Class {:?}", app_class);
    println!("Window ID {:?}", active_window.0);
    Ok({})

}

fn main () {
    match run() {
        Err(err) => println!("FATAL ERROR: {}", err),
        Ok(_) => println!("Done, goodbye!")
    };
}