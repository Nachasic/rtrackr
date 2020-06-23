mod xorg;
use xorg::*;

fn main () -> Result<(), Null> {
    let mut session = Session::open()?;
    let active_window = Window::active_window(&mut session).unwrap();

    let title = active_window.get_title(&session.display)?;
    println!("Title {:?}", title);

    let (app_name, class) = active_window.get_name_and_class(&session.display)?;
    println!("Application name {:?}", app_name);
    println!("Class {:?}", class);
    Ok({})
}