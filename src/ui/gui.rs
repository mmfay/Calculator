use std::collections::VecDeque;
use eframe::{egui, App, Frame};

#[derive(Default)]

pub struct MyApp {
    name: String,
    counter: i32,
    current_number: String,
    queue: VecDeque<String>,
    stack_op: Vec<String>,
    stack_nums: Vec<f64>,
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
                    self.operation();
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
                    self.operation();
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
                    self.operation();
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
                    self.operation();
                    self.equal();
                }
                if ui.add_sized([40.0,40.0], egui::Button::new("/")).clicked() {
                    self.operation();
                    self.queue.push_back("/".to_string());
                }
            });

        });
    }

}

impl MyApp {
    fn equal(&mut self) {
        println!("triggering calc");

        // clear old stacks
        self.stack_nums.clear();
        self.stack_op.clear();

        // fill stacks from queue into separate buckets of ops and nums
        while let Some(token) = self.queue.pop_front() {
            if let Ok(num) = token.parse::<f64>() {
                self.stack_nums.push(num);
            } else {
                self.stack_op.push(token);
            }
        }

        // First pass: handle multiplication and division
        let mut nums = vec![self.stack_nums[0]];
        let mut ops = vec![];

        // i = index, op = element (operation), multiplication and division section
        for (i, op) in self.stack_op.iter().enumerate() {

            let b = self.stack_nums[i + 1];

            match op.as_str() {

                // if op is multiplication, take a * b and push to nums.
                "x" => {

                    let a = nums.pop().unwrap();
                    nums.push(a * b);

                },

                // if op is division, take a / b and push to nums.
                "/" => {

                    if b == 0.0 {
                        println!("Division by zero!");
                        return;
                    }
                    let a = nums.pop().unwrap();
                    nums.push(a / b);

                },

                // if addition or subtraction, dont do op until all multiplication and division yet
                "+" | "-" => {

                    nums.push(b);
                    ops.push(op.clone());

                },

                // in case any new operator is added and not handled
                _ => {

                    println!("Unknown operator: {}", op);
                    return;

                }

            }
        }

        // i = index, op = element (operation), addition and subtraction section
        let mut result = nums[0];
        for (i, op) in ops.iter().enumerate() {
            let b = nums[i + 1];
            match op.as_str() {
                "+" => result += b,
                "-" => result -= b,
                _ => unreachable!(),
            }
        }

        println!("Result: {}", result);

        // Show result and reset
        self.current_number = result.to_string();
        self.queue.clear();
        self.stack_nums.clear();
        self.stack_op.clear();
    }
    fn operation(&mut self) {
        self.queue.push_back(self.current_number.clone());
        self.current_number = "".to_string();
    }
}