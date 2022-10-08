use std::str;

// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i32,
    name: &'static str,
    age: i32,
    gender: i16,
}

fn main() {
    static mut POPULATION: i32 = -1;
    static mut PEOPLE: Vec<Person> = Vec::new();

    pub unsafe fn create_person() -> Person {
        POPULATION += 1;
        let temp_person: Person = Person { name: "John", age: 0, gender: 0, id: POPULATION };

        return temp_person;
    }

    pub unsafe fn update_sim() {
        let people_temp = &mut PEOPLE;
        for id in 0..2 {
            people_temp[id].age += 1;
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

    for id in 0..2 {
        unsafe {
            println!("[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n", PEOPLE[id].id, PEOPLE[id].name, PEOPLE[id].age,
                     PEOPLE[0].gender)
        }
    }
    unsafe { update_sim() };

    for id in 0..2 {
        unsafe {
            println!("[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n", PEOPLE[id].id, PEOPLE[id].name, PEOPLE[id].age,
                     PEOPLE[0].gender)
        }
    }
}
