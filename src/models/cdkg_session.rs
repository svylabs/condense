use diesel::prelude::*;
use serde::{Serialize, Deserialize};

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