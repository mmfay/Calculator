use std::collections::VecDeque;
use eframe::{egui, App, Frame};

#[derive(Default)]

pub struct MyApp {
    name: String,
    counter: i32,
    current_number: String,
    queue: VecDeque<String>,
    stack_op: Vec<String>,
    stack_nums: Vec<i32>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("7")).clicked() {
                    self.current_number.push_str("7");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("8")).clicked() {
                    self.current_number.push_str("8");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("9")).clicked() {
                    self.current_number.push_str("9");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("x")).clicked() {
                    self.queue.push_back("x".to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("4")).clicked() {
                    self.current_number.push_str("4");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("5")).clicked() {
                    self.current_number.push_str("5");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("6")).clicked() {
                    self.current_number.push_str("6");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("-")).clicked() {
                    self.queue.push_back("-".to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("1")).clicked() {
                    self.current_number.push_str("1");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("2")).clicked() {
                    self.current_number.push_str("2");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("3")).clicked() {
                    self.current_number.push_str("3");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("+")).clicked() {
                    self.queue.push_back("+".to_string());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([40.0,40.0], egui::Button::new("0")).clicked() {
                    self.current_number.push_str("0");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new(".")).clicked() {
                    self.current_number.push_str(".");
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("=")).clicked() {
                    self.equal();
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("/")).clicked() {
                    self.queue.push_back("/".to_string());
                }
            });

        });
    }

}

impl MyApp {
    fn equal(&mut self) {
        println!("triggering calc");
        while self.queue.len() > 0 {
            let tmp = self.queue.pop_front();
            println!("{:?}",tmp);
        }
    }
}