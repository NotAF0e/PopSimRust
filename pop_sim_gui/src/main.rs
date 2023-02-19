// GUI VERSION

// TODO:
// -[x] Migrant system
// -[x] Differing death causes(random old age death)
// -[x] Simulation info window for end of simulation
// -[] Epidemics
// -[] More start settings
// -[x] Quality of life(Pausing [x], better table (table v2) [x])

#![windows_subsystem = "windows"]

use rand::Rng;
use rand::seq::IteratorRandom;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use std::{
    convert::From,
    fs::File,
    io::{ BufRead, BufReader },
    ops::RangeInclusive,
    time::{ Duration, Instant },
};

use eframe::egui;
use eframe::emath::Align;
use egui::{ plot::{ Line, Plot, PlotPoints }, Color32, Pos2, Vec2, Visuals };

// Person data struct
#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    id: i64,
    name: String,
    // In months
    age: Option<i16>,
    sex: Sex,
    fertility: f32,
    lover: Option<i64>,
    has_disease: bool,
    seed: f32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sex {
    Male,
    Female,
}

struct Sim {
    population: i64,
    people: Vec<Person>,

    graph_data: Vec<[f64; 2]>,

    months_to_sim: i32,
    sim_running: bool,
    lover_fix: bool,
    start_months: i32,
    start_settings_set: bool,
    start_people_created: bool,
    start_pairs_of_people: i32,
}

struct AppData {
    frame_time: Duration,
    app_scale: f32,
    table_shown: bool,
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path).expect("Failed to open icon path").into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() {
    pub struct App {
        app_data: AppData,
        sim_data: Sim,
    }

    // The code which renders the application
    // This section also handles simulation which may be decoupled to increase performance itf
    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            ctx.set_pixels_per_point(self.app_data.app_scale);
            let frame_start = Instant::now();

            egui::CentralPanel::default().show(ctx, |ui| {
                // Bottom settings panel
                egui::TopBottomPanel::bottom("settings").show(ctx, |ui| {
                    // Left to right side ui elements
                    ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
                        egui::CollapsingHeader
                            ::new(egui::RichText::new(format!("THEME")).size(15.0))
                            .show(ui, egui::widgets::global_dark_light_mode_buttons);

                        egui::CollapsingHeader
                            ::new(egui::RichText::new(format!("APPLICATION SIZE")).size(15.0))
                            .show(ui, |ui|
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.app_data.app_scale,
                                        RangeInclusive::new(0.5, 2.0)
                                    )
                                )
                            );
                        egui::CollapsingHeader
                            ::new(egui::RichText::new(format!("DEV SETTINGS")).size(15.0))
                            .show(ui, |ui| {
                                if ui.add(egui::Button::new("Enable/Disable lover fix")).clicked() {
                                    if self.sim_data.lover_fix {
                                        self.sim_data.lover_fix = false;
                                    } else {
                                        self.sim_data.lover_fix = true;
                                    }
                                }
                            });

                        // Right to left side ui elements
                        ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText
                                        ::new(format!("{:?}", self.app_data.frame_time))
                                        .size(15.0)
                                )
                            );
                        });
                    });
                });

                // Setting the start settings
                if !self.sim_data.start_settings_set {
                    self.sim_data.start_months = self.sim_data.months_to_sim;

                    egui::Grid::new("start_settings_1").show(ui, |ui| {
                        ui.add(egui::Label::new("Number of months to simulate(0 - 4800):"));
                        ui.add(
                            egui::DragValue
                                ::new(&mut self.sim_data.months_to_sim)
                                .clamp_range(RangeInclusive::new(0, 4800))
                        );
                        ui.end_row();
                        ui.label(
                            egui::RichText::new(
                                format!("Years: {}", self.sim_data.start_months / 12)
                            )
                        );
                        ui.end_row();
                    });

                    egui::Grid::new("start_settings_2").show(ui, |ui| {
                        ui.add(egui::Label::new("Number of pairs of people to begin with:"));
                        ui.add(
                            egui::DragValue
                                ::new(&mut self.sim_data.start_pairs_of_people)
                                .clamp_range(RangeInclusive::new(0, 1000))
                        );
                        ui.end_row();
                    });

                    ui.add_space(15.0);
                    if ui.button("Begin simulation").clicked() {
                        if !self.sim_data.start_people_created {
                            // Creates Adam and Eve
                            for _ in 0..self.sim_data.start_pairs_of_people {
                                let adam: Person = self.sim_data.create_person(Sex::Male);
                                let eve: Person = self.sim_data.create_person(Sex::Female);
                                self.sim_data.people.push(adam);
                                self.sim_data.people.push(eve);
                            }

                            self.sim_data.start_people_created = true;
                        }
                        self.sim_data.start_settings_set = true;
                    }
                }

                // Main sim update loop screen
                if self.sim_data.start_settings_set && self.sim_data.months_to_sim != 0 {
                    if self.sim_data.months_to_sim != 0 && self.sim_data.sim_running {
                        // Updating the sim
                        if self.sim_data.people.len() != 0 {
                            self.sim_data.update_sim(self.sim_data.lover_fix);
                            self.sim_data.update_fertility();
                            self.sim_data.people.retain(|person| person.age.is_some());

                            self.sim_data.months_to_sim -= 1;

                            // Graph data pushing
                            self.sim_data.graph_data.push([
                                (self.sim_data.start_months as f64) -
                                    (self.sim_data.months_to_sim as f64),
                                self.sim_data.people.len() as f64,
                            ]);
                        } else {
                            ui.colored_label(Color32::from_rgb(222, 0, 0), "Simulation died :(");
                        }
                    }

                    ui.label(
                        egui::RichText
                            ::new(format!("Population: {}", self.sim_data.people.len()))
                            .size(125.0)
                    );
                    ui.label(
                        egui::RichText
                            ::new(
                                format!(
                                    "Months Passed: {}",
                                    self.sim_data.start_months - self.sim_data.months_to_sim
                                )
                            )
                            .size(25.0)
                    );
                    ui.label(
                        egui::RichText
                            ::new(format!("Months left: {}", self.sim_data.months_to_sim))
                            .size(15.0)
                    );

                    if ui.button("Play/Pause").clicked() {
                        if self.sim_data.sim_running {
                            self.sim_data.sim_running = false;
                        } else {
                            self.sim_data.sim_running = true;
                        }
                    }
                    if ui.button("Enable/Disable table").clicked() {
                        if self.app_data.table_shown {
                            self.app_data.table_shown = false;
                        } else {
                            self.app_data.table_shown = true;
                        }
                    }

                    // Plot which shows population through time
                    egui::Window
                        ::new("Plot - Population against months")
                        .default_pos(Pos2 { x: 7.0, y: 250.0 })
                        .show(ctx, |ui| {
                            let data: PlotPoints = PlotPoints::new(
                                self.sim_data.graph_data.clone()
                            );
                            let line = Line::new(data);
                            Plot::new("plot")
                                .view_aspect(2.0)
                                .allow_drag(false)
                                .allow_scroll(false)
                                .allow_zoom(false)
                                .allow_boxed_zoom(false)
                                .allow_double_click_reset(false)
                                .show(ui, |plot_ui| plot_ui.line(line));
                        });

                    // A table with all the people in the simulation
                    if !self.sim_data.people.is_empty() && self.app_data.table_shown {
                        egui::SidePanel::right("Table").show(ctx, |ui| {
                            let text_style = egui::TextStyle::Body;
                            let row_height = ui.text_style_height(&text_style);

                            egui::ScrollArea
                                ::vertical()
                                .stick_to_bottom(true)
                                .auto_shrink([false; 2])
                                .show_rows(
                                    ui,
                                    row_height,
                                    self.sim_data.people.len(),
                                    |ui, row_range| {
                                        for id in row_range {
                                            let text = format!(
                                                "[ID: {:?}] Name: {:?} |  Age: {:?} | Sex: {:?} | \
                                        Fertility: {:?} | Lover: {:?} | Random seed: {:?}",

                                                self.sim_data.people[id].id,
                                                self.sim_data.people[id].name,
                                                ((if self.sim_data.people[id].age.is_some() {
                                                    self.sim_data.people[id].age.unwrap() as f32
                                                } else {
                                                    0.0
                                                }) / 12.0) as i32,
                                                self.sim_data.people[id].sex,
                                                self.sim_data.people[id].fertility,
                                                self.sim_data.people[id].lover,
                                                self.sim_data.people[id].seed
                                            );
                                            let collap_header_text =
                                                self.sim_data.people[id].name.to_string() +
                                                " | Age: " +
                                                &(
                                                    ((if self.sim_data.people[id].age.is_some() {
                                                        self.sim_data.people[id].age.unwrap() as f32
                                                    } else {
                                                        0.0
                                                    }) / 12.0) as i32
                                                ).to_string();
                                            ui.push_id(self.sim_data.people[id].id, |ui| {
                                                egui::CollapsingHeader
                                                    ::new(collap_header_text)
                                                    .open(Some(true))
                                                    .show(ui, |ui| {
                                                        ui.label(text);
                                                    });
                                                ui.separator();
                                            });
                                        }
                                    }
                                );
                        });
                    }
                }

                // Simulation completion screen
                if self.sim_data.months_to_sim == 0 {
                    ui.add_space(5.0);
                    if ui.style_mut().visuals == Visuals::light() {
                        ui.label(
                            egui::RichText
                                ::new(format!("Simulation completed :)"))
                                .size(60.0)
                                .color(Color32::from_rgb(0, 0, 204))
                        );
                    } else {
                        ui.label(
                            egui::RichText
                                ::new(format!("Simulation completed :)"))
                                .size(60.0)
                                .color(Color32::from_rgb(128, 255, 0))
                        );
                    }
                    ui.separator();
                    ui.label(
                        egui::RichText
                            ::new(format!("Simulation stats:"))
                            .size(50.0)
                            .text_style(egui::TextStyle::Heading)
                    );
                    ui.label(
                        egui::RichText
                            ::new(format!("-Population: {}", self.sim_data.people.len()))
                            .size(30.0)
                    );
                    ui.label(
                        egui::RichText
                            ::new(
                                format!(
                                    "-Total people that ever existed: {}",
                                    self.sim_data.people.last().unwrap().id + 1
                                )
                            )
                            .size(30.0)
                    );
                    ui.label(
                        egui::RichText
                            ::new(
                                format!(
                                    "-Months Passed: {}",
                                    self.sim_data.start_months - self.sim_data.months_to_sim
                                )
                            )
                            .size(30.0)
                    );
                }

                self.app_data.frame_time = frame_start.elapsed();
                // println!("{:?}", self.sim_data.people);

                ctx.request_repaint();
            });
        }
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                sim_data: Sim {
                    people: vec![],
                    population: -1,

                    graph_data: vec![],

                    sim_running: true,
                    lover_fix: false,
                    months_to_sim: 2400,
                    start_months: 0,
                    start_settings_set: false,
                    start_people_created: false,
                    start_pairs_of_people: 5,
                },

                // Checks for spawning Adam and Eve, months, start button, amount of pairs, etc
                app_data: AppData {
                    table_shown: false,

                    app_scale: 0.9,
                    frame_time: Duration::new(0, 0),
                },
            }
        }
    }

    // Custom options
    let options = eframe::NativeOptions {
        maximized: true,
        initial_window_size: Option::from(Vec2::new(1500_f32, 750_f32)),
        min_window_size: Option::from(Vec2::new(600_f32, 400_f32)),
        vsync: true,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        centered: true,
        icon_data: Some(load_icon("./PopSimLogo.png")),
        ..Default::default()
    };

    // Runs the application
    eframe::run_native(
        "PopSim",
        options,
        Box::new(|_cc| Box::new(App::default()))
    ).expect("OUCH");
}

impl Sim {
    pub fn create_person(&mut self, sex: Sex) -> Person {
        self.population += 1;
        let name: String = self.generate_name(&sex).unwrap();
        let temp_person: Person = Person {
            id: self.population,
            name,
            age: Some(0),
            sex,
            fertility: 0.0,
            has_disease: false,
            lover: None,

            // Seed is for random values which will stay consistent
            seed: rand::thread_rng().gen_range(0.1..100.0),
        };

        temp_person
    }

    pub fn update_sim(&mut self, lover_fix: bool) {
        for id in 0..self.people.len() {
            if self.people[id].age != None {
                if lover_fix {
                    // Set the lover as None in person.lover if they are dead
                    // THIS IS A VERY INEFFICIENT CHECK
                    if Some(self.people[id].age.unwrap() * 12) > Some(12 * 12) {
                        for person in self.people.clone().into_iter() {
                            if
                                self.people[id].lover.is_some() &&
                                Some(person.id) == self.people[id].lover
                            {
                                if Some(person.id).is_none() {
                                    self.people[id].lover = None;
                                }
                            }
                        }
                    }
                }

                // Ages all people by 1 month
                self.people[id].age = Some(self.people[id].age.unwrap_or(0) + 1);

                // println!("{:?}", people_temp);

                // Chooses people what will have babies
                if self.people[id].lover == None && self.people[id].age > Some(12 * 12) {
                    // Creates a random number to chose a lover for person
                    let lover = rand::thread_rng().gen_range(0..self.people.len());

                    // println!("{}", lover);

                    // If the person is not the lover and if the person does not have a lover one is given
                    if
                        lover != id &&
                        self.people[lover].lover == None &&
                        self.people[id].sex != self.people[lover].sex &&
                        rand::thread_rng().gen_range(0.0..100.0) >= 95.0
                    {
                        self.people[id].lover = Some(self.people[lover].id);
                        self.people[lover].lover = Some(self.people[id].id);
                    }
                }

                // Changes id to -1 for people who will be killed/removed from vec
                let ages = [2, 5, 10, 25, 35, 45, 60, 70, 80, 90];
                let weights = [5, 5, 25, 55, 75, 105, 135, 1050, 350, 150];
                let dist = WeightedIndex::new(&weights).unwrap();
                // println!("{}", ages[dist.sample(&mut rng)]);
                if
                    self.people[id].age > Some(ages[dist.sample(&mut thread_rng())] * 12) &&
                    rand::thread_rng().gen_range(0.0..1.0) > 0.98
                {
                    // Age of death in months
                    self.people[id].age = None;
                }

                // println!("{}", self.people.len());
            }
        }

        // Creating babies
        for id in 0..self.people.len() {
            if self.people[id].age > Some(12 * 12) && self.people[id].lover != None {
                // Divide top range buy 12 to get amount of average days that a woman can reproduce for
                let baby_chance = rand::thread_rng().gen_range(0.0..350.0);
                if baby_chance <= self.people[id].fertility {
                    // Creates a baby!!!
                    let sex: Sex = if rand::random::<f32>() < 0.5 {
                        Sex::Male
                    } else {
                        Sex::Female
                    };

                    let john: Person = self.create_person(sex);

                    self.people.push(john);
                }
            }
        }
    }

    pub fn update_fertility(&mut self) {
        for id in 0..self.people.len() {
            if self.people[id].age != None {
                let age = self.people[id].age;
                let fertility = if self.people[id].sex == Sex::Female {
                    // To get the average child/woman add all bellow fertilises and divide by 6
                    if age < Some(20 * 12) {
                        1.1
                    } else if age < Some(30 * 12) {
                        3.0
                    } else if age < Some(40 * 12) {
                        3.8
                    } else if age < Some(50 * 12) {
                        2.0
                    } else if age < Some(60 * 12) {
                        1.0
                    } else {
                        0.3
                    }
                } else {
                    0.0
                };
                self.people[id].fertility = fertility;
            }
        }
    }

    pub fn generate_name(&mut self, sex: &Sex) -> Option<String> {
        if sex == &Sex::Male {
            let name_f: BufReader<File> = BufReader::new(
                File::open("names/male_names.txt").unwrap_or_else(|_e|
                    panic!("male_names.txt not found!")
                )
            );
            name_f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut rand::thread_rng())
        } else {
            let name_f: BufReader<File> = BufReader::new(
                File::open("names/female_names.txt").unwrap_or_else(|_e|
                    panic!("female_names.txt not found!")
                )
            );
            name_f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut rand::thread_rng())
        }
    }
}