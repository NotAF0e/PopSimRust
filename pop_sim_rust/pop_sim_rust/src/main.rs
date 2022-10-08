use std::str;
use gregorian::{Date};

// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i32,
    name: &'static str,
    age: f32,
    gender: i16,
}

fn main() {
    let date: Date = Date::new(0000, 1, 1).unwrap();
    static mut POPULATION: i32 = -1;
    static mut PEOPLE: Vec<Person> = Vec::new();

    pub unsafe fn create_person() -> Person {
        POPULATION += 1;
        let temp_person: Person = Person { name: "John", age: 0.0, gender: 0, id: POPULATION };

        return temp_person;
    }
    let add_day = ||{
        date.add_days(1);
        println!("{:?}", date)
    };

    pub fn update_sim() {
        let people_temp = unsafe { &mut PEOPLE };
        for id in 0..2 {
            people_temp[id].age += 1.0;
        }
    }

    pub fn print_people() {
        for id in 0..unsafe { PEOPLE.len() } {
            unsafe {
                println!("[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n", PEOPLE[id].id, PEOPLE[id].name, (PEOPLE[id].age / 12.0),
                         PEOPLE[0].gender)
            }
        }
    }
    let people_vec = unsafe { &mut PEOPLE };

    let john: Person = unsafe { create_person() };
    people_vec.push(john);

    let john2: Person = unsafe { create_person() };
    people_vec.push(john2);

    // Graphing variables
    // let mp: Vec<i32> = Vec::new();
    // let pop: Vec<i32> = Vec::new();
    // let tp: i32 = -1;

    print_people();
    add_day();
    update_sim();
    print_people();
}
