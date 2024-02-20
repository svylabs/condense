use serde::{Serialize, Deserialize};
use crate::models::{cdkg_session::{CDKGSession, CDKGSessionParticipant}};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListSessionsInput {
    pub session_status: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitParticipantInput {
    pub session_id: i32,
    pub public_key: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSessionResult {
    pub session_details: CDKGSession,
    pub participant_details: Vec<CDKGSessionParticipant>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRoundInput {
    pub session_id: i32,
    pub round_data: Vec<u8>
}