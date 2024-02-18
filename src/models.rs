use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub name: String,
    pub description: String,
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

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::ckg_sessions)]
pub struct CDKGSession {
    pub id: i32,
    pub initiated_by: i32,
    pub threshold: i32,
    pub total_participants: i32,
    pub current_state: String,
    pub ckg_session_timeout: i32,
    pub generated_public_key: String,
    pub created_on: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::ckg_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCDKGSession {
    pub initiated_by: i32,
    pub threshold: i32,
    pub total_participants: i32,
    pub current_state: String,
    pub ckg_session_timeout: i32,
    pub generated_public_key: String,
}


#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::ckg_session_participant_values)]
pub struct CDKGSessionParticipant {
    pub id: i32,
    pub ckg_session_id: Option<i32>,
    pub participant_id: Option<i32>,
    pub current_state: String,
    pub session_public_keys: Option<Vec<u8>>,
    pub round1_data: Option<Vec<u8>>,
    pub round2_data: Option<Vec<u8>>,
    pub round3_data: Option<Vec<u8>>,
    pub updated_at: chrono::NaiveDateTime,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::ckg_session_participant_values)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCDKGSessionParticipant {
    pub ckg_session_id: i32,
    pub participant_id: i32,
    pub current_state: String,
    pub session_public_keys: Option<Vec<u8>>,
    pub round1_data: Option<Vec<u8>>,
    pub round2_data: Option<Vec<u8>>,
    pub round3_data: Option<Vec<u8>>,
}