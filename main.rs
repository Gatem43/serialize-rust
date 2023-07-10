use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    // Сериализация в JSON
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Сериализованные данные в JSON: {}", serialized);

    // Сохранение сериализованных данных в файл
    let mut file = File::create("person.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();

    // Чтение сериализованных данных из файла
    let mut file = File::open("person.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Десериализация из JSON
    let deserialized: Person = serde_json::from_str(&contents).unwrap();
    println!("Десериализованные данные: {:?}", deserialized);
}
