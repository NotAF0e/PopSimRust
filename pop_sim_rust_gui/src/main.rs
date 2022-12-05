// GUI VERSION

use std::ops::RangeInclusive;
use rand::Rng;
use std::str;
use eframe::egui;
use egui::{Color32, Vec2, Visuals};


// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i64,
    name: &'static str,
    gender: u8,
    age: i16,
    stats: Vec<f32>,
    love_vec: Vec<i64>,
    seed: f32,
}

#[derive(Debug)]
pub struct World {
    name: &'static str,
    age: i64,
    food: f32,
    healthcare_death_range: Vec<f32>,
}

struct Sim {
    population: i64,
    people: Vec<Person>,
}

struct Checks {
    vec: Vec<i32>,
    start_months: i32,
}

impl Sim {
    pub fn create_person(&mut self, gender: u8) -> Person {
        self.population += 1;
        let temp_person: Person = Person {
            id: self.population,
            name: "John",
            gender,
            age: 0,

            // Health, Happiness
            stats: vec![100.0, 100.0],
            love_vec: vec![-1, 100],

            // Seed is for random values which will stay consistent
            seed: rand::thread_rng().gen_range(1.0..100.0),
        };

        temp_person
    }

    pub fn update_sim(&mut self, world: &World) {
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
                    if lover != id && self.people[lover].love_vec[0] == -1 &&
                        self.people[id].gender != self.people[lover].gender && rand::thread_rng().gen_range(
                        0.0..100.0) >= 95.0 {
                        self.people[id].love_vec[0] = lover as i64;
                        self.people[lover].love_vec[0] = id as i64;
                    }
                }


                // Removes or adds health and happiness using seed and global food amount
                if self.people[id].seed + world.food <= 120.0 {
                    self.people[id].stats[0] -= rand::thread_rng().gen_range(
                        0.0..1.0);
                    self.people[id].stats[1] -= rand::thread_rng().gen_range(
                        0.0..0.7);
                } else {
                    self.people[id].stats[0] += rand::thread_rng().gen_range(
                        0.0..1.0);
                    self.people[id].stats[1] += rand::thread_rng().gen_range(
                        0.0..0.5);
                }

                // Resets max values
                if self.people[id].stats[0] > 100.0 {
                    self.people[id].stats[0] = 100.0
                }
                if self.people[id].stats[1] > 100.0 {
                    self.people[id].stats[1] = 100.0
                }

                // println!("{}", self.people.len());

                // Changes id to -1 for people who will be killed/removed from vec
                if id < self.people.len() && self.people[id].love_vec[0] != -1
                    && self.people[id].age > 30 * 12
                    || self.people[id].stats[0] <= 0.0
                    || (self.people[id].age == 0 && world.food < 30.0) {
                    self.people[id].age = -1;
                }
            }
        }

        // Creating babies
        for _ in 0..self.people.len() {
            let baby_chance = rand::thread_rng().gen_range(0..10000);
            if baby_chance < 40 {
                // Creates a baby!!!
                let gender = rand::thread_rng().gen_range(0..2);
                let john: Person = self.create_person(gender);

                self.people.push(john);
            }
        }
    }


    // pub fn print_people(&self) {                                   FOR DEBUG!
    //     println!("\n**~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~**");
    //     for id in 0..self.people.len() {
    //         println!("------------------------------------------");
    //         println!(
    //             "[ID: {:?}]\n\
    //               Name: {:?}\n\
    //               Age: {:?}\n\
    //               Gender: {:?}\n\
    //               Lover(Lover's id, Affection): {:?}\n\
    //               Stats(Health, Happiness): {:?}\n\
    //               Seed: {:?}",
    //             self.people[id].id,
    //             self.people[id].name,
    //             self.people[id].age as f32 / 12.0,
    //             self.people[id].gender,
    //             self.people[id].love_vec,
    //             self.people[id].stats,
    //             self.people[id].seed
    //         )
    //     }
    // }
}

fn main() {
    pub struct Application {
        sim_data: Sim,
        world_data: World,
        checks: Checks,
    }

    impl eframe::App for Application {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {

                // Setting the start settings
                if self.checks.vec[2] == 0 {
                    ui.add(egui::Slider::new(&mut self.checks.vec[1], RangeInclusive::new(0, 1000)));
                    self.checks.start_months = self.checks.vec[1];
                    if ui.button("Begin simulation").clicked() {
                        self.checks.vec[2] = 1;
                    }
                }

                // Start settings. Such as amount of people and months of creation
                if self.checks.vec[0] == 0 {
                    let john: Person = self.sim_data.create_person(0);
                    let john2: Person = self.sim_data.create_person(1);
                    self.sim_data.people.push(john);
                    self.sim_data.people.push(john2);
                    self.checks.vec[0] = 1;
                }

                if self.checks.vec[2] == 1 {
                    if self.checks.vec[1] != 0 {
                        self.sim_data.update_sim(&self.world_data);

                        for id in 0..self.sim_data.people.len() - 1 {
                            if id < self.sim_data.people.len() && self.sim_data.people[id].love_vec[0] != -1
                                && self.sim_data.people[id].age > 30 * 12
                                || self.sim_data.people[id].stats[0] <= 0.0
                            {
                                self.sim_data.people[id].age = -1;
                            }
                        }

                        self.sim_data.people.retain(|person| person.age != -1);
                        self.checks.vec[1] -= 1;
                    }


                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label(egui::RichText::new(
                            format!("Population: {}", self.sim_data.people.len())).size(125.0));

                        ui.label(egui::RichText::new(
                            format!("Months Passed: {}", self.checks.start_months - self.checks.vec[1])).size(25.0));

                        ui.label(egui::RichText::new(
                            format!("Months left: {}", self.checks.vec[1])).size(15.0));


                        if self.checks.vec[1] == 0 {
                            ui.add_space(5.0);
                            if ui.style_mut().visuals == Visuals::light() {
                                ui.colored_label(Color32::from_rgb(0, 0, 204), "Simulation completed :)");
                            } else {
                                ui.colored_label(Color32::from_rgb(128, 255, 0), "Simulation completed :)");
                            }
                        }
                    });
                }

                egui::TopBottomPanel::bottom("settings").show(ctx, |ui| {
                    egui::CollapsingHeader::new("THEME")
                        .show(ui, |ui| egui::
                        widgets::global_dark_light_mode_buttons(ui));
                });

                // println!("{:?}", self.sim_data.people);


                ctx.request_repaint();
            });
        }
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                sim_data: Sim {
                    people: vec![],
                    population: -1,
                },
                world_data: World {
                    name: "Earth",
                    age: 4_543_000_000 * 12,
                    food: 100.0,
                    healthcare_death_range: vec![0.0, 0.2], // Per month
                },
                // check for spawning Adam and Eve, months, start button
                checks: Checks {
                    vec: vec![0, 100, 0],
                    start_months: 100,
                },
            }
        }
    }

    let mut options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(925 as f32, 500 as f32)),
        min_window_size: Option::from(Vec2::new(600 as f32, 400 as f32)),
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
    };

    eframe::run_native(
        "PopSim",
        options,
        Box::new(|_cc| Box::new(Application::default())),
    );
}