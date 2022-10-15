use crate::egui::Vec2;
use eframe::egui;

const CSIZE: usize = 30000;

#[derive(Clone, Copy)]
enum OPS {
    Decrement,
    Increment,
    Input,
    LoopEnd,
    LoopStart,
    Print,
    ShiftLeft,
    ShiftRight,
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2 { x: 930.0, y: 650.0 }),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "BF Interpreter",
        options,
        Box::new(|cc| Box::new(BFInterpreter::new(cc))),
    );
}

struct BFInterpreter {
    cells: [u8; CSIZE],
    current_instruction: usize,
    eof: bool,
    fnord: bool,
    index: usize,
    input: bool,
    input_text: String,
    loop_stack: Vec<usize>,
    offset_cells: usize,
    program_ops: Vec<OPS>,
    program_text: String,
    result_text: String,
}

impl BFInterpreter {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        BFInterpreter {
            cells: [0; CSIZE],
            current_instruction: 0,
            eof: false,
            fnord: false,
            index: 0,
            input: false,
            input_text: String::new(),
            loop_stack: Vec::new(),
            offset_cells: 0,
            program_ops: Vec::new(),
            program_text: String::from("-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[->+++++<]>+.------------.---[->+++<]>.-[--->+<]>---.+++.------.--------.-[--->+<]>..."),
            result_text: String::new(),
        }
    }
    fn run(&mut self) {
        if self.input && !self.eof {
            self.cells[self.index] = self.input_text.chars().next().unwrap() as u8;
            self.input = false;
        }
        if !self.input && !self.eof {
            loop {
                if self.program_ops.is_empty() {
                    break;
                }
                let op = self.program_ops[self.current_instruction];
                match op {
                    OPS::ShiftRight => {
                        self.index += 1;
                        if self.index == CSIZE {
                            self.index = 0;
                        }
                    }
                    OPS::ShiftLeft => {
                        if (self.index as isize) - 1 == -1 {
                            self.index = CSIZE - 1;
                        } else {
                            self.index -= 1;
                        }
                    }
                    OPS::Increment => {
                        self.cells[self.index] = self.cells[self.index].wrapping_add(1);
                    }
                    OPS::Decrement => {
                        self.cells[self.index] = self.cells[self.index].wrapping_sub(1);
                    }
                    OPS::Print => {
                        let mut tmp = [0, 4];
                        self.result_text
                            .push_str(char::encode_utf8(self.cells[self.index] as char, &mut tmp));
                    }
                    OPS::Input => {
                        self.input = true;
                        self.current_instruction += 1;
                        if self.current_instruction == self.program_ops.len() {
                            self.current_instruction = 0;
                            self.eof = true;
                        }
                        break;
                    }
                    OPS::LoopStart => {
                        self.loop_stack.push(self.current_instruction);
                    }
                    OPS::LoopEnd => {
                        if self.cells[self.index] != 0 {
                            self.current_instruction = *self.loop_stack.last().unwrap();
                        } else {
                            self.loop_stack.pop();
                        }
                    }
                }
                self.current_instruction += 1;
                if self.current_instruction == self.program_ops.len() {
                    self.current_instruction = 0;
                    self.eof = true;
                    break;
                }
            }
        }
    }
}

impl eframe::App for BFInterpreter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.fnord {
                    ui.heading("BrainFNORD Interpreter");
                    ui.horizontal(|ui| {
                        ui.label("BrainFNORD program to run");
                        if ui.button("Convert to Brainfuck mode").clicked() {
                            self.fnord = false;
                        }
                    });
                } else {
                    ui.heading("Brainfuck Interpreter");
                    ui.horizontal(|ui| {
                        ui.label("Brainfuck program to run");
                        if ui.button("Convert to BrainFNORD mode").clicked() {
                            self.fnord = true;
                        }
                    });
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 400.0],
                        egui::TextEdit::multiline(&mut self.program_text)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .desired_rows(29),
                    );
                });
                ui.horizontal(|ui| {
                    for i in 0..19 {
                        ui.vertical(|ui| {
                            ui.add(
                                egui::Label::new(format!("{}", i + self.offset_cells)).wrap(false),
                            );
                            ui.add(egui::DragValue::new(&mut self.cells[i + self.offset_cells]));
                        });
                    }
                });
                ui.add(
                    egui::Slider::new(&mut self.offset_cells, 0..=CSIZE - 19).text("Scroll Cells"),
                );
                ui.heading("Results Panel");
                ui.horizontal(|ui| {
                    if self.input {
                        ui.label("Input a character please");
                        ui.add(egui::TextEdit::singleline(&mut self.input_text));
                        if ui.button("Enter your input").clicked() {
                            self.run();
                        }
                    } else if ui.button("Run").clicked() {
                        if self.fnord {
                            let mut token_index = 0;
                            loop {
                                if token_index == self.program_text.as_bytes().len() {
                                    break;
                                }
                                let token_start: char =
                                    self.program_text.as_bytes()[token_index] as char;
                                match token_start {
                                    'f' => {
                                        let token = "fnord";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::ShiftLeft);
                                            token_index += token.len();
                                        }
                                    }
                                    'k' => {
                                        let token = "kallisti";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::ShiftRight);
                                            token_index += token.len();
                                        }
                                    }
                                    'p' => {
                                        let token = "pineal";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::Print);
                                            token_index += token.len();
                                        }
                                    }
                                    'c' => {
                                        let token = "chaos";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::Input);
                                            token_index += token.len();
                                        }
                                    }
                                    '2' => {
                                        let token = "23";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::LoopStart);
                                            token_index += token.len();
                                        }
                                    }
                                    '5' => {
                                        let token = "5";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::Increment);
                                            token_index += token.len();
                                        }
                                    }
                                    'h' => {
                                        let token = "hail";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::Decrement);
                                            token_index += token.len();
                                        }
                                    }
                                    'e' => {
                                        let token = "eris";
                                        if &self.program_text
                                            [token_index..token_index + token.len()]
                                            == token
                                        {
                                            self.program_ops.push(OPS::LoopEnd);
                                            token_index += token.len();
                                        }
                                    }
                                    _ => {
                                        token_index += 1;
                                    }
                                }
                            }
                        } else {
                            for c in self.program_text.chars() {
                                match c {
                                    '>' => {
                                        self.program_ops.push(OPS::ShiftRight);
                                    }
                                    '<' => {
                                        self.program_ops.push(OPS::ShiftLeft);
                                    }
                                    '+' => {
                                        self.program_ops.push(OPS::Increment);
                                    }
                                    '-' => {
                                        self.program_ops.push(OPS::Decrement);
                                    }
                                    '.' => {
                                        self.program_ops.push(OPS::Print);
                                    }
                                    ',' => {
                                        self.program_ops.push(OPS::Input);
                                    }
                                    '[' => {
                                        self.program_ops.push(OPS::LoopStart);
                                    }
                                    ']' => {
                                        self.program_ops.push(OPS::LoopEnd);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        self.run();
                    }
                    if ui.button("Reset").clicked() {
                        self.cells = [0; CSIZE];
                        self.index = 0;
                        self.result_text = "".to_owned();
                    }
                    if self.eof {
                        self.program_ops = Vec::new();
                        self.loop_stack = Vec::new();
                        self.input = false;
                        self.current_instruction = 0;
                        self.eof = false;
                    }
                });
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.result_text),
                );
            });
        });
    }
}
