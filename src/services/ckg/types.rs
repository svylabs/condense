use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListSessionsInput {
    pub session_status: Option<String>
}

pub struct InitParticipantInput {
    pub session_id: i32,
    pub participant_id: i32,
    pub participant_public_key: Vec<u8>
}