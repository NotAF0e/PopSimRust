#![windows_subsystem = "windows"] // Hide console window on Windows

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use std::ops::RangeInclusive;
use rand::Rng;
use std::str;
use std::convert::From;

use eframe::egui;
use eframe::emath::Align;
use egui::{Color32, Vec2, Visuals,
           plot::{Plot, PlotPoints, Line}};


#[derive(Clone)]
pub struct AppData {
    app_scale: f32,
}

// Person data struct
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Person {
    id: i64,
    name: &'static str,
    // In months
    age: i16,
    sex: Sex,
    // Details refers to simulation detail like fertility CHANGE WITH MORTALITY SOON
    details: Vec<f32>,
    love_vec: Vec<i64>,
    seed: f32,
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone)]
struct Sim {
    population: Arc<Mutex<i64>>,
    people: Arc<Mutex<Vec<Person>>>,
    graph_data: Vec<[f64; 2]>,
}

#[derive(Clone)]
struct Checks {
    data: Vec<i32>,
    start_months: i32,
}

impl Sim {
    pub fn create_person(&mut self, sex: Sex) -> Person {
        *self.population.lock().unwrap() += 1;

        let temp_person: Person = Person {
            id: *self.population.lock().unwrap(),
            name: "John",
            age: 0,
            sex,
            details: vec![0.0],
            love_vec: vec![-1],

            // Seed is for random values which will stay consistent
            seed: rand::thread_rng().gen_range(0.1..100.0),
        };

        temp_person
    }

    pub fn update_sim(&mut self) {
        let mut people = self.people.lock().unwrap().clone();

        for id in 0..people.len() {
            if people[id].age != -1 {

                // Ages all people by 1 month
                people[id].age += 1;

                // println!("{:?}", people_temp);

                // Chooses people what will have babies
                if people[id].love_vec[0] == -1 && people[id].age > 12 * 12 {

                    // Creates a random number to chose a lover for person
                    let lover = rand::thread_rng().gen_range(0..people.len());

                    // println!("{}", lover);

                    // If the person is not the lover and if the person does not have a lover one is given
                    if lover != id && people[lover].love_vec[0] == -1 &&
                        people[id].sex != people[lover].sex && rand::thread_rng().gen_range(
                        0.0..100.0) >= 95.0 {
                        people[id].love_vec[0] = lover as i64;
                        people[lover].love_vec[0] = id as i64;
                    }
                }


                // Remove the lover from love_vec if they are dead
                match people.get(people[id].love_vec[0] as usize) {
                    Some(_loved_one) => {}
                    None => {
                        if people[id].love_vec[0] != -1 {
                            people[id].love_vec[0] = -1
                        }
                    }
                }


                // println!("{}", self.people.len());

                // Changes id to -1 for people who will be killed/removed from vec
                if people[id].age > 70 * 12 { // Age of death in months
                    people[id].age = -1;
                }
            }
        }


        // Creating babies
        for id in 0..people.len() {
            if people[id].age > 12 * 12 && people[id].love_vec[0] != -1 {

                // Divide top range buy 12 to get amount of average days that a woman can reproduce for
                let baby_chance = rand::thread_rng().gen_range(0.0..350.0);
                if baby_chance <= (people[id].details[0] as f32) {
                    // Creates a baby!!!
                    let sex: Sex = if rand::random::<f32>() < 0.5 {
                        Sex::Male
                    } else {
                        Sex::Female
                    };

                    let john: &Person = &self.create_person(sex);

                    people.push(john.clone());
                }
            }
        }
    }

    pub fn update_details(&mut self) {
        let mut people = self.people.lock().unwrap();
        for id in 0..people.len() {
            if people[id].age != -1 {
                let age = people[id].age;
                let fertility = if people[id].sex == Sex::Female {

                    // To get the average child/woman add all bellow fertilises and divide by 6
                    if age < 20 * 12 {
                        1.1
                    } else if age < 30 * 12 {
                        3.0
                    } else if age < 40 * 12 {
                        3.8
                    } else if age < 50 * 12 {
                        2.0
                    } else if age < 60 * 12 {
                        1.0
                    } else {
                        0.3
                    }
                } else {
                    0.0
                };
                people[id].details[0] = fertility;
            };
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: true,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(1500_f32, 750_f32)),
        min_window_size: Option::from(Vec2::new(600_f32, 400_f32)),
        max_window_size: None,
        resizable: true,
        transparent: true,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
        event_loop_builder: None,
        shader_version: None,
        centered: true,
    };
    let mut app = App::default();

    // Does simulation calculations
    thread::spawn(move || {
        loop {
            // Creates Adam and Eve
            if app.checks.data[0] == 0 {
                for _ in 0..1 { // Manually change amount of people at start here
                    let john: Person = app.sim_data.create_person(Sex::Male);
                    let john2: Person = app.sim_data.create_person(Sex::Female);
                    app.sim_data.people.lock().unwrap().push(john);
                    app.sim_data.people.lock().unwrap().push(john2);
                }

                app.checks.data[0] = 1;
            }

            if app.checks.data[1] != 0 {
                app.sim_data.update_sim();
                app.sim_data.update_details();

                // Graph data pushing
                app.sim_data.graph_data.push([
                    app.checks.start_months as f64 - app.checks.data[1] as f64,
                    app.sim_data.people.lock().unwrap().len() as f64]);

                app.sim_data.people.lock().unwrap().retain(|person| person.age != -1);
                app.checks.data[1] -= 1;
            }
        }
    });

    // Runs application
    eframe::run_native(
        "Test",
        options,
        Box::new(move |_cc| Box::new(app.clone())),
    );
}

#[derive(Clone)]
pub struct App {
    app_data: AppData,
    sim_data: Sim,
    checks: Checks,
}

impl Default for App {
    fn default() -> Self {
        Self {
            app_data: AppData {
                app_scale: 1.1,
            },
            sim_data: Sim {
                people: Arc::new(Mutex::new(vec![])),
                population: Arc::new(Mutex::new(-1)),
                graph_data: vec![],
            },
            // Check for spawning Adam and Eve, months, start button
            checks: Checks {
                data: vec![0, 480, 0],
                start_months: 0, // To change this val just change Checks::data[1]
            },
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.app_data.app_scale);
        egui::CentralPanel::default().show(ctx, |ui| {

            // Bottom settings panel
            egui::TopBottomPanel::bottom("settings").show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    egui::CollapsingHeader::new("THEME")
                        .show(ui, egui::
                        widgets::global_dark_light_mode_buttons);

                    egui::CollapsingHeader::new("APPLICATION SIZE")
                        .show(ui, |ui|
                            ui.add(egui::Slider::new(&mut self.app_data.app_scale,
                                                     RangeInclusive::new(0.5, 2.0))));
                });
            });

            // Setting the start settings
            if self.checks.data[2] == 0 {
                self.checks.start_months = self.checks.data[1];

                egui::Grid::new("start_settings_1").show(ui, |ui| {
                    ui.add(egui::Label::new("Number of months to simulate(0 - 4800):"));
                    ui.add(egui::DragValue::new(&mut self.checks.data[1])
                        .clamp_range(RangeInclusive::new(0, 4800)));
                    ui.end_row();
                    ui.label(egui::RichText::new(format!("Years: {}", self.checks.start_months / 12)));
                    ui.end_row();
                });
                ui.add_space(15.0);

                if ui.button("Begin simulation").clicked() {
                    self.checks.data[2] = 1;
                }
            }


            if self.checks.data[2] == 1 {
                ui.label(egui::RichText::new(
                    format!("Population: {}", self.sim_data.people.lock().unwrap().len())).size(125.0));

                ui.label(egui::RichText::new(
                    format!("Months Passed: {}", self.checks.start_months - self.checks.data[1])).size(25.0));

                ui.label(egui::RichText::new(
                    format!("Months left: {}", self.checks.data[1])).size(15.0));

                // Simulation completion text
                if self.checks.data[1] == 0 {
                    ui.add_space(5.0);
                    if ui.style_mut().visuals == Visuals::light() {
                        ui.colored_label(Color32::from_rgb(0, 0, 204),
                                         "Simulation completed :)");
                    } else {
                        ui.colored_label(Color32::from_rgb(128, 255, 0),
                                         "Simulation completed :)");
                    }
                }

                // Plot which shows population through time
                egui::Window::new("Plot - Population against months").show(ctx, |ui| {
                    let data: PlotPoints = PlotPoints::new(self.sim_data.graph_data.clone());
                    let line = Line::new(data);
                    Plot::new("plot").view_aspect(2.0)
                        .allow_drag(false)
                        .allow_scroll(false)
                        .allow_zoom(false)
                        .allow_boxed_zoom(false)
                        .allow_double_click_reset(false)
                        .show(ui, |plot_ui| plot_ui.line(line));
                });

                // A table with all the people in the simulation
                egui::SidePanel::right("Table").show(ctx, |ui| {
                    let text_style = egui::TextStyle::Body;
                    let row_height = ui.text_style_height(&text_style);

                    egui::ScrollArea::vertical().stick_to_bottom(true).auto_shrink([false; 2]).show_rows(
                        ui,
                        row_height,
                        self.sim_data.people.lock().unwrap().len(),
                        |ui, row_range| {
                            for id in row_range {
                                let people = self.sim_data.people.lock().unwrap();
                                let text = format!("[ID: {:?}] Name: {:?} |  Age: {:?} | Sex: {:?} | \
                                    Details: {:?} | Lover(Lover's id, Affection): {:?} | Seed: {:?}",
                                                   people[id].id,
                                                   people[id].name,
                                                   (people[id].age as f32 / 12.0) as i32,
                                                   people[id].sex,
                                                   people[id].details,
                                                   people[id].love_vec,
                                                   people[id].seed);
                                ui.label(text);
                            }
                        },
                    );
                });
            }


            ui.label(format!("{:?}", self.sim_data.people.lock().unwrap()));
            ui.label(format!("TEST"));
        });
    }
}