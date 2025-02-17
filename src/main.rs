use std::env;

use winsafe::{self as w, co::WH, gui, prelude::*, HHOOK, KEYBDINPUT};

fn main() {
    println!("Hello, world!");

    let parsed_args: Vec<String> = env::args().collect();

    parsed_args.iter().for_each(|f| println!("{f}"));

    // here the thread id is set to 0 to target all desktop threads (we'll see if this is bad later :) )
    let hookResult = HHOOK::SetWindowsHookEx(WH::KEYBOARD_LL, keyboardHook, None, Some(0));
    let hook = match hookResult {
        Ok(result) => result,
        Err(err) => panic!("Problem opening the file: {err:?}"),
    };

    // here we are just creating a window to try and keep the app running
    let myWin = MyWindow::new();
    if let Err(e) = myWin.wnd.run_main(None) {
        // ... and run it
        eprintln!("{}", e);
    }
}

extern "system" fn keyboardHook(code: i32, wParam: usize, lParam: isize) -> isize {
    println!("Got some keyboard event: {code}, {wParam}, {lParam}");

    // TODO: if code !== 0, then pass the message to CallNextHookEx() and return the result

    // otherwise, do stuff
    // TODO: we need to somehow take the pointer "lParam" from C and cast it to the KEYBDINPUT struct in Rust
    let parsedStruct = unsafe {
        KEYBDINPUT::from((HHOOK::ptr(&self) as *mut KEYBDINPUT).);
    };

    return 1;
}

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
