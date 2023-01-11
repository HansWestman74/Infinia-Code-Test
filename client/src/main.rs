const URL_INFINIA: &str = "http://localhost:8080/infinia";
use reqwest;

mod models;
use models::{create_people, People};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    match people_test(&client).await {
        Ok(people) => {
            println!("{:?}", people)
        }
        Err(e) => println!("{:?}", e),
    };

    Ok(())
}

async fn people_test(client: &reqwest::Client) -> Result<People, Box<dyn std::error::Error>> {
    let people = create_people();

    let people = client
        .post(URL_INFINIA.to_string() + "/people")
        .json(&people)
        .send()
        .await?
        .json::<People>()
        .await?;
    Ok(people)
}
