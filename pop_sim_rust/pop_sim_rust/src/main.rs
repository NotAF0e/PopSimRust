use std::str;
use rand::Rng;

// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i32,
    name: &'static str,
    gender: i16,
    age: i32,
    love_vec: Vec<i32>,
}

fn main() {
    static mut POPULATION: i32 = -1;
    static mut PEOPLE: Vec<Person> = Vec::new();

    pub unsafe fn create_person() -> Person {
        POPULATION += 1;
        let temp_person: Person = Person {
            id: POPULATION,
            name: "John",
            gender: 0,
            age: 0,
            love_vec: vec![-1, 100],
        };

        return temp_person;
    }

    pub fn update_sim() {
        let people_temp = unsafe { &mut PEOPLE };
        for id in 0..unsafe { PEOPLE.len() } {
            // Ages all people by 1 day
            people_temp[id].age += 1;

            // Creates a random number to chose a lover for person
            let lover = rand::thread_rng().gen_range(0..=(unsafe { PEOPLE.len() } - 1)) as i32;

            // If the person is not the lover and if the person does not have a lover one is given
            if lover != id as i32 && people_temp[id].love_vec[0] == -1 {
                println!("{} from {}", &lover, &id);
                people_temp[id].love_vec[0] = lover;
            }
        }
    }

    pub fn print_people() {
        for id in 0..unsafe { PEOPLE.len() } {
            println!("-------------------------------------------");
            unsafe {
                println!("[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n\
                  Lover: {:?}", PEOPLE[id].id, PEOPLE[id].name, PEOPLE[id].age,
                         PEOPLE[id].gender, PEOPLE[id].love_vec)
            }
        }
    }
    let people_temp = unsafe { &mut PEOPLE };

    let john: Person = unsafe { create_person() };
    people_temp.push(john);

    let john2: Person = unsafe { create_person() };
    people_temp.push(john2);

    // Graphing variables
    // let mp: Vec<i32> = Vec::new();
    // let pop: Vec<i32> = Vec::new();
    // let tp: i32 = -1;

    print_people();
    update_sim();
    update_sim();
    print_people();
}
