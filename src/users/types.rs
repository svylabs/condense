use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserInput {
    pub username: String,
    pub roles: Option<Vec<String>>
}