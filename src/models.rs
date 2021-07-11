use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct PeopleResponse {
    pub data: Vec<Person>    
}