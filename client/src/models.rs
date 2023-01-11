use serde::{Deserialize, Serialize};

/// Basic data structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub height: u32,
}

/// Wrapper struct for the people data
#[derive(Deserialize, Serialize, Debug)]
pub struct People {
    pub people: Vec<Person>,
}

/// Helper function to create data
pub fn create_people() -> People {
    let mut people = vec![
        Person {
            name: "Hans".to_string(),
            height: 189,
        },
        Person {
            name: "Andrej".to_string(),
            height: 186,
        },
    ];
    People { people }
}
