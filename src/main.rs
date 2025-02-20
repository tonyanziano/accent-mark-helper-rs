use std::env;
use winsafe::{
    co::{VK, WH},
    gui,
    prelude::*,
    HHOOK, KEYBDINPUT,
};

fn main() {
    println!("Hello, world!");

    let parsed_args: Vec<String> = env::args().collect();

    parsed_args.iter().for_each(|f| println!("{f}"));

    // here the thread id is set to 0 to target all desktop threads (we'll see if this is bad later :) )
    let hook_result = HHOOK::SetWindowsHookEx(WH::KEYBOARD_LL, keyboard_hook, None, Some(0));
    let mut inner_hook = match hook_result {
        Ok(hhook) => hhook,
        Err(err) => {
            panic!("There was an error while trying to create the hook: {err:?}");
        }
    };

    // here we are just creating a window to try and keep the app running
    let my_win = MyWindow::new();
    if let Err(e) = my_win.wnd.run_main(None) {
        // ... and run it
        eprintln!("{}", e);
    }
    println!("End of program.");
    // clean up the keyboard hook
    inner_hook
        .UnhookWindowsHookEx()
        .expect("There was an issue cleaning up the keyboard listener on app quit.");
}

extern "system" fn keyboard_hook(code: i32, wparam: usize, lparam: isize) -> isize {
    println!("Got some keyboard event: {code}, {wparam}, {lparam}");

    // if code !== 0, then pass the message to CallNextHookEx() and return the result
    // (advised by Windows docs)
    if code != 0 {
        return HHOOK::CallNextHookEx(&HHOOK::NULL, WH::KEYBOARD_LL, wparam, lparam);
    }

    // lets take the raw pointer and cast it to a pointer
    let raw_ptr = lparam as *mut isize;
    // now let's cast this to a pointer of the type we are actually trying to access
    let my_ptr = raw_ptr as *mut KEYBDINPUT;
    // now we dereference the pointer (with *) to get the struct, and then create a mutable ref to it
    let my_ptr_ref = unsafe { &mut *my_ptr };
    let key_code = my_ptr_ref.wVk;
    println!("Trying to look at C pointer props: {key_code}");
    match key_code {
        VK::SHIFT => println!("Got SHIFT key!"),
        VK::CONTROL => println!("Got CTRL key!"),
        VK::CHAR_0 => println!("Got 0 key!"),
        _ => println!("Got some other key we don't care about!"),
    };

    // return the result of the next hook in the chain
    //  NOTE: The first argument is supposed to be the HHOOK handle returned by the initial
    //  SetWindowsHookEx. However, I can't figure out how to make this variable available to
    //  this function in Rust, and after some digging, the API docs specifically say
    //  that this parameter is ignored
    //  (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex#parameters)
    HHOOK::CallNextHookEx(&HHOOK::NULL, WH::KEYBOARD_LL, wparam, lparam)
}

//
// ==== Code below was copied from a sample ====
//
#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain,   // responsible for managing the window
    btn_hello: gui::Button, // a button
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: (300, 150),
                ..Default::default() // leave all other options as default
            },
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: (20, 20),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone(); // clone so it can be passed into the closure
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?; // call native Windows API
            Ok(())
        });
    }
}
