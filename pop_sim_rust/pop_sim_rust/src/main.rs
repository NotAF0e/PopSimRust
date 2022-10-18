use rand::Rng;
use std::str;
use std::time::Instant;
// use plotters;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Person data struct
#[derive(Debug)]
pub struct Person {
    id: u64,
    name: &'static str,
    gender: u8,
    age: i16,
    love_vec: Vec<i64>,
}

struct Sim {
    population: u64,
    people: Vec<Person>,
}

impl Sim {
    pub fn create_person(&mut self) -> Person {
        self.population += 1;
        let temp_person: Person = Person {
            id: self.population,
            name: "John",
            gender: 0,
            age: 0,
            love_vec: vec![-1, 100],
        };

        temp_person
    }
    pub fn update_sim(&mut self, mut steps: i32) -> i32 {
        for id in 0..self.people.len() {
            if self.people[id].age != -1 {
                // Ages all people by 1 month
                // println!("{:?}", people_temp);
                self.people[id].age += 1;

                if self.people[id].love_vec[0] == -1 {
                    // Creates a random number to chose a lover for person
                    let lover = rand::thread_rng().gen_range(0..=self.people.len()) as i64;

                    // If the person is not the lover and if the person does not have a lover one is given
                    if lover != id as i64 && self.people[id].love_vec[0] == -1 {
                        self.people[id].love_vec[0] = lover;
                        steps += 1;
                    }
                    steps += 1;
                }

                if self.people[id].love_vec[1] as i32 != -1 {
                    let baby_chance = rand::thread_rng().gen_range(0..100) as u32;
                    if baby_chance < 2 {
                        // Creates a baby!!!
                        let john: Person = self.create_person();
                        self.people.push(john);
                        steps += 1;
                    }
                }
            if self.people[id].age > 12 * 30 {
                self.people[id].age = -1;
            }
            }

            steps += 1;
        }
        steps
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
                self.people[id].age,
                self.people[id].gender,
                self.people[id].love_vec
            )
        }
    }
}

fn main() {
    let mut sim = Sim {
        people: vec![],
        population: 0,
    };

    let start = Instant::now();

    let john: Person = sim.create_person();

    let john2: Person = sim.create_person();
    sim.people.push(john);
    sim.people.push(john2);
    sim.print_people();
    let mut steps = 0;

    for _ in 0..12 * 60 {
        steps = sim.update_sim(steps);
    }

    let duration = start.elapsed();

    println!("\nPeople: {:?} | Steps: {}", sim.people.len(), steps);
    println!(
        "The memory size of POPULATION is {}",
        sim.people.len() * std::mem::size_of::<Person>()
    );

    // Time took to complete code
    println!("Time taken to calculate: {:?}", duration);
    // print_people();
}
