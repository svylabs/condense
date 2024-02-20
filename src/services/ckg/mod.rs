pub mod types;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{schema::{ckg_sessions, ckg_session_participant_values}, Pool};
use crate::models::{roles::Role, user_roles::UserRole, users::User, cdkg_session::{CDKGSession, NewCDKGSession, NewCDKGSessionParticipant}};
use chrono::prelude::{Utc};
use crate::services::ckg::types::ListSessionsInput;

pub enum ParticipantState {
    Initiated,
    Round1,
    Round2,
    Round3,
    Completed
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

#[post("/new-session")]
async fn new_dkg_session(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let result = conn.transaction::<CDKGSession, diesel::result::Error, _>(|conn| {
        let signer_role = Role::find_by_name("signer", conn).unwrap();
        let signer_users = UserRole::find_users_by_role_id(signer_role.id, conn).unwrap();
        let signers = User::find_users_by_ids(signer_users.iter().map(|user_role| user_role.user_id).collect::<Vec<i32>>(), conn).unwrap();
        let new_session = NewCDKGSession {
            initiated_by: 2, // change this
            threshold: ((signers.len() as i32) / 2 + 1),
            total_participants: signers.len() as i32,
            current_state: "Requested".to_string(),
            ckg_session_timeout: 0,
            created_on: Utc::now().naive_utc()
        };
        let inserted_session = diesel::insert_into(ckg_sessions::table)
            .values(&new_session)
            .get_result::<CDKGSession>(conn)
            .unwrap();
        let participants = signers.iter().map(|signer| {
            NewCDKGSessionParticipant {
                ckg_session_id: inserted_session.id,
                participant_id: signer.id,
                current_state: "".to_string(),
                session_public_keys: None,
                round1_data: None,
                round2_data: None,
                round3_data: None
            }
        }).collect::<Vec<NewCDKGSessionParticipant>>();
        let inserted_participants = diesel::insert_into(ckg_session_participant_values::table)
            .values(&participants)
            .execute(conn)
            .unwrap();
        Ok(inserted_session)
    });
    match result {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/list-sessions")]
async fn list_sessions(body: web::Json<ListSessionsInput>, db: web::Data<Pool>) -> impl Responder {
    // List sessions based on filter
    let mut conn = db.get().unwrap();
    let input = body.into_inner();
    println!("Inputs {:?}", input);
    let session_status = match input.session_status {
        Some(status) => status,
        None => "Requested".to_string()
    };
    let result = CDKGSession::list_sessions(session_status, &mut conn);
    match result {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
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
            .service(new_dkg_session)
            .service(list_sessions)
            .service(round2)
            .service(round3)
    );
}