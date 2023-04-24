/*!
    ## A windows app to help with debugging development of Toylang
    Pass the debug flag to toylang.exe (-d or --debug) to open this interactive debugger which steps through the state of the Compiler's AST to troublshoot compilation errors.
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;

use crate::ast::elements::ElementsVec;
use crate::file::DebugFileContents;
use crate::{Compiler, DebugErrorStack, DebugLinesOfChars, DebugLinesOfTokens, DebugLogs};
use std::process;

const APP_NAME: &str = "Toylang - Compiler debugger";

#[derive(Default, Debug)]
pub struct MyData {
    pub history_max: String,
    pub history: Vec<String>,
    pub step: usize,
    pub step_max: usize,
    pub step_is_running: bool,
    pub step_is_resetting: bool,
}

impl MyData {
    pub fn history_update(&mut self, new_vec: &Vec<String>) {
        self.history = new_vec.clone();
    }
    pub fn history_max_update(&mut self, string: String) {
        self.history_max = string;
    }
    pub fn step_update(&mut self, val: usize) {
        self.step = val;
    }
    pub fn step_max_update(&mut self, val: usize) {
        self.step_max = val;
    }
    pub fn step_is_running_update(&mut self, val: bool) {
        self.step_is_running = val;
    }
    pub fn step_is_resetting_update(&mut self, val: bool) {
        self.step_is_resetting = val;
    }
}

#[derive(Default, NwgUi)]
pub struct ToylangDebugger {
    mydata: RefCell<MyData>,

    #[nwg_resource(source_file: Some("src/icon_128.ico"))]
    icon_128: nwg::Icon,

    #[nwg_resource(source_file: Some("src/icon_200.ico"))]
    icon_200: nwg::Icon,

    #[nwg_control(size: (1910, 800), position: (1912, 0), title: APP_NAME, flags: "WINDOW|VISIBLE|MAXIMIZED|RESIZABLE", icon: Some(&data.icon_200))]
    #[nwg_events( OnWindowClose: [ToylangDebugger::close], OnInit: [ToylangDebugger::rich_text_input_init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

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

    // Row 1
    #[nwg_control(text: "1. get file")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_1_get_file] )]
    button0: nwg::Button,

    #[nwg_control(text: "2. set lines of chars")]
    #[nwg_layout_item(layout: grid, row: 1, col: 2, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_2_set_lines_of_chars] )]
    button1: nwg::Button,

    #[nwg_control(text: "3. set lines of tokens")]
    #[nwg_layout_item(layout: grid, row: 1, col: 4, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_3_set_lines_of_tokens] )]
    button2: nwg::Button,

    #[nwg_control(text: "4. parse each line...")]
    #[nwg_layout_item(layout: grid, row: 1, col: 6, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_4_parse_each_line] )]
    button3: nwg::Button,

    #[nwg_control(text: "5. set output")]
    #[nwg_layout_item(layout: grid, row: 1, col: 8, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_5_set_output] )]
    button4: nwg::Button,

    #[nwg_control(text: "reset")]
    #[nwg_layout_item(layout: grid, row: 1, col: 10, col_span: 2)]
    #[nwg_events( OnButtonClick: [ToylangDebugger::change_step_reset] )]
    button_reset: nwg::Button,

    // Row 2
    #[nwg_control(text: "File Contents")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 0, col_span: 3)]
    label0: nwg::Label,

    #[nwg_control(text: "Lines of chars")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 3, col_span: 3)]
    label1: nwg::Label,

    #[nwg_control(text: "Lines of tokens")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 6, col_span: 3)]
    label2: nwg::Label,

    #[nwg_control(text: "AST previous")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 9, col_span: 3)]
    label3: nwg::Label,

    #[nwg_control(text: "AST current")]
    #[nwg_layout_item(layout: grid, row: 2,  col: 12, col_span: 3)]
    label4: nwg::Label,

    // Row 3
    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 3, col: 0, row_span: 5, col_span: 3)]
    richtext_input: nwg::RichTextBox,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 3, col: 3, row_span: 5, col_span: 3)]
    richtext_loc: nwg::RichTextBox,

    #[nwg_control(text: "",)]
    #[nwg_layout_item(layout: grid, row: 3, col: 6, row_span: 5, col_span:3)]
    richtext_lot: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 3, col: 9, row_span: 5, col_span: 3)]
    richtext_ast_previous: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 3, col: 12, row_span: 5, col_span: 3)]
    richtext_ast_current: nwg::RichTextBox,

    // Row 4
    #[nwg_control(text: "Error stack")]
    #[nwg_layout_item(layout: grid, row: 8,  col: 0, col_span: 3)]
    label5: nwg::Label,

    #[nwg_control(text: "Logs")]
    #[nwg_layout_item(layout: grid, row: 8,  col: 3, col_span: 3)]
    label6: nwg::Label,

    #[nwg_control(text: "AST (0 of 0)")]
    #[nwg_layout_item(layout: grid, row: 8,  col: 6, col_span: 1)]
    label7: nwg::Label,

    #[nwg_control(parent: window, size: (250, 30), position: (900,700), range: Some(0..0) )]
    #[nwg_events( OnHorizontalScroll: [ToylangDebugger::history_update] )]
    history_trackbar: nwg::TrackBar,

    #[nwg_control(text: "AST Current (Tree)")]
    #[nwg_layout_item(layout: grid, row: 8,  col: 9, col_span: 3)]
    label8: nwg::Label,

    #[nwg_control(text: "Output")]
    #[nwg_layout_item(layout: grid, row: 8,  col: 12, col_span: 3)]
    label9: nwg::Label,

    // Row 5
    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 9, col: 0, row_span: 5, col_span: 3)]
    richtext_error_stack: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 9, col: 3, row_span: 5, col_span: 3)]
    richtext_logs: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 9, col: 6, row_span: 5, col_span: 3)]
    richtext_dynamic_ast: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 9, col: 9, row_span: 5, col_span: 3)]
    richtext_tree: nwg::RichTextBox,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 9, col: 12, row_span: 5, col_span: 3)]
    richtext_output: nwg::RichTextBox,
}

impl ToylangDebugger {
    fn show_tray_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    pub fn change_step_1_get_file(&self) {
        self.mydata.borrow_mut().step_update(1);
        self.mydata.borrow_mut().step_max_update(1);
        self.mydata.borrow_mut().step_is_running_update(true);
    }

    pub fn change_step_2_set_lines_of_chars(&self) {
        self.mydata.borrow_mut().step_update(1);
        self.mydata.borrow_mut().step_max_update(2);
        self.mydata.borrow_mut().step_is_running_update(true);
    }

    pub fn change_step_3_set_lines_of_tokens(&self) {
        self.mydata.borrow_mut().step_update(1);
        self.mydata.borrow_mut().step_max_update(3);
        self.mydata.borrow_mut().step_is_running_update(true);
    }

    pub fn change_step_4_parse_each_line(&self) {
        self.mydata.borrow_mut().step_update(1);
        self.mydata.borrow_mut().step_max_update(4);
        self.mydata.borrow_mut().step_is_running_update(true);
    }

    pub fn change_step_5_set_output(&self) {
        self.mydata.borrow_mut().step_update(1);
        self.mydata.borrow_mut().step_max_update(5);
        self.mydata.borrow_mut().step_is_running_update(true);
    }

    pub fn change_step_reset(&self) {
        self.mydata.borrow_mut().step_is_resetting_update(true);
        self.mydata.borrow_mut().step_is_running_update(true);
    }

    pub fn change_step_stop(&self) {
        self.mydata.borrow_mut().step_is_running_update(false);
    }

    pub fn rich_text_input_init(&self) {
        let heading = format!("{}\r\n", APP_NAME);
        let text = "\r\n\r\nClick the buttons above in sequence\r\nto see the gradual output of the compiler's\r\ninternal data structures as it processes\r\nyour input filepath.";
        let all_text = [&heading, text].join("");
        self.richtext_input.set_text(&all_text);

        self.richtext_input.set_selection(0..(heading.len() - 1) as u32);
        self.richtext_input.set_char_format(&nwg::CharFormat {
            height: Some(400),
            text_color: Some([23, 105, 170]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });

        self.richtext_input.set_selection((heading.len() - 1) as u32..(all_text.len() - 1) as u32);
        self.richtext_input.set_char_format(&nwg::CharFormat {
            height: Some(300),
            text_color: Some([10, 10, 10]),
            font_face_name: Some("Calibri".to_string()),
            ..Default::default()
        });
    }

    fn rich_text_control_set_text(&self, control: &nwg::RichTextBox, text: &str) {
        control.set_text(text);
        control.set_selection(0..(control.text().len() as u32));
        control.set_char_format(&nwg::CharFormat {
            height: Some(150),
            text_color: Some([10, 10, 10]),
            font_face_name: Some("Courier".to_string()),
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

    fn history_update(&self) {
        self.label7.set_text(&format!("History ({} of {})", &self.history_trackbar.pos(), &self.mydata.borrow().history_max));
        self.rich_text_control_set_text(&self.richtext_dynamic_ast, &self.mydata.borrow_mut().history[self.history_trackbar.pos()]);
        self.richtext_dynamic_ast.scroll_lastline();
        self.richtext_dynamic_ast.scroll(-20);
    }
}

fn init(ui: &toylang_debugger_ui::ToylangDebuggerUi, input: String, debug: bool, output: Option<String>) -> Compiler {
    ui.textinput_filepath.set_text(&input);
    ui.textinput_outputdir.set_text(&(output.clone().unwrap()));
    ui.rich_text_control_set_text(&ui.richtext_loc, " ");
    ui.rich_text_control_set_text(&ui.richtext_lot, " ");
    ui.rich_text_control_set_text(&ui.richtext_ast_previous, " ");
    ui.rich_text_control_set_text(&ui.richtext_ast_current, " ");
    ui.rich_text_control_set_text(&ui.richtext_error_stack, " ");
    ui.rich_text_control_set_text(&ui.richtext_logs, " ");
    ui.rich_text_control_set_text(&ui.richtext_tree, " ");
    ui.rich_text_control_set_text(&ui.richtext_output, " ");
    return Compiler::new(input.clone(), debug, output.clone(), true).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

pub fn run(input: String, debug: bool, output: Option<String>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let ui = ToylangDebugger::build_ui(Default::default()).expect("Failed to build UI");
    let mut compiler = init(&ui, input.clone(), debug, output.clone());
    let mut log_prev = "".to_string();

    nwg::dispatch_thread_events_with_callback(move || {
        let step: usize = ui.mydata.borrow().step;
        let step_max: usize = ui.mydata.borrow().step_max;
        let step_is_running: bool = ui.mydata.borrow().step_is_running;
        let reset: bool = ui.mydata.borrow().step_is_resetting;
        let log = format!("# {} {} {} {}", step, step_max, step_is_running, reset);
        if log != log_prev {
            println!("{}", &log);
            log_prev = log;
        }

        if step_is_running {
            let mut completed_step = compiler.debug_step(step);
            //dbg!(step, step_max, completed_step);
            if reset {
                //dbg!("## reset");
                compiler = init(&ui, input.clone(), debug, output.clone());
                ui.button0.set_enabled(true);
                ui.button1.set_enabled(true);
                ui.button2.set_enabled(true);
                ui.button3.set_enabled(true);
                ui.button4.set_enabled(true);
                ui.mydata.borrow_mut().history_update(&vec![String::new()]);
                ui.mydata.borrow_mut().history_max_update("".to_string());
                ui.history_trackbar.set_pos(0);
                ui.history_trackbar.set_range_max(0);
                ui.history_update();
                ui.mydata.borrow_mut().step_update(0);
                ui.mydata.borrow_mut().step_max_update(0);
                ui.mydata.borrow_mut().step_is_running_update(false);
                ui.mydata.borrow_mut().step_is_resetting_update(false);
                completed_step = 0;
            }

            if completed_step == 1 {
                //dbg!(step, step_max, completed_step);
                let txt_input_debug = DebugFileContents(&compiler.file.filecontents);
                let txt_input = format!("{:?}", txt_input_debug);
                let txt_ast = format!("{:?}", compiler.ast);
                let txt_tree = format!("{:?}", ElementsVec(compiler.ast.elements.clone()));
                let txt_error = format!("{:?}", DebugErrorStack(&compiler.error_stack));
                let txt_logs = format!("{:?}", DebugLogs(&compiler.logs));
                let txt_output = format!("{}", compiler.output);

                ui.rich_text_control_set_text(&ui.richtext_input, &txt_input);
                ui.rich_text_control_set_text(&ui.richtext_ast_current, &txt_ast);
                ui.rich_text_control_set_text(&ui.richtext_error_stack, &txt_error);
                ui.rich_text_control_set_text(&ui.richtext_logs, &txt_logs);
                ui.rich_text_control_set_text(&ui.richtext_tree, &txt_tree);
                ui.rich_text_control_set_text(&ui.richtext_output, &txt_output);

                ui.button0.set_enabled(false);

                if step_max == 1 as usize {
                    ui.mydata.borrow_mut().step_is_running_update(false);
                } else {
                    ui.mydata.borrow_mut().step_update(2);
                }
            }

            if completed_step == 2 {
                //dbg!(step, step_max, completed_step);
                let txt_loc = format!("{:?}", DebugLinesOfChars(&compiler.lines_of_chars));
                ui.rich_text_control_set_text(&ui.richtext_loc, &txt_loc);
                ui.label1.set_text(&format!("Lines of chars (0 - {})", &compiler.lines_of_chars.len() - 1));

                ui.button0.set_enabled(false);
                ui.button1.set_enabled(false);
                if step_max == 2 as usize {
                    ui.mydata.borrow_mut().step_is_running_update(false);
                } else {
                    ui.mydata.borrow_mut().step_update(3);
                }
            }

            if completed_step == 3 {
                //dbg!(step, step_max, completed_step);
                let txt_lot = format!("{:?}", DebugLinesOfTokens(&compiler.lines_of_tokens));
                ui.rich_text_control_set_text(&ui.richtext_lot, &txt_lot);
                ui.label2.set_text(&format!("Lines of tokens (0 - {})", &compiler.lines_of_tokens.len() - 1));

                ui.button0.set_enabled(false);
                ui.button1.set_enabled(false);
                ui.button2.set_enabled(false);
                if step_max == 3 as usize {
                    ui.mydata.borrow_mut().step_is_running_update(false);
                } else {
                    ui.mydata.borrow_mut().step_update(4);
                }
            }

            if step_max >= 4 as usize {
                if completed_step == 3 {
                    //dbg!(step, step_max, completed_step);
                    let current_text = ui.richtext_ast_current.text();
                    let new_text = format!("{:?}", compiler.ast);
                    let txt_logs = format!("{:?}", DebugLogs(&compiler.logs));
                    let txt_tree = format!("{:?}", ElementsVec(compiler.ast.elements.clone()));
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
                    ui.richtext_ast_current.set_selection(first_non_matching_char..new_len - 1);
                    ui.richtext_ast_current.set_char_format(&nwg::CharFormat {
                        text_color: Some([20, 200, 20]),
                        ..Default::default()
                    });
                    ui.richtext_ast_current.scroll_lastline();
                    ui.richtext_ast_current.scroll(-20);

                    // update richtext_error_stack
                    let txt_error = format!("{:?}", DebugErrorStack(&compiler.error_stack));
                    ui.rich_text_control_set_text(&ui.richtext_error_stack, &txt_error);
                    ui.richtext_error_stack.set_selection(28..((txt_error.len() as u32) - 4));
                    ui.richtext_error_stack.set_char_format(&nwg::CharFormat {
                        text_color: Some([200, 20, 20]),
                        ..Default::default()
                    });
                    ui.label5.set_text(&format!("Error stack ({})", &compiler.error_stack.len()));

                    ui.rich_text_control_set_text(&ui.richtext_logs, &txt_logs);

                    ui.rich_text_control_set_text(&ui.richtext_tree, &txt_tree);

                    if compiler.logs.len() as usize > 0 {
                        let pos = &compiler.debug_compiler_history.len() - 1;
                        ui.mydata.borrow_mut().history_max_update(format!("{}", pos));
                        ui.mydata.borrow_mut().history_update(&compiler.debug_compiler_history);
                        ui.history_trackbar.set_range_max(pos);
                        ui.history_update();
                        ui.history_trackbar.set_pos(pos);
                    }

                    // update richtext_output
                    let txt_output = format!("{}", compiler.ast.output);
                    ui.rich_text_control_set_text(&ui.richtext_output, &txt_output);
                }
                if completed_step == 4 {
                    ui.button0.set_enabled(false);
                    ui.button1.set_enabled(false);
                    ui.button2.set_enabled(false);
                    ui.button3.set_enabled(false);
                    if step_max == 4 as usize {
                        ui.mydata.borrow_mut().step_is_running_update(false);
                    } else {
                        ui.mydata.borrow_mut().step_update(5);
                    }
                }
            }

            if completed_step == 5 {
                //dbg!(step, step_max, completed_step);
                let current_text = ui.richtext_ast_current.text();
                let new_text = format!("{:?}", compiler.ast);
                let txt_logs = format!("{:?}", DebugLogs(&compiler.logs));
                let txt_tree = format!("{:?}", ElementsVec(compiler.ast.elements.clone()));
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
                ui.richtext_ast_current.set_selection(first_non_matching_char..new_len - 1);
                ui.richtext_ast_current.set_char_format(&nwg::CharFormat {
                    text_color: Some([20, 200, 20]),
                    ..Default::default()
                });
                ui.richtext_ast_current.scroll_lastline();
                ui.richtext_ast_current.scroll(-20);

                ui.rich_text_control_set_text(&ui.richtext_logs, &txt_logs);
                ui.rich_text_control_set_text(&ui.richtext_tree, &txt_tree);

                if compiler.logs.len() as usize > 0 {
                    let pos = &compiler.debug_compiler_history.len() - 1;
                    ui.mydata.borrow_mut().history_max_update(format!("{}", pos));
                    ui.mydata.borrow_mut().history_update(&compiler.debug_compiler_history);
                    ui.history_trackbar.set_range_max(pos);
                    ui.history_update();
                    ui.history_trackbar.set_pos(pos);
                }

                // update richtext_error_stack
                let txt_error = format!("{:?}", DebugErrorStack(&compiler.error_stack));
                ui.rich_text_control_set_text(&ui.richtext_error_stack, &txt_error);
                ui.richtext_error_stack.set_selection(28..((txt_error.len() as u32) - 4));
                ui.richtext_error_stack.set_char_format(&nwg::CharFormat {
                    text_color: Some([200, 20, 20]),
                    ..Default::default()
                });
                ui.label5.set_text(&format!("Error stack ({})", &compiler.error_stack.len()));

                // update richtext_output
                let txt_output = format!("{}", compiler.ast.output);
                ui.rich_text_control_set_text(&ui.richtext_output, &txt_output);

                ui.button0.set_enabled(false);
                ui.button1.set_enabled(false);
                ui.button2.set_enabled(false);
                ui.button3.set_enabled(false);
                ui.button4.set_enabled(false);

                ui.mydata.borrow_mut().step_is_running_update(false);
                ui.mydata.borrow_mut().step_update(0);
            }
        }
    });
}
