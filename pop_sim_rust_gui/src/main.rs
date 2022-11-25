// GUI VERSION

use rand::Rng;
use std::str;
use std::time::Instant;
use std::{thread, time};
use std::sync::Arc;
use eframe::egui;
use egui::plot::{Plot, Line};


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

                // Randomly removes health from a person depending on healthcare range values
                // self.people[id].stats[0] -= rand::thread_rng().gen_range(
                //     world.healthcare_death_range[0]..world.healthcare_death_range[1]);

                // println!("{}", self.people[id].seed + world.food);

                // Removes or adds health and happiness using seed and global food amount
                if self.people[id].seed + world.food <= 120.0 {
                    self.people[id].stats[0] -= rand::thread_rng().gen_range(
                        0.0..5.0);
                    self.people[id].stats[1] -= rand::thread_rng().gen_range(
                        0.0..1.5);
                } else {
                    self.people[id].stats[0] += rand::thread_rng().gen_range(
                        0.0..0.7);
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


    pub fn print_people(&self) {
        println!("\n**~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~**");
        for id in 0..self.people.len() {
            println!("------------------------------------------");
            println!(
                "[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n\
                  Lover(Lover's id, Affection): {:?}\n\
                  Stats(Health, Happiness): {:?}\n\
                  Seed: {:?}",
                self.people[id].id,
                self.people[id].name,
                self.people[id].age as f32 / 12.0,
                self.people[id].gender,
                self.people[id].love_vec,
                self.people[id].stats,
                self.people[id].seed
            )
        }
    }
}

fn main() {
    let mut world = World {
        name: "Earth",
        age: 4_543_000_000,

        // Available globally on average for each person.
        // 100 would be the exact amount so 75 would be too little
        // The randomness will be consistent using person.seed
        food: 75.0,
        healthcare_death_range: vec![0.0, 0.2], // Per month
    };

    let mut sim = Sim {
        people: vec![],
        population: -1,
    };

    let start = Instant::now();

    pub struct Application {
        test1: f32,
        sim_data: Sim,
        world_data: World,
        checks: Vec<i32>,
    }

    pub enum TextStyle {
        Small,
        Body,
        Monospace,
        Button,
        Heading,
        Name(Arc<str>),
    }

    impl eframe::App for Application {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ctx.set_pixels_per_point(5.0);
                if self.checks[0] == 0 {
                    let john: Person = self.sim_data.create_person(0);
                    let john2: Person = self.sim_data.create_person(1);
                    self.sim_data.people.push(john);
                    self.sim_data.people.push(john2);
                    self.checks[0] = 1;
                }

                ui.horizontal(|ui| {
                    ui.heading(format!("Population: {}", self.sim_data.people.len()));
                });

                if self.checks[1] != 0 {
                    self.sim_data.update_sim(&self.world_data);
                    self.checks[1] -= 1
                }


                ui.label(egui::RichText::new(
                    format!("Months Passed: {}", self.checks[1])).size(7.5));


                ui.label(egui::RichText::new(
                    format!("Months left: {}", 2400 - self.checks[1])).size(2.5));


                // ui.add(egui::Slider::new(&mut self.test1, 0.0..=120.0).text("age"));
                // if ui.button("+ 1").clicked() {
                //     self.test1 += 1.0;
                // }

                ctx.request_repaint();
            });
        }
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                test1: 0.0,
                sim_data: Sim {
                    people: vec![],
                    population: -1,
                },
                world_data: World {
                    name: "Earth",
                    age: 4_543_000_000,
                    food: 75.0,
                    healthcare_death_range: vec![0.0, 0.2], // Per month
                },
                // check for spawning Adam and Eve, months
                checks: vec![0, 2400],
            }
        }
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PopSim",
        options,
        Box::new(|_cc| Box::new(Application::default())),
    );


    let john: Person = sim.create_person(0);

    let john2: Person = sim.create_person(1);
    sim.people.push(john);
    sim.people.push(john2);


    let years = 200; // Change this if you want more simulation time

    // Simulate 'years' amount of years
    for _ in 0..12 * &years {
        sim.update_sim(&world);
    }

    for id in 0..sim.people.len() {
        if id < sim.people.len() && sim.people[id].love_vec[0] != -1
            && sim.people[id].age > 30 * 12
            || sim.people[id].stats[0] <= 0.0
            || (sim.people[id].age <= 5 * 12 && world.food < 30.0) {
            sim.people[id].age = -1;
        }
    }

    sim.people.retain(|person| person.age != -1);

    let duration = start.elapsed();

    sim.print_people();

    println!("People: {:?}", sim.people.len());

    // Time took to complete code
    println!("Time taken to calculate: {:#?}", duration);

    thread::sleep(time::Duration::from_secs(20));
}
