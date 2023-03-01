// This module of 'simulation' handles epidemics

use crate::simulation::*;

use std::{ ops::Range };

#[derive(Clone)]
pub struct Epidemic {
    pub population_infected: bool,
    pub population_cured: bool,
    pub cure_produced: bool,

    pub cure_remaining_time: i8,

    pub infection_range: Range<f32>,
    pub lethality: f32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EpidemicDetails {
    pub has_disease: bool,
    pub has_cure: bool,
    pub people_infected: i16,
}

impl Epidemic {
    pub fn update_epidemic(
        &mut self,
        id: usize,
        sim: &mut Sim,
        people: &mut Vec<Person>
    ) -> Vec<Person> {
        let people_mut = people.clone();

        // Initial epidemic start
        if sim.progress_epidemic && !self.population_infected {
            self.population_infected = true;
            people[rand::thread_rng().gen_range(0..people_mut.len())].epidemic.has_disease = true;

            println!("Infected");
        }

        // Main loop which will infect people who are not cured
        if
            !sim.people[id].epidemic.has_cure &&
            sim.progress_epidemic &&
            self.population_infected &&
            !self.cure_produced &&
            !self.population_cured
        {
            if sim.people[id].epidemic.has_disease && sim.people[id].epidemic.people_infected > 6 {
            }
        }
        if sim.people[id].epidemic.has_cure {
            sim.people[id].epidemic.has_disease = false;
        }

        // Stops epidemic if nobody is infected
        for person in people.clone() {
            if person.epidemic.has_disease {
                break;
            }
            self.end_epidemic(sim);
        }
        return people.to_vec();
    }

    pub fn begin_cure(&mut self) {}

    pub fn end_epidemic(&mut self, sim: &mut Sim) {
        sim.progress_epidemic = false;
        sim.epidemic = Box::new(Epidemic {
            population_infected: false,
            population_cured: false,
            cure_produced: false,

            cure_remaining_time: 100,

            infection_range: 0.0..0.0,
            lethality: 0.0,
        });
    }
}