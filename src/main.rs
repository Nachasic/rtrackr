mod xorg;
use xorg::*;

fn main () -> Result<(), &'static(dyn std::error::Error)> {
    let display = Display::open().unwrap();
    let active_window = Window::active_window(&display).unwrap();

    let title = active_window.get_title(&display)
        .unwrap_or_else(|err| err.to_string());
    println!("Title {:?}", title);

    let (app_name, class) = active_window.get_name_and_class(&display)
        .unwrap_or_else(|err|
            (err.to_string(), String::from(""))
        );
    println!("Application name {:?}", app_name);
    println!("Class {:?}", class);

    Ok({})
}