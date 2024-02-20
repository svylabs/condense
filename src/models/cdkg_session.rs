use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use diesel::sql_types::Timestamp;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ckg_sessions)]
pub struct CDKGSession {
    pub id: i32,
    pub initiated_by: i32,
    pub threshold: i32,
    pub total_participants: i32,
    pub current_state: String,
    pub ckg_session_timeout: i32,
    pub generated_public_key: Option<Vec<u8>>,
    pub created_on: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl CDKGSession {
    pub fn list_sessions(state: String, conn: &mut PgConnection) -> Result<Vec<CDKGSession>, diesel::result::Error> {
        use crate::schema::ckg_sessions::dsl::*;
        ckg_sessions.filter(current_state.eq(state))
            .load(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::ckg_sessions)]
pub struct NewCDKGSession {
    pub initiated_by: i32,
    pub threshold: i32,
    pub total_participants: i32,
    pub current_state: String,
    pub ckg_session_timeout: i32,
    pub created_on: NaiveDateTime
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
    pub updated_at: diesel::sql_types::Timestamp,
    pub created_on: diesel::sql_types::Timestamp,
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