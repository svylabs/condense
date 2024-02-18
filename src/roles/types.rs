use serde::{Serialize, Deserialize};

use crate::models::NewRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRolesInput {
    pub roles: Vec<NewRole>
}