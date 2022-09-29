use std::str;

#[derive(Debug)]
pub struct Person{
    name: &'static str,
    age: i32,
    gender: i8
}


fn create_person() -> Person {
    let temp_person = Person{name: "John", age: 0, gender: 0};

    return temp_person
}

fn main() {
    let mut people: Vec<Person> = Vec::new();
    let john: Person = create_person();

    people.push(john);

    // Graphing variables
    // let mp: Vec<i32> = Vec::new();
    // let pop: Vec<i32> = Vec::new();
    // let tp: i32 = -1;


    println!("Name: {:?}\n\
             Age: {:?}\n\
             Gender: {:?}\n", people[0].name, people[0].age, people[0].gender)

}
