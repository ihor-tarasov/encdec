use crate::driver::{self, Driver};
use eframe::egui::{self, Ui};

#[derive(PartialEq, Debug)]
pub enum Method {
    XOR,
    Addict,
    Subtract,
}

impl Method {
    pub fn process(&self, key: &str, input: &mut [u8]) {
        let mut driver: Box<dyn Driver> = match self {
            Method::XOR => Box::new(driver::xor(key)),
            Method::Addict => Box::new(driver::addict(key)),
            Method::Subtract => Box::new(driver::subtract(key)),
        };

        driver.feed(input);
    }

    pub fn show(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Method")
            .selected_text(format!("{:?}", self))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, Method::XOR, "XOR");
                ui.selectable_value(self, Method::Addict, "Addict");
                ui.selectable_value(self, Method::Subtract, "Subtract");
            });
    }
}
