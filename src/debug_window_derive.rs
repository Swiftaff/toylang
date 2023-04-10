/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use crate::Compiler;
use crate::DebugLinesOfChars;
use std::process;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (1918, 800), position: (1912, 0), title: "Toylan compiler debugger", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::close], OnInit: [BasicApp::rich_text_input_init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "", flags: "NONE")]
    #[nwg_layout_item(layout: grid, row: 0,  col: 0)]
    label: nwg::Label,

    // Row 0
    #[nwg_control(text: "filepath")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    filepath_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 0, col: 1, col_span: 2)]
    filepath_text: nwg::TextInput,

    #[nwg_control(text: "outputdir")]
    #[nwg_layout_item(layout: grid, row: 0, col: 4)]
    outputdir_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 0, col: 5, col_span: 2)]
    outputdir_text: nwg::TextInput,

    // Row 1
    #[nwg_control(text: "0. get file")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::change_step_0_get_file] )]
    button0: nwg::Button,

    #[nwg_control(text: "1. set_lines_of_chars")]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    #[nwg_events( OnButtonClick: [BasicApp::change_step_1_set_lines_of_chars] )]
    button1: nwg::Button,

    #[nwg_control(text: "stop")]
    #[nwg_layout_item(layout: grid, row: 1, col: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::change_step_stop] )]
    stop_button: nwg::Button,

    // Row 2
    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 2, col: 0, row_span: 10, col_span: 5)]
    rich_text_input: nwg::RichTextBox,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 2, col: 5, row_span: 10, col_span: 5)]
    rich_text_ast: nwg::RichTextBox,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 2, col: 10, row_span: 10, col_span: 5)]
    rich_text_loc: nwg::RichTextBox,
}

impl BasicApp {
    pub fn change_step_0_get_file(&self) {
        self.label.set_text("0. get file");
    }

    pub fn change_step_1_set_lines_of_chars(&self) {
        self.label.set_text("1. set_lines_of_chars");
    }

    pub fn change_step_stop(&self) {
        self.label.set_text("stop");
    }

    pub fn rich_text_input_init(&self) {
        let heading = "Test heading\r\n";
        let text = "Example paragraph text";
        let all_text = [heading, text].join("");
        self.rich_text_input.set_text(&all_text);

        self.rich_text_input
            .set_selection(0..(heading.len() - 1) as u32);
        self.rich_text_input.set_char_format(&nwg::CharFormat {
            height: Some(500),
            text_color: Some([50, 50, 150]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });

        self.rich_text_input
            .set_selection((heading.len() - 1) as u32..(all_text.len() - 1) as u32);
        self.rich_text_input.set_char_format(&nwg::CharFormat {
            height: Some(300),
            text_color: Some([10, 10, 10]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });
    }

    pub fn rich_text_input_set_text(&self, text: &str) {
        self.rich_text_control_set_text(&self.rich_text_input, text);
    }

    fn rich_text_control_set_text(&self, control: &nwg::RichTextBox, text: &str) {
        control.set_text(text);
        control.set_selection(0..control.len() as u32);
        control.set_char_format(&nwg::CharFormat {
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

    let mut compiler = Compiler::new(input.clone(), debug, output.clone()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    ui.filepath_text.set_text(&input);
    ui.outputdir_text.set_text(&(output.unwrap()));

    let mut step = "".to_string();
    nwg::dispatch_thread_events_with_callback(move || {
        step = ui.label.text();
        let _result = compiler.debug_step(&step);
        if step == "0. get file" {
            let txt_input = compiler.rem_first_and_last(&compiler.file.filecontents);
            let txt_ast = format!("{:?}", compiler.ast,);
            ui.rich_text_control_set_text(&ui.rich_text_input, &txt_input);
            ui.rich_text_control_set_text(&ui.rich_text_ast, &txt_ast);
            ui.label.set_text("stop");
        }
        if step == "1. set_lines_of_chars" {
            let txt_loc = format!("{:?}", DebugLinesOfChars(&compiler.lines_of_chars));
            ui.rich_text_control_set_text(&ui.rich_text_loc, &txt_loc);
            ui.label.set_text("stop");
        }
    });
}
