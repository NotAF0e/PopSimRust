use std::str;

// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i32,
    name: &'static str,
    age: i32,
    gender: i8,
}

fn main() {
    static mut POPULATION: i32 = -1;
    let mut people: Vec<Person> = Vec::new();

    pub unsafe fn create_person() -> Person {
        POPULATION += 1;
        let temp_person: Person = Person { name: "John", age: 0, gender: 0, id: POPULATION };

        return temp_person;
    }

    let john: Person = unsafe { create_person() };
    people.push(john);

    let john2: Person = unsafe { create_person() };
    people.push(john2);

    // Graphing variables
    // let mp: Vec<i32> = Vec::new();
    // let pop: Vec<i32> = Vec::new();
    // let tp: i32 = -1;
    println!("{:?}", people);

    for id in 0..2 {
        println!("[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n", people[id].id, people[id].name, people[id].age,
                 people[0].gender)
    }
}
