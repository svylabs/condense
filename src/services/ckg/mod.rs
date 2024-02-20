pub mod types;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{schema::{ckg_sessions, ckg_session_participant_values}, Pool};
use crate::models::{roles::Role, user_roles::UserRole, users::User, cdkg_session::{CDKGSession, NewCDKGSession, NewCDKGSessionParticipant, CDKGSessionParticipant}};
use chrono::prelude::{Utc};
use crate::services::ckg::types::{ListSessionsInput, InitParticipantInput, GetSessionResult, PostRoundInput};

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
                current_state: "".to_string()
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
async fn init_participant(body: web::Json<InitParticipantInput>, db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let input = body.into_inner();
    let participant_id = 2; // change this
    let result = conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        let participant_session = CDKGSessionParticipant::find_by_session_and_participant(input.session_id, participant_id, conn).unwrap();
        diesel::update(ckg_session_participant_values::table)
            .filter(ckg_session_participant_values::id.eq(participant_session.id))
            .set((ckg_session_participant_values::current_state.eq("Initiated"),
                 (ckg_session_participant_values::session_public_key.eq(input.public_key))
                )
            )
            .execute(conn)
    });
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/session/{id}")]
async fn get_session(path: web::Path<i32>, db: web::Data<Pool>) -> impl Responder {
    // Get the session details
    let mut conn = db.get().unwrap();
    let id = path.into_inner();
    let result = conn.transaction::<GetSessionResult, diesel::result::Error, _>(|conn| {
        let session_details = CDKGSession::find_by_id(id, conn).unwrap();
        let participant_details = CDKGSessionParticipant::find_by_session(session_details.id, conn).unwrap();
        Ok(GetSessionResult { session_details, participant_details})
    }).unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/round1")]
async fn round1(body: web::Json<PostRoundInput>, db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let input = body.into_inner();
    let participant_id = 2; // change this
    let result = conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        let participant_session = CDKGSessionParticipant::find_by_session_and_participant(input.session_id, participant_id, conn).unwrap();
        diesel::update(ckg_session_participant_values::table)
            .filter(ckg_session_participant_values::id.eq(participant_session.id))
            .set((ckg_session_participant_values::round1_data.eq(input.round_data), ckg_session_participant_values::current_state.eq("Round1")))
            .execute(conn)
    });
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/round2")]
async fn round2(body: web::Json<PostRoundInput>, db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let input = body.into_inner();
    let participant_id = 2; // change this
    let result = conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        let participant_session = CDKGSessionParticipant::find_by_session_and_participant(input.session_id, participant_id, conn).unwrap();
        diesel::update(ckg_session_participant_values::table)
            .filter(ckg_session_participant_values::id.eq(participant_session.id))
            .set((ckg_session_participant_values::round2_data.eq(input.round_data), ckg_session_participant_values::current_state.eq("Round2")))
            .execute(conn)
    });
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/round3")]
async fn round3(body: web::Json<PostRoundInput>, db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let input = body.into_inner();
    let participant_id = 2; // change this
    let result = conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        let participant_session = CDKGSessionParticipant::find_by_session_and_participant(input.session_id, participant_id, conn).unwrap();
        diesel::update(ckg_session_participant_values::table)
            .filter(ckg_session_participant_values::id.eq(participant_session.id))
            .set((ckg_session_participant_values::round3_data.eq(input.round_data), ckg_session_participant_values::current_state.eq("Round3")))
            .execute(conn)
    });
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ckg")
            .service(new_dkg_session)
            .service(list_sessions)
            .service(init_participant)
            .service(get_session)
            .service(round1)
            .service(round2)
            .service(round3)
    );
}