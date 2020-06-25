
use x11::xlib::*;
use std::slice;

use crate::Display;

pub struct MouseState {
    pub coords: (i32, i32),
    pub button_pressed: Vec<bool>
}

pub fn query_pointer(display: &Display, root_window: Window) -> MouseState {
    let mut root_x = 0;
    let mut root_y = 0;
    let mut win_x = 0;
    let mut win_y = 0;
    let mut root_return = 0;
    let mut child_return = 0;
    let mut mask_return = 0;
    unsafe {
        XQueryPointer(
            display.0,
            root_window,
            &mut root_return,
            &mut child_return,
            &mut root_x,
            &mut root_y,
            &mut win_x,
            &mut win_y,
            &mut mask_return,
        );
    }
    let button1pressed = mask_return & Button1Mask > 0;
    let button2pressed = mask_return & Button2Mask > 0;
    let button3pressed = mask_return & Button3Mask > 0;
    let button4pressed = mask_return & Button4Mask > 0;
    let button5pressed = mask_return & Button5Mask > 0;

    // Use 1-based indexing here so people can just query the button
    // number they're interested in directly.
    let button_pressed = vec![
        false,
        button1pressed,
        button2pressed,
        button3pressed,
        button4pressed,
        button5pressed,
    ];
    MouseState {
        coords: (win_x, win_y),
        button_pressed: button_pressed,
    }
}

pub fn query_keymap(display: &Display) -> Vec<u8> {
    let mut keycodes = vec![];
    unsafe {
        let keymap: *mut i8 = [0; 32].as_mut_ptr();
        XQueryKeymap(display.0, keymap);
        for (ix, byte) in
            slice::from_raw_parts(keymap, 32).iter().enumerate()
        {
            for bit in 0_u8..8_u8 {
                let bitmask = 1 << bit;
                if byte & bitmask != 0 {
                    //x11 keycode uses kernel keycode with an offset of 8.
                    let x11_key = ix as u8 * 8 + bit;
                    let kernel_key = x11_key - 8;
                    keycodes.push(kernel_key);
                }
            }
        }
    }
    keycodes
}