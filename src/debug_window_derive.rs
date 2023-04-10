/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use crate::Compiler;
use std::process;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (1918, 800), position: (1912, 0), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::close], OnInit: [BasicApp::rich_text_box_init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Heisenberg", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0, col_span: 2)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "0. get file")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    #[nwg_events( OnButtonClick: [BasicApp::change_step_0_get_file] )]
    start_button: nwg::Button,

    #[nwg_control(text: "stop")]
    #[nwg_layout_item(layout: grid, col: 1, row: 1)]
    #[nwg_events( OnButtonClick: [BasicApp::change_step_stop] )]
    stop_button: nwg::Button,

    #[nwg_control(text: "", flags: "NONE")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    label: nwg::Label,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 10)]
    rich_text: nwg::RichTextBox,
}

impl BasicApp {
    pub fn change_step_0_get_file(&self) {
        self.label.set_text("0. get file");
    }

    pub fn change_step_stop(&self) {
        self.label.set_text("stop");
    }

    pub fn rich_text_box_init(&self) {
        let heading = "Test heading\r\n";
        let text = "Example paragraph text";
        let all_text = [heading, text].join("");
        self.rich_text.set_text(&all_text);

        self.rich_text.set_selection(0..(heading.len() - 1) as u32);
        self.rich_text.set_char_format(&nwg::CharFormat {
            height: Some(500),
            text_color: Some([50, 50, 150]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });

        self.rich_text
            .set_selection((heading.len() - 1) as u32..(all_text.len() - 1) as u32);
        self.rich_text.set_char_format(&nwg::CharFormat {
            height: Some(300),
            text_color: Some([10, 10, 10]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });
    }

    pub fn rich_text_box_set_text(&self, text: &str) {
        self.rich_text.set_text(text);
        self.rich_text.set_selection(0..self.rich_text.len() as u32);
        self.rich_text.set_char_format(&nwg::CharFormat {
            height: Some(300),
            text_color: Some([10, 10, 10]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub fn run(input: String, debug: bool, output: Option<String>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    let mut compiler = Compiler::new(input, debug, output).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //println!("compiler {:?}", compiler);

    let mut step = "".to_string();
    nwg::dispatch_thread_events_with_callback(move || {
        step = ui.label.text();
        let result = compiler.debug_step(&step);
        if step == "0. get file" {
            ui.rich_text_box_set_text(&result);
            ui.label.set_text("stop");
        }
    });
}
