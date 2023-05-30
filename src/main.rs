mod driver;
mod method;
use eframe::egui;
use method::*;
use std::borrow::Cow;

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
                });

            ui.label("Input:");
            ui.text_edit_multiline(&mut self.input);

            let mut data = match self.input_kind {
                DataKind::Text => self.input.clone().into_bytes(),
                DataKind::Hex => match hex::decode(self.input.as_str()) {
                    Ok(data) => data,
                    Err(error) => {
                        ui.label(format!("{error:?}"));
                        return;
                    }
                },
            };

            ui.label("Key:");
            ui.text_edit_multiline(&mut self.key);

            if self.key.is_empty() {
                ui.label("Key can't be empty");
                return;
            }

            self.method.show(ui);

            self.method.process(self.key.as_str(), data.as_mut_slice());

            self.result_string = match String::from_utf8_lossy(&data) {
                Cow::Borrowed(s) => s.to_string(),
                Cow::Owned(s) => s,
            };

            ui.label("String result:");
            ui.text_edit_multiline(&mut self.result_string);

            self.result_hex = hex::encode(data);

            ui.label("HEX result:");
            ui.text_edit_multiline(&mut self.result_hex);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Encrypter",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
