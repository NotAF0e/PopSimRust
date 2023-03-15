// This module handles the simulation aspect of PopSim

use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::IteratorRandom,
    thread_rng, Rng,
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Person data struct
#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub id: i64,
    pub name: String,

    // In months
    pub age: Option<i16>,
    pub sex: Sex,
    pub fertility: f32,
    pub lover: Option<i64>,

    pub epidemic: EpidemicDetails,

    pub seed: f32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone)]
pub struct Sim {
    pub population: i64,
    pub people: Vec<Person>,

    pub months_to_sim: i32,
    pub sim_running: bool,
    pub lover_fix: bool,
    pub start_months: i32,
    pub start_settings_set: bool,
    pub start_people_created: bool,
    pub start_pairs_of_people: i32,
}

pub struct SimStats {
    pub graph_data: Vec<[f64; 2]>,

    pub people_born: i32,
    pub people_dead: i32,
    pub average_lifespan: i32,
    pub amount_of_lovers_total: i32,
    pub average_fertility: i32,
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
            lover: None,

            epidemic: EpidemicDetails {
                infected: false,
                cured: false,
                people_infected: 0,
            },

            // Seed is for random values which will stay consistent
            seed: rand::thread_rng().gen_range(0.0..100.0),
        };

        temp_person
    }

    pub fn update_sim(&mut self, epidemic: &mut Epidemic, sim_stats: &mut SimStats) {
        // Stat check vairables
        let mut born = 0;
        let mut dead = 0;

        // Main sim loop (1 month of calculations)
        for id in 0..self.people.len() {
            if self.people[id].age != None {
                // Ages all people by 1 month
                self.people[id].age = Some(self.people[id].age.unwrap_or(0) + 1);

                // println!("{:?}", people_temp);

                // Chooses people what will have babies
                if self.people[id].lover == None && self.people[id].age > Some(12 * 12) {
                    // Creates a random number to chose a lover for person
                    let lover = rand::thread_rng().gen_range(0..self.people.len());

                    // println!("{}", lover);

                    // If the person is not the lover and if the person does not have a lover one is given
                    if lover != id
                        && self.people[lover].lover == None
                        && self.people[id].sex != self.people[lover].sex
                        && rand::thread_rng().gen_range(0.0..100.0) >= 95.0
                    {
                        self.people[id].lover = Some(self.people[lover].id);
                        self.people[lover].lover = Some(self.people[id].id);
                    }
                }

                // Changes id to -1 for people who will be killed/removed from vec
                let ages = [2, 5, 10, 25, 35, 45, 60, 70, 80, 90];
                let weights = [5, 5, 25, 55, 75, 105, 135, 1050, 350, 150];
                let dist = WeightedIndex::new(&weights).unwrap();

                if self.people[id].age > Some(ages[dist.sample(&mut thread_rng())] * 12)
                    && rand::thread_rng().gen_range(0.0..1.0) > 0.98
                    || (self.people[id].epidemic.infected
                        && self.people[id].age > Some(ages[dist.sample(&mut thread_rng())] * 12)
                        && rand::thread_rng().gen_range(0.0..1.0) > 0.98)
                {
                    // Handles death of a person
                    self.people[id].age = None;

                    dead += 1;
                }

                // println!("{}", self.people.len());

                // Creating babies
                if self.people[id].age > Some(12 * 12)
                    && !(self.people[id].epidemic.infected
                        && (epidemic.lethality > 1.0 || rand::thread_rng().gen_bool(0.0)))
                    && self.people[id].lover != None
                {
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
                        born += 1;
                    }
                }
            }
            self.update_fertility(id);
        }

        if epidemic.progress_epidemic && epidemic.stats.number_of_infected == 0 {
            epidemic.start_epidemic(self, epidemic.start_vals);
        }
        if epidemic.progress_epidemic {
            epidemic.update_epidemic(self);
        }
        if epidemic.stats.number_of_infected == 0 {
            epidemic.end_epidemic();
        }

        epidemic.update_cure(self);
        epidemic.check_end_cure();

        if self.months_to_sim % 100 == 0 {
            self.fix_lovers();
        }

        self.people.retain(|person| person.age.is_some());

        sim_stats.people_born += born;
        sim_stats.people_dead += dead;
    }

    pub fn update_fertility(&mut self, id: usize) {
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

    pub fn fix_lovers(&mut self) {
        for id in 0..self.people.len() {
            if self.lover_fix && self.people[id].age != None {
                // Set the lover as None in person.lover if they are dead
                // THIS IS A VERY INEFFICIENT CHECK
                if Some(self.people[id].age.unwrap() * 12) > Some(12 * 12) {
                    for person in self.people.clone().into_iter() {
                        if self.people[id].lover.is_some()
                            && Some(person.id) == self.people[id].lover
                        {
                            if Some(person.id).is_none() {
                                self.people[id].lover = None;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn generate_name(&mut self, sex: &Sex) -> Option<String> {
        if sex == &Sex::Male {
            let name_f: BufReader<File> = BufReader::new(
                File::open("names/male_names.txt")
                    .unwrap_or_else(|_e| panic!("male_names.txt not found!")),
            );
            name_f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut rand::thread_rng())
        } else {
            let name_f: BufReader<File> = BufReader::new(
                File::open("names/female_names.txt")
                    .unwrap_or_else(|_e| panic!("female_names.txt not found!")),
            );
            name_f
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut rand::thread_rng())
        }
    }
}

// Epidemic code ----------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Epidemic {
    pub progress_epidemic: bool,
    pub progress_cure: bool,

    pub cure_remaining_time: f32,
    pub cure_produced: bool,

    pub r_number: i8,
    pub infectivity: f32,
    pub lethality: f32,

    pub stats: EpidemicPersonStats,
    pub start_vals: EpidemicStartVals,
}

impl Default for Epidemic {
    fn default() -> Self {
        Epidemic {
            progress_epidemic: false,
            progress_cure: false,

            cure_remaining_time: 100.0,
            cure_produced: false,

            r_number: 0,
            infectivity: 0.0,
            lethality: 0.0,

            stats: EpidemicPersonStats {
                sim_state: EpidemicSimState::NoEpidemic,
                graph_data: vec![],
                number_of_infected: 0,
                number_of_cured: 0,
            },
            start_vals: EpidemicStartVals {
                num_of_people_to_infect: 0,
                r_number: 0,
                infectivity: 0.0,
                lethality: 0.0,
            },
        }
    }
}

impl std::fmt::Display for Epidemic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(
            f,
            "{:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}\n {:?}",
            ("Sim state: ", self.stats.sim_state),
            ("Number of infected: ", self.stats.number_of_infected),
            ("Number of cured: ", self.stats.number_of_cured),
            ("Progress epidemic: ", self.progress_epidemic),
            ("Progress cure: ", self.progress_cure),
            ("Cure remaining time: ", self.cure_remaining_time),
            ("Cure produced: ", self.cure_produced),
            ("R number: ", self.r_number),
            ("Infectivity: ", self.infectivity),
            ("Lethality: ", self.lethality)
        );
    }
}

#[derive(Clone, Debug)]
pub struct EpidemicPersonStats {
    pub sim_state: EpidemicSimState,
    pub graph_data: Vec<[f64; 2]>,
    pub number_of_infected: usize,
    pub number_of_cured: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct EpidemicStartVals {
    pub num_of_people_to_infect: i16,
    pub r_number: i8,
    pub infectivity: f32,
    pub lethality: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum EpidemicSimState {
    Infected,
    NoEpidemic,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EpidemicDetails {
    pub infected: bool,
    pub cured: bool,
    pub people_infected: i16,
}

impl Epidemic {
    pub fn update_epidemic(&mut self, sim: &mut Sim) {
        let sim_mut = sim.clone();
        let mut rng = rand::thread_rng();

        for id in 0..sim.people.len() {
            // Main loop which will infect people who are not cured
            if sim.people[id].epidemic.infected
                && self.progress_epidemic
                && self.stats.number_of_infected != 0
            {
                if sim.people[id].epidemic.people_infected
                    < (self.r_number + rng.gen_range(0..=self.r_number)).into()
                    && rng.gen_bool((self.infectivity).into())
                {
                    let person_to_infect = rng.gen_range(0..sim_mut.people.len());

                    if !sim.people[person_to_infect].epidemic.cured {
                        sim.people[person_to_infect].epidemic.infected = true;
                        sim.people[id].epidemic.people_infected += 1;
                    }
                }
            }

            if sim.people[id].epidemic.cured {
                sim.people[id].epidemic.infected = false;
            }
        }

        self.stats.number_of_infected = sim
            .people
            .iter()
            .filter(|&person| person.epidemic.infected)
            .count();
    }

    pub fn start_epidemic(&mut self, sim: &mut Sim, mut start_params: EpidemicStartVals) {
        let mut rng = rand::thread_rng();

        let mut people_to_infect = start_params.num_of_people_to_infect;
        start_params.infectivity /= 1000.0;
        start_params.lethality /= 20.0;

        for id in 0..sim.people.len() {
            if !sim.people[id].epidemic.infected {
                // Initial epidemic start
                if people_to_infect != 0
                    && self.progress_epidemic
                    && self.stats.number_of_infected == 0
                {
                    let person_to_infect = rng.gen_range(0..sim.people.len());

                    if !sim.people[person_to_infect].epidemic.infected {
                        sim.people[person_to_infect].epidemic.infected = true;
                    }

                    sim.people[person_to_infect].epidemic.people_infected += 1;

                    self.r_number = start_params.r_number;
                    self.infectivity = start_params.infectivity;
                    self.lethality = start_params.lethality;

                    self.stats.sim_state = EpidemicSimState::Infected;
                    people_to_infect -= 1;
                }
            }
        }
    }

    pub fn end_epidemic(&mut self) -> bool {
        if !self.progress_epidemic {
            self.r_number = 0;
            self.infectivity = 0.0;
        }

        // Stops epidemic if nobody is infected
        if self.stats.number_of_infected == 0 {
            self.progress_epidemic = false;
            self.progress_cure = false;

            self.stats.sim_state = EpidemicSimState::NoEpidemic;
            return false;
        } else {
            return true;
        }
    }

    pub fn update_cure(&mut self, sim: &mut Sim) {
        let mut rng = rand::thread_rng();

        if self.progress_cure {
            if self.cure_remaining_time <= 0.0 {
                self.cure_produced = true;
            } else {
                self.cure_remaining_time -= 1.0;
            }

            if self.cure_produced {
                let cures_in_a_month = 5;
                
                for _ in 0..=cures_in_a_month {
                    let person_to_cure = rng.gen_range(0..sim.people.len());

                    if !sim.people[person_to_cure].epidemic.cured {
                        sim.people[person_to_cure].epidemic.infected = false;
                        sim.people[person_to_cure].epidemic.cured = true;
                    }
                }
                
            }
        }

        self.stats.number_of_cured = sim
            .people
            .iter()
            .filter(|&person| person.epidemic.cured)
            .count();
    }

    pub fn check_end_cure(&mut self) {}
}
