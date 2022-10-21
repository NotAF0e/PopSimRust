use rand::Rng;
use std::str;
use std::time::Instant;
use indicatif::*;
// use plotters;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i64,
    name: &'static str,
    gender: u8,
    age: i16,
    love_vec: Vec<i64>,
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
            love_vec: vec![-1, 100],
        };

        temp_person
    }

    pub fn update_sim(&mut self) {
        for id in 0..self.people.len() {
            if self.people[id].age != -1 {
                // Ages all people by 1 month
                // println!("{:?}", people_temp);
                self.people[id].age += 1;

                if self.people[id].love_vec[0] == -1 && self.people[id].age > 12 * 12 {
                    // Creates a random number to chose a lover for person
                    let lover = rand::thread_rng().gen_range(0..self.people.len());
                    // println!("{}", lover);

                    // If the person is not the lover and if the person does not have a lover one is given
                    if lover != id && self.people[lover].love_vec[0] == -1 && self.people[id].gender != self.people[lover].gender {
                        self.people[id].love_vec[0] = lover as i64;
                        self.people[lover].love_vec[0] = id as i64;
                    }
                }

                if self.people[id].love_vec[1] != -1 {
                    let baby_chance = rand::thread_rng().gen_range(0..1000);
                    if baby_chance < 6 {
                        // Creates a baby!!!
                        let gender = rand::thread_rng().gen_range(0..2);
                        let john: Person = self.create_person(gender);
                        self.people.push(john);
                    }
                }
                if self.people[id].age > 12 * 30 { self.people[id].age = -1; }
                if self.people[id].love_vec[0] != -1 && self.people[self.people[id].love_vec[0] as usize].age == -1
                { self.people[id].love_vec[0] = -1; }

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
                  Lover: {:?}",
                self.people[id].id,
                self.people[id].name,
                self.people[id].age as f32 / 12.0,
                self.people[id].gender,
                self.people[id].love_vec
            )
        }
    }
}


fn main() {
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

    let years = 200;
    let bar = ProgressBar::new(12 * years);
    bar.set_style(ProgressStyle::with_template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());
    for _ in 0..12 * years {
        sim.update_sim();
        bar.inc(1);
    }
    bar.finish_and_clear();

    let duration = start.elapsed();

    // sim.print_people();

    println!("People: {:?}", sim.people.len());

    // Time took to complete code
    println!("Time taken to calculate: {:?}", duration);
}
