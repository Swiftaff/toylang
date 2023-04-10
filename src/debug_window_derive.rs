/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use crate::file::DebugFileContents;
use crate::{Compiler, DebugLinesOfChars, DebugLinesOfTokens};
use std::process;

const APP_NAME: &str = "Toylang - Compiler debugger";

#[derive(Default, NwgUi)]
pub struct ToylangDebugger {
    #[nwg_resource(source_file: Some("src/icon_128.ico"))]
    icon_128: nwg::Icon,

    #[nwg_resource(source_file: Some("src/icon_200.ico"))]
    icon_200: nwg::Icon,

    #[nwg_control(size: (1910, 800), position: (1912, 0), title: APP_NAME, flags: "WINDOW|VISIBLE", icon: Some(&data.icon_200))]
    #[nwg_events( OnWindowClose: [ToylangDebugger::close], OnInit: [ToylangDebugger::rich_text_input_init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "", flags: "NONE")]
    #[nwg_layout_item(layout: grid, row: 0,  col: 0)]
    label_hidden_step: nwg::Label,

    // Tray Menu
    #[nwg_control(icon: Some(&data.icon_128), balloon_icon: Some(&data.icon_200), tip: Some(APP_NAME))]
    #[nwg_events(OnContextMenu: [ToylangDebugger::show_tray_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: APP_NAME, disabled: true)]
    tray_item0: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit...")]
    #[nwg_events(OnMenuItemSelected: [ToylangDebugger::close])]
    tray_item1: nwg::MenuItem,

    // Row 0
    #[nwg_control(text: "filepath")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    label_filepath: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 0, col: 1, col_span: 2)]
    textinput_filepath: nwg::TextInput,

    #[nwg_control(text: "outputdir")]
    #[nwg_layout_item(layout: grid, row: 0, col: 4)]
    label_outputdir: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 0, col: 5, col_span: 2)]
    textinput_outputdir: nwg::TextInput,

    #[nwg_control(text: "current_line: 0")]
    #[nwg_layout_item(layout: grid, row: 0, col: 8, col_span: 2)]
    label_currentline: nwg::Label,

    #[nwg_control(text: "current_line_token: 0")]
    #[nwg_layout_item(layout: grid, row: 0, col: 10, col_span: 2)]
    label_currentlinetoken: nwg::Label,

    #[nwg_control(text: "error_stack length: 0")]
    #[nwg_layout_item(layout: grid, row: 0, col: 12, col_span: 2)]
    label_errorstack: nwg::Label,

    // Row 1
    #[nwg_control(text: "0. get file")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_0_get_file] )]
    button0: nwg::Button,

    #[nwg_control(text: "1. set_lines_of_chars")]
    #[nwg_layout_item(layout: grid, row: 1, col: 2, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_1_set_lines_of_chars] )]
    button1: nwg::Button,

    #[nwg_control(text: "2. set_lines_of_tokens")]
    #[nwg_layout_item(layout: grid, row: 1, col: 4, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_2_set_lines_of_tokens] )]
    button2: nwg::Button,

    #[nwg_control(text: "3. parse each line...")]
    #[nwg_layout_item(layout: grid, row: 1, col: 6, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_3_parse_each_line] )]
    button3: nwg::Button,

    #[nwg_control(text: "stop")]
    #[nwg_layout_item(layout: grid, row: 1, col: 8, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_stop] )]
    stop_button: nwg::Button,

    // Row 2
    #[nwg_control(text: "File Contents")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 0, col_span: 4)]
    label0: nwg::Label,

    #[nwg_control(text: "Lines of chars")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 4, col_span: 4)]
    label1: nwg::Label,

    #[nwg_control(text: "Lines of tokens")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 8, col_span: 4)]
    label2: nwg::Label,

    #[nwg_control(text: "AST previous")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 12, col_span: 4)]
    label3: nwg::Label,

    #[nwg_control(text: "AST current")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 16, col_span: 4)]
    label4: nwg::Label,

    // Row 3
    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 3, col: 0, row_span: 10, col_span: 4)]
    richtext_input: nwg::RichTextBox,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 3, col: 4, row_span: 10, col_span: 4)]
    richtext_loc: nwg::RichTextBox,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 3, col: 8, row_span: 10, col_span:4)]
    richtext_lot: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 3, col: 12, row_span: 10, col_span: 4)]
    richtext_ast_previous: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 3, col: 16, row_span: 10, col_span: 4)]
    richtext_ast_current: nwg::RichTextBox,
}

impl ToylangDebugger {
    fn show_tray_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    pub fn change_step_0_get_file(&self) {
        self.label_hidden_step.set_text("0. get file");
    }

    pub fn change_step_1_set_lines_of_chars(&self) {
        self.label_hidden_step.set_text("1. set_lines_of_chars");
    }

    pub fn change_step_2_set_lines_of_tokens(&self) {
        self.label_hidden_step.set_text("2. set_lines_of_tokens");
    }

    pub fn change_step_3_parse_each_line(&self) {
        self.label_hidden_step.set_text("3. parse_each_line");
    }

    pub fn change_step_stop(&self) {
        self.label_hidden_step.set_text("stop");
    }

    pub fn rich_text_input_init(&self) {
        let heading = format!("{}\r\n", APP_NAME);
        let text = "\r\n\r\nClick the buttons above in sequence\r\nto see the gradual output of the compiler's\r\ninternal data structures as it processes\r\nyour input filepath.";
        let all_text = [&heading, text].join("");
        self.richtext_input.set_text(&all_text);

        self.richtext_input
            .set_selection(0..(heading.len() - 1) as u32);
        self.richtext_input.set_char_format(&nwg::CharFormat {
            height: Some(400),
            text_color: Some([23, 105, 170]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });

        self.richtext_input
            .set_selection((heading.len() - 1) as u32..(all_text.len() - 1) as u32);
        self.richtext_input.set_char_format(&nwg::CharFormat {
            height: Some(300),
            text_color: Some([10, 10, 10]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });
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
        //control.set_para_format(&nwg::ParaFormat {
        //    numbering: Some(nwg::ParaNumbering::Arabic),
        //    ..Default::default()
        //});
        control.set_selection(0..0);
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub fn run(input: String, debug: bool, output: Option<String>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let ui = ToylangDebugger::build_ui(Default::default()).expect("Failed to build UI");

    let mut compiler = Compiler::new(input.clone(), debug, output.clone()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    ui.textinput_filepath.set_text(&input);
    ui.textinput_outputdir.set_text(&(output.unwrap()));
    ui.rich_text_control_set_text(&ui.richtext_loc, " ");
    ui.rich_text_control_set_text(&ui.richtext_lot, " ");
    ui.rich_text_control_set_text(&ui.richtext_ast_previous, " ");
    ui.rich_text_control_set_text(&ui.richtext_ast_current, " ");

    let mut step = "".to_string();
    nwg::dispatch_thread_events_with_callback(move || {
        step = ui.label_hidden_step.text();
        let _result = compiler.debug_step(&step);
        if step == "0. get file" {
            let txt_input_debug = DebugFileContents(&compiler.file.filecontents);
            let txt_input = format!("{:?}", txt_input_debug);
            let txt_ast = format!("{:?}", compiler.ast);
            ui.rich_text_control_set_text(&ui.richtext_input, &txt_input);
            ui.rich_text_control_set_text(&ui.richtext_ast_current, &txt_ast);
            ui.label_hidden_step.set_text("stop");
            ui.button0.set_enabled(false);
        }

        if step == "1. set_lines_of_chars" {
            let txt_loc = format!("{:?}", DebugLinesOfChars(&compiler.lines_of_chars));
            ui.rich_text_control_set_text(&ui.richtext_loc, &txt_loc);
            ui.label_hidden_step.set_text("stop");
            ui.button1.set_enabled(false);
        }

        if step == "2. set_lines_of_tokens" {
            let txt_lot = format!("{:?}", DebugLinesOfTokens(&compiler.lines_of_tokens));
            ui.rich_text_control_set_text(&ui.richtext_lot, &txt_lot);
            ui.label_hidden_step.set_text("stop");
            ui.button2.set_enabled(false);
        }

        if step == "3. parse_each_line" {
            let current_text = ui.richtext_ast_current.text();
            let new_text = format!("{:?}", compiler.ast);
            let new_len = new_text.len() as u32;
            let mut first_non_matching_char = 0;
            for (c1, c2) in new_text.chars().zip(current_text.chars()) {
                if c1 != c2 {
                    break;
                }
                first_non_matching_char += 1;
            }

            // update richtext_ast_previous
            ui.rich_text_control_set_text(&ui.richtext_ast_previous, &current_text);
            ui.richtext_ast_previous.scroll_lastline();
            ui.richtext_ast_previous.scroll(-20);

            // update richtext_ast_current
            ui.rich_text_control_set_text(&ui.richtext_ast_current, &new_text);
            ui.richtext_ast_current
                .set_selection(first_non_matching_char..new_len - 1);
            ui.richtext_ast_current.set_char_format(&nwg::CharFormat {
                text_color: Some([20, 200, 20]),
                ..Default::default()
            });
            ui.richtext_ast_current.scroll_lastline();
            ui.richtext_ast_current.scroll(-20);

            //update other fields
            ui.label_currentline
                .set_text(&format!("current_line: {}", compiler.current_line));
            ui.label_currentlinetoken.set_text(&format!(
                "current_line_token: {}",
                compiler.current_line_token
            ));
            ui.label_hidden_step.set_text("stop");

            //disable button when done
            if compiler.current_line == compiler.lines_of_tokens.len() - 1 {
                ui.button3.set_enabled(false);
            }
        }
    });
}
