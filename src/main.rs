mod xorg;

use xorg::*;

fn main () -> Result<(), Null> {
    let mut session = Session::open()?;
    let active_window = Window::active_window(&mut session).expect("could not get actuive window");
    let title = active_window.get_title(&session.display)?;
    println!("{:?}", title);
    Ok({})
}