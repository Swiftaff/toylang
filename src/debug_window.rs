/*!
    A very simple application that shows your name in a message box.
    Uses layouts to position the controls in the window
*/

extern crate native_windows_gui as nwg;
use crate::compiler_runner;
use nwg::NativeUi;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    layout: nwg::GridLayout,
    pub name_edit: nwg::TextInput,
    hello_button: nwg::Button,
}

impl BasicApp {
    pub fn win_title(&self, txt: &str) {
        self.window.set_text(txt);
    }

    pub fn say_hello(&self, txt: &str) {
        nwg::modal_info_message(
            &self.window,
            txt,
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "Goodbye",
            &format!("Goodbye {}", self.name_edit.text()),
        );
        nwg::stop_thread_dispatch();
    }
}

pub struct BasicAppUi {
    pub inner: Rc<BasicApp>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use super::*;
    use native_windows_gui as nwg;

    use std::ops::Deref;

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((300, 300))
                .title("Basic example")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .text("Heisenberg")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.name_edit)?;

            nwg::Button::builder()
                .text("Say my name")
                .parent(&data.window)
                .build(&mut data.hello_button)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &ui.hello_button {
                                BasicApp::say_hello(&ui, "testy1");
                            }
                        }
                        E::OnWindowClose => {
                            if &handle == &ui.window {
                                BasicApp::say_goodbye(&ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(1)
                .child(0, 0, &ui.name_edit)
                .child_item(nwg::GridLayoutItem::new(&ui.hello_button, 0, 1, 1, 2))
                .build(&ui.layout)?;

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

pub fn run(input: String, debug: bool, output: Option<String>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    compiler_runner::main(input, debug, output);
    let mut count = 1;
    nwg::dispatch_thread_events_with_callback(move || {
        count = count + 1;
        println!("test {}", count)
    });

    //
}
