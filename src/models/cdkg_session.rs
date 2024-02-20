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

    pub fn find_by_id(session_id: i32, conn: &mut PgConnection) -> Result<CDKGSession, diesel::result::Error> {
        use crate::schema::ckg_sessions::dsl::*;
        ckg_sessions.filter(id.eq(session_id))
            .first(conn)
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


#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ckg_session_participant_values)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CDKGSessionParticipant {
    pub id: i32,
    pub ckg_session_id: i32,
    pub participant_id: i32,
    pub current_state: Option<String>,
    pub session_public_key: Option<Vec<u8>>,
    pub round1_data: Option<Vec<u8>>,
    pub round2_data: Option<Vec<u8>>,
    pub round3_data: Option<Vec<u8>>,
    pub created_on: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl CDKGSessionParticipant {
    pub fn find_by_session_and_participant(session_id: i32, participant: i32, conn: &mut PgConnection) -> Result<CDKGSessionParticipant, diesel::result::Error> {
        use crate::schema::ckg_session_participant_values::dsl::*;
        ckg_session_participant_values.filter(ckg_session_id.eq(session_id))
            .filter(participant_id.eq(participant))
            .first(conn)
            .map_err(|e| e.into())
    }

    pub fn find_by_session(session_id: i32, conn: &mut PgConnection) -> Result<Vec<CDKGSessionParticipant>, diesel::result::Error> {
        use crate::schema::ckg_session_participant_values::dsl::*;
        ckg_session_participant_values.filter(ckg_session_id.eq(session_id))
            .load(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::ckg_session_participant_values)]
pub struct NewCDKGSessionParticipant {
    pub ckg_session_id: i32,
    pub participant_id: i32,
    pub current_state: String
}

pub struct CDKGSessionParticipantInitialization {
    pub ckg_session_id: i32,
    pub participant_id: i32,
    pub current_state: String,
    pub session_public_keys: Option<Vec<u8>>
}