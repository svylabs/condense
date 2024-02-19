use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::{roles::Role};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32
}

impl NewUserRole {
    pub fn new(user_id: i32, roles: &Vec<Role>) -> Vec<NewUserRole> {
        roles.into_iter().map(|role| {
            NewUserRole {
                user_id,
                role_id: role.id
            }
        }).collect::<Vec<NewUserRole>>()
    }
}