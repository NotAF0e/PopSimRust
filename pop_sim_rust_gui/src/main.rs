// GUI VERSION

// TODO:
// -[] Migrant/Emigrant system
// -[] Mortality
// -[] Differing death causes(random old age death)
// -[] Epidemics
// -[] Outside world influence(Migrant/Emigrant v2, plagues, things occuring outside of sim_region)
// -[] More settings
// -[] Notices(News of important info)
// -[] Quality of life(Pausing [], better table [x], better app sizing [])

#![windows_subsystem = "windows"]

use rand::Rng;
use rand::seq::IteratorRandom;
use std::{ convert::From, ops::RangeInclusive, fs::File, io::{ BufRead, BufReader } };

use eframe::egui;
use eframe::emath::Align;
use egui::{ Color32, Vec2, Visuals, plot::{ Plot, PlotPoints, Line }, Pos2 };

pub struct AppData {
    app_scale: f32,
}

// Person data struct
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Person {
    id: i64,
    name: String,
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
pub enum Sex {
    Male,
    Female,
}

struct Sim {
    population: i64,
    people: Vec<Person>,
    graph_data: Vec<[f64; 2]>,
}

#[derive(Debug)]
pub struct World {
    name: String,
    age: i64,
}

struct Checks {
    data: Vec<i32>,
    start_months: i32,
}

fn main() {
    pub struct App {
        app_data: AppData,
        sim_data: Sim,
        world_data: World,
        checks: Checks,
    }

    // The code which renders the application
    // This section also handles simulation which may be decoupled to increase performance itf
    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            ctx.set_pixels_per_point(self.app_data.app_scale);

            egui::CentralPanel::default().show(ctx, |ui| {
                // Bottom settings panel
                egui::TopBottomPanel::bottom("settings").show(ctx, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        egui::CollapsingHeader
                            ::new("THEME")
                            .show(ui, egui::widgets::global_dark_light_mode_buttons);

                        egui::CollapsingHeader
                            ::new("APPLICATION SIZE")
                            .show(ui, |ui|
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.app_data.app_scale,
                                        RangeInclusive::new(0.5, 2.0)
                                    )
                                )
                            );
                    });
                });

                // Setting the start settings
                if self.checks.data[2] == 0 {
                    self.checks.start_months = self.checks.data[1];

                    egui::Grid::new("start_settings_1").show(ui, |ui| {
                        ui.add(egui::Label::new("Number of months to simulate(0 - 4800):"));
                        ui.add(
                            egui::DragValue
                                ::new(&mut self.checks.data[1])
                                .clamp_range(RangeInclusive::new(0, 4800))
                        );
                        ui.end_row();
                        ui.label(
                            egui::RichText::new(format!("Years: {}", self.checks.start_months / 12))
                        );
                        ui.end_row();
                    });

                    egui::Grid::new("start_settings_2").show(ui, |ui| {
                        ui.add(egui::Label::new("Number of pairs of people to begin with:"));
                        ui.add(
                            egui::DragValue
                                ::new(&mut self.checks.data[3])
                                .clamp_range(RangeInclusive::new(0, 1000))
                        );
                        ui.end_row();
                    });

                    ui.add_space(15.0);
                    if ui.button("Begin simulation").clicked() {
                        self.checks.data[2] = 1;
                    }
                }

                // Creates Adam and Eve
                if self.checks.data[0] == 0 {
                    for _ in 0..self.checks.data[3] {
                        let adam: Person = self.sim_data.create_person(Sex::Male);
                        let eve: Person = self.sim_data.create_person(Sex::Female);
                        self.sim_data.people.push(adam);
                        self.sim_data.people.push(eve);
                    }

                    self.checks.data[0] = 1;
                }

                if self.checks.data[2] == 1 {
                    if self.checks.data[1] != 0 {
                        // Updating the sim
                        self.sim_data.update_sim(&self.world_data);
                        self.sim_data.update_details();
                        self.sim_data.people.retain(|person| person.age != -1);
                        self.checks.data[1] -= 1;

                        // Graph data pushing
                        self.sim_data.graph_data.push([
                            (self.checks.start_months as f64) - (self.checks.data[1] as f64),
                            self.sim_data.people.len() as f64,
                        ]);
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
                                    self.checks.start_months - self.checks.data[1]
                                )
                            )
                            .size(25.0)
                    );

                    ui.label(
                        egui::RichText
                            ::new(format!("Months left: {}", self.checks.data[1]))
                            .size(15.0)
                    );

                    // Simulation completion text
                    if self.checks.data[1] == 0 {
                        ui.add_space(5.0);
                        if ui.style_mut().visuals == Visuals::light() {
                            ui.colored_label(
                                Color32::from_rgb(0, 0, 204),
                                "Simulation completed :)"
                            );
                        } else {
                            ui.colored_label(
                                Color32::from_rgb(128, 255, 0),
                                "Simulation completed :)"
                            );
                        }
                    }

                    // Plot which shows population through time
                    egui::Window
                        ::new("Plot - Population against months")
                        .default_pos(Pos2 { x: 7.0, y: 225.0 })
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
                    if self.sim_data.people.len() != 0 {
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
                                        Details: {:?} | Lover(Lover's id, Affection): {:?} | Seed: {:?}",

                                                self.sim_data.people[id].id,
                                                self.sim_data.people[id].name,
                                                ((self.sim_data.people[id].age as f32) /
                                                    12.0) as i32,
                                                self.sim_data.people[id].sex,
                                                self.sim_data.people[id].details,
                                                self.sim_data.people[id].love_vec,
                                                self.sim_data.people[id].seed
                                            );
                                            let collap_header_text =
                                                self.sim_data.people[id].name.to_string() +
                                                &" | Age: ".to_string() +
                                                &(
                                                    ((self.sim_data.people[id].age as f32) /
                                                        12.0) as i32
                                                ).to_string();

                                            egui::CollapsingHeader
                                                ::new(collap_header_text)
                                                .open(Some(true))
                                                .show(ui, |ui| {
                                                    ui.label(text);
                                                });
                                            ui.separator();
                                        }
                                    }
                                );
                        });
                    }
                }

                // println!("{:?}", self.sim_data.people);

                ctx.request_repaint();
            });
        }
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                app_data: AppData {
                    app_scale: 1.1,
                },
                sim_data: Sim {
                    people: vec![],
                    population: -1,
                    graph_data: vec![],
                },
                world_data: World {
                    name: "Earth".to_string(),
                    age: 0 * 12,
                },

                // Checks for spawning Adam and Eve, months, start button, amount of pairs
                checks: Checks {
                    // These are defaults
                    data: vec![0, 2400, 0, 1],
                    start_months: 0, // To change this val just change Checks::data[1]
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
        ..Default::default()
    };

    // Runs the application
    eframe::run_native(
        "PopSim",
        options,
        Box::new(|_cc| Box::new(App::default()))
    );
}

impl Sim {
    pub fn create_person(&mut self, sex: Sex) -> Person {
        self.population += 1;
        let name: String = self.generate_name(&sex).unwrap();
        let temp_person: Person = Person {
            id: self.population,
            name,
            age: 0,
            sex,
            details: vec![0.0],
            love_vec: vec![-1],

            // Seed is for random values which will stay consistent
            seed: rand::thread_rng().gen_range(0.1..100.0),
        };

        temp_person
    }

    pub fn update_sim(&mut self, _world: &World) {
        for id in 0..self.people.len() {
            if self.people[id].age != -1 {
                // Ages all people by 1 month
                self.people[id].age += 1;

                // println!("{:?}", people_temp);

                // Chooses people what will have babies
                if self.people[id].love_vec[0] == -1 && self.people[id].age > 12 * 12 {
                    // Creates a random number to chose a lover for person
                    let lover = rand::thread_rng().gen_range(0..self.people.len());

                    // println!("{}", lover);

                    // If the person is not the lover and if the person does not have a lover one is given
                    if
                        lover != id &&
                        self.people[lover].love_vec[0] == -1 &&
                        self.people[id].sex != self.people[lover].sex &&
                        rand::thread_rng().gen_range(0.0..100.0) >= 95.0
                    {
                        self.people[id].love_vec[0] = lover as i64;
                        self.people[lover].love_vec[0] = id as i64;
                    }
                }

                // Remove the lover from love_vec if they are dead
                match self.people.get(self.people[id].love_vec[0] as usize) {
                    Some(_loved_one) => {}
                    None => {
                        if self.people[id].love_vec[0] != -1 {
                            self.people[id].love_vec[0] = -1;
                        }
                    }
                }

                // println!("{}", self.people.len());

                // Changes id to -1 for people who will be killed/removed from vec
                if self.people[id].age > 70 * 12 {
                    // Age of death in months
                    self.people[id].age = -1;
                }
            }
        }

        // Creating babies
        for id in 0..self.people.len() {
            if self.people[id].age > 12 * 12 && self.people[id].love_vec[0] != -1 {
                // Divide top range buy 12 to get amount of average days that a woman can reproduce for
                let baby_chance = rand::thread_rng().gen_range(0.0..350.0);
                if baby_chance <= (self.people[id].details[0] as f32) {
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

    pub fn update_details(&mut self) {
        for id in 0..self.people.len() {
            if self.people[id].age != -1 {
                let age = self.people[id].age;
                let fertility = if self.people[id].sex == Sex::Female {
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
                self.people[id].details[0] = fertility;
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
            let name: Option<String> = name_f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut rand::thread_rng());
            return name;
        } else {
            let name_f: BufReader<File> = BufReader::new(
                File::open("names/female_names.txt").unwrap_or_else(|_e|
                    panic!("female_names.txt not found!")
                )
            );
            let name: Option<String> = name_f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut rand::thread_rng());
            return name;
        }
    }
}