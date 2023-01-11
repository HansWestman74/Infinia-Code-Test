use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub height: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct People {
    pub people: Vec<Person>,
}
