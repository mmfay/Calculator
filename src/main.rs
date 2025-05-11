use eframe::{egui, App, CreationContext, Frame};
use std::collections::VecDeque;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    counter: i32,
    stack: VecDeque<String>,
    stack_op: Vec<String>,
    stack_nums: Vec<i32>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("7")).clicked() {
                    self.stack.push_back("7".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("8")).clicked() {
                    self.stack.push_back("8".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("9")).clicked() {
                    self.stack.push_back("9".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("x")).clicked() {
                    self.stack.push_back("x".to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("4")).clicked() {
                    self.stack.push_back("4".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("5")).clicked() {
                    self.stack.push_back("5".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("6")).clicked() {
                    self.stack.push_back("6".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("-")).clicked() {
                    self.stack.push_back("-".to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("1")).clicked() {
                    self.stack.push_back("1".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("2")).clicked() {
                    self.stack.push_back("2".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("3")).clicked() {
                    self.stack.push_back("3".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("+")).clicked() {
                    self.stack.push_back("+".to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("0")).clicked() {
                    self.stack.push_back("0".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new(".")).clicked() {
                    self.stack.push_back(".".to_string());
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("=")).clicked() {
                    self.equal();
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("/")).clicked() {
                    self.stack.push_back("/".to_string());
                }
            });

        });
    }

}

impl MyApp {
    fn equal(&mut self) {
        println!("triggering calc");
        while self.stack.len() > 0 {
            let tmp = self.stack.pop_front();
            println!("{:?}",tmp);
        }
    }
}
