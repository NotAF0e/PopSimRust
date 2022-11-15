use rand::Rng;
use std::str;
use std::time::Instant;
use indicatif::*;
use std::{thread, time};
// use plotters;


// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i64,
    name: &'static str,
    gender: u8,
    age: i16,
    stats: Vec<f32>,
    love_vec: Vec<i64>,
}

#[derive(Debug)]
pub struct World {
    name: &'static str,
    age: i64,
    food: f32,
    healthcare_death_val: f32,
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
            //         Health, Happiness
            stats: vec![100.0, 100.0],
            love_vec: vec![-1, 100],
        };

        temp_person
    }

    pub fn update_sim(&mut self, healthcare_death_val: f32) {
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
                        self.people[id].gender != self.people[lover].gender {
                        self.people[id].love_vec[0] = lover as i64;
                        self.people[lover].love_vec[0] = id as i64;
                    }
                }

                // Randomly removes health from a person
                self.people[id].stats[0] -= rand::thread_rng().gen_range(0.0..healthcare_death_val);

                // println!("{i}, {}", self.people.len());

                // Changes id to -1 for people who will be killed/removed from vec
                if id < self.people.len() && self.people[id].love_vec[0] != -1
                    && self.people[self.people[id].love_vec[0] as usize].age == -1
                    && self.people[id].stats[0] >= 0.0 {
                    self.people[id].love_vec[0] = -1;
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
                  Stats(Health, Happiness): {:?}",
                self.people[id].id,
                self.people[id].name,
                self.people[id].age as f32 / 12.0,
                self.people[id].gender,
                self.people[id].love_vec,
                self.people[id].stats
            )
        }
    }
}

fn main() {
    let mut world = World {
        name: "Earth",
        age: 4_543_000_000,
        food: 100.0,
        healthcare_death_val: 1.0,
    };

    let mut sim = Sim {
        people: vec![],
        population: -1,
    };

    let start = Instant::now();

    let john: Person = sim.create_person(0);

    let john2: Person = sim.create_person(1);
    sim.people.push(john);
    sim.people.push(john2);
    sim.print_people();
    println!("**~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~**\n");

    let years = 300; // Change this if you want more simulation time

    let bar = ProgressBar::new(12 * years);
    bar.set_style(ProgressStyle::with_template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());
    // Simulate 'years' amount of years
    for _ in 0..12 * years {
        sim.update_sim(world.healthcare_death_val);
        bar.inc(1);
    }
    sim.people.retain(|person| person.age != -1);
    bar.finish_and_clear();

    let duration = start.elapsed();

    // sim.print_people();

    println!("People: {:?}", sim.people.len());

    // Time took to complete code
    println!("Time taken to calculate: {:?}", duration);

    thread::sleep(time::Duration::from_secs(20));
}
