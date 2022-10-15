#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::*;
use itertools::Itertools;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;

#[derive(Clone)]
struct MyDroppedFile {
    dropped_file: DroppedFile,
}

impl PartialEq for MyDroppedFile {
    fn eq(&self, otherfile: &Self) -> bool {
        self.dropped_file.path == otherfile.dropped_file.path
    }
}

#[derive(Default)]
struct Bf2Bf {
    dropped_files: Vec<MyDroppedFile>,
    picked_path: Option<String>,
    converted: bool,
}

impl Bf2Bf {
    fn append(&mut self, dropped_files_vec: &mut Vec<DroppedFile>) {
        for f in dropped_files_vec {
            self.dropped_files.append(&mut vec![MyDroppedFile {
                dropped_file: f.clone(),
            }]);
        }
    }

    fn write_bf2_file(&mut self, file: String) -> Result<(), Box<dyn Error>> {
        let mut output_path = PathBuf::from(file.clone());
        output_path.set_extension("bf2");
        let mut output_program = File::create(output_path)?;
        let program_code: String = fs::read_to_string(file)?;
        for c in program_code.chars() {
            match c {
                '>' => {
                    write!(output_program, "kallisti")?;
                }
                '<' => {
                    write!(output_program, "fnord")?;
                }
                '+' => {
                    write!(output_program, "5")?;
                }
                '-' => {
                    write!(output_program, "hail")?;
                }
                '.' => {
                    write!(output_program, "pineal")?;
                }
                ',' => {
                    write!(output_program, "chaos")?;
                }
                '[' => {
                    write!(output_program, "23")?;
                }
                ']' => {
                    write!(output_program, "eris")?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        for mdf in self.dropped_files.clone() {
            self.write_bf2_file(
                mdf.dropped_file
                    .path
                    .as_ref()
                    .unwrap()
                    .as_path()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            )?;
        }
        if let Some(picked) = &self.picked_path {
            self.write_bf2_file(picked.clone())?;
        }
        self.dropped_files = Vec::new();
        self.picked_path = None;
        Ok(())
    }
}

impl eframe::App for Bf2Bf {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag and drop a brainfuck program to convert to brainFNORD2");
            if ui.button("Open file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
                self.process().ok();
                self.converted = true;
            }
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Converted files:");
                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.dropped_file.path {
                            path.display().to_string()
                        } else if !file.dropped_file.name.is_empty() {
                            file.dropped_file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.dropped_file.bytes {
                            write!(info, " ({} bytes)", bytes.len()).ok();
                        }
                        ui.label(info);
                    }
                });
            }
            if self.converted {
                ui.label("Converted!");
            }
        });
        preview_files_being_dropped(ctx);
        if !ctx.input().raw.dropped_files.is_empty() {
            self.append(&mut ctx.input().raw.dropped_files.clone());
            self.dropped_files = self
                .dropped_files
                .clone()
                .into_iter()
                .unique_by(|f| f.clone().dropped_file.path)
                .filter(|f| !f.clone().dropped_file.path.unwrap().is_dir())
                .collect();
            self.process().ok();
            self.converted = true;
        }
    }
}

fn preview_files_being_dropped(ctx: &Context) {
    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Converting files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                write!(text, "\n{}", path.display()).ok();
            } else if !file.mime.is_empty() {
                write!(text, "\n{}", file.mime).ok();
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "BrainFuck to BrainFNORD2",
        options,
        Box::new(|_cc| Box::new(Bf2Bf::default())),
    );
}
