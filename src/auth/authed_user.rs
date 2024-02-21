
use actix_web::{Error, FromRequest, HttpRequest, Result, dev::Payload};
use futures::future::{Ready};
pub struct AuthedUser {
    pub participant_id: i32,
}

impl FromRequest for AuthedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let participant_id = 2;
        println!("participant_id: {:?}", participant_id );
        Ok(AuthedUser { participant_id })
    }
}