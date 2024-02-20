use serde::{Serialize, Deserialize};

use crate::models::roles::NewRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRolesInput {
    pub roles: Vec<NewRole>
}