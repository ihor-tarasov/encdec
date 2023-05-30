mod driver;
use std::borrow::Cow;

use driver::*;

use eframe::NativeOptions;
use eframe::egui;

trait Process {
    fn process(&self, d: &mut dyn Driver) -> Vec<u8>;
}

impl<S: AsRef<[u8]>> Process for S {
    fn process(&self, d: &mut dyn Driver) -> Vec<u8> {
        self.as_ref().iter().cloned().filter_map(|b| d.feed(b)).collect()
    }
}

#[derive(PartialEq, Debug)]
enum Method {
    XOR,
    Addict,
    Subtract,
}

#[derive(PartialEq, Debug)]
enum DataKind {
    Text,
    Hex,
}

struct MyApp {
    input_kind: DataKind,
    input: String,
    key: String,
    result_hex: String,
    result_string: String,
    method: Method,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_kind: DataKind::Text,
            input: "".to_string(),
            key: "".to_string(),
            result_hex: String::new(),
            result_string: String::new(),
            method: Method::XOR,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Input type")
                .selected_text(format!("{:?}", self.input_kind))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.input_kind, DataKind::Text, "Text");
                    ui.selectable_value(&mut self.input_kind, DataKind::Hex, "Hex");
                }
            );

            ui.horizontal(|ui| {
                let name_label = ui.label("Input:");
                ui.text_edit_multiline(&mut self.input)
                    .labelled_by(name_label.id);
            });

            let input = match self.input_kind {
                DataKind::Text => self.input.clone().into_bytes(),
                DataKind::Hex => {
                    for b in self.input.bytes() {
                        if !((b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')) {
                            ui.label("Invalid input HEX symbols");
                            return;
                        }
                    }
                    self.input.as_bytes().chunks(2).map(|b| {
                        if b.len() == 1 {
                            (b[0], None)
                        } else {
                            (b[0], Some(b[1]))
                        }
                    }).map(|(c0, c1)| {
                        fn hex_to_byte(c: u8) -> u8 {
                            if c >= b'0' && c <= b'9' {
                                c - b'0'
                            } else if c >= b'a' && c <= b'f' {
                                c - b'a' + 10
                            } else if c >= b'A' && c <= b'F' {
                                c - b'A' + 10
                            } else {
                                panic!()
                            }
                        }

                        (hex_to_byte(c0), c1.and_then(|b| Some(hex_to_byte(b))))
                    }).map(|(c0, c1)| {
                        let mut r = c0 << 4;
                        if let Some(c) = c1 {
                            r = r | c;
                        }
                        r
                    } ).collect()
                }
            };
            
            ui.horizontal(|ui| {
                let name_label = ui.label("Key:");
                ui.text_edit_multiline(&mut self.key)
                    .labelled_by(name_label.id);
            });

            if self.key.is_empty() {
                ui.label("Key can't be empty");
                return;
            }

            egui::ComboBox::from_label("Method")
                .selected_text(format!("{:?}", self.method))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.method, Method::XOR, "XOR");
                    ui.selectable_value(&mut self.method, Method::Addict, "Addict");
                    ui.selectable_value(&mut self.method, Method::Subtract, "Subtract");
                }
            );

            let mut driver: Box<dyn Driver> = match self.method {
                Method::XOR => Box::new(XOR::new(self.key.bytes())),
                Method::Addict => Box::new(Addict::new(self.key.bytes())),
                Method::Subtract => Box::new(Subtract::new(self.key.bytes())),
            };

            let result = input.process(driver.as_mut());

            self.result_string = match String::from_utf8_lossy(&result) {
                Cow::Borrowed(s) => s.to_string(),
                Cow::Owned(s) => s,
            };
            
            ui.horizontal(|ui| {
                let name_label = ui.label("String result:");
                ui.text_edit_multiline(&mut self.result_string)
                    .labelled_by(name_label.id);
            });

            self.result_hex.clear();

            for b in result {
                fn byte_to_hex(c: u8) -> u8 {
                    if c <= 9 {
                        b'0' + c
                    } else if c >= 10 && c <= 15 {
                        b'A' + (c - 10)
                    } else {
                        panic!("{c}")
                    }
                }

                self.result_hex.push(byte_to_hex(b >> 4) as char);
                self.result_hex.push(byte_to_hex(b & 0xF) as char);
            }

            ui.horizontal(|ui| {
                let name_label = ui.label("HEX result:");
                ui.text_edit_multiline(&mut self.result_hex)
                    .labelled_by(name_label.id);
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Encrypter", options, Box::new(|_cc| Box::<MyApp>::default()))
}
