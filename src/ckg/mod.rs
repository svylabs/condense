use actix_web::{get, post, web, HttpResponse, Responder};
use crate::{schema::{roles, user_roles, users}, Pool};
use crate::models::{roles::Role, user_roles::UserRole, users::User, cdkg_session::NewCDKGSession};
use diesel::prelude::*;

pub enum ParticipantState {
    Initiated,
    Round1,
    Round2,
    Round3,
    Completed
}

pub struct Participant {
    pub id: u64,
    pub state: ParticipantState,
    pub communication_key: Option<Vec<u8>>, // Ed25519 public key that will be used to encrypt data using ECDH
    pub round1_data: Option<Vec<Vec<u8>>>,
    pub round2_data: Option<Vec<Vec<u8>>>,
    pub round3_data: Option<Vec<Vec<u8>>>
}

pub enum CDKGRequestState {
    Requested,
    Initiated,
    Round1,
    Round2,
    Round3,
    Completed,
    Error,
    Timedout
}

/*
pub struct CDKGSession {
    pub id: u64,
    pub participants: Vec<Participant>,
    pub threshold: u16,
    pub current_state: CDKGRequestState,
    pub start_time: u64,
    pub last_updated: u64,
    pub timeout: u64,
    pub key: Option<Vec<u8>>,
}
*/

#[post("/new-session")]
async fn new_dkg_session(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let signer_role = roles::table
        .filter(roles::name.eq("signer"))
        .first::<Role>(&mut conn)
        .expect("Error loading roles");
    let signer_users = user_roles::table
        .filter(user_roles::role_id.eq(signer_role.id))
        .load::<UserRole>(&mut conn)
        .expect("Error loading users");
    let signers = users::table
        .filter(users::id.eq_any(signer_users.iter().map(|user_role| user_role.user_id).collect::<Vec<i32>>()))
        .load::<User>(&mut conn)
        .expect("Error loading users");

    let new_session = NewCDKGSession {
        initiated_by: 0,
        threshold: 0,
        total_participants: 0,
        current_state: "Requested".to_string(),
        ckg_session_timeout: 0,
        generated_public_key: "".to_string()
    };
    // Create a new CDKG session
    /*
    state.cdkg_sessions.lock().unwrap().push(CDKGSession {
        id: 0,
        participants: Vec::new(),
        threshold: 0,
        current_state: CDKGRequestState::Requested,
        start_time: 0,
        last_updated: 0,
        timeout: 0,
        key: None
    });
    */
    // Set the participants, threshold, and current state
    // Return the session ID
    HttpResponse::Ok().body("Round 1")
}

#[get("/list-sessions")]
async fn list_sessions() -> impl Responder {
    // List sessions based on filter
    HttpResponse::Ok().body("Round 1")
}

#[post("/init-participant")]
async fn init_participant() -> impl Responder {
    // Initialize the participant for the dkg session
    HttpResponse::Ok().body("Round 1")
}

#[get("/session/:id")]
async fn get_session() -> impl Responder {
    // Get the session details
    HttpResponse::Ok().body("Round 1")
}

#[post("/round1")]
async fn round1() -> impl Responder {
    // Post round 1 data
    HttpResponse::Ok().body("Round 1")
}

#[post("/round2")]
async fn round2() -> impl Responder {
    //Post round 2 data
    HttpResponse::Ok().body("Round 2")
}

#[get("/round3")]
async fn round3() -> impl Responder {
    // Post round 3 data
    HttpResponse::Ok().body("Round 3")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ckg")
            .service(round1)
            .service(round2)
            .service(round3)
    );
}