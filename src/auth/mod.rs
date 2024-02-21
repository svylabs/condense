use actix_web::{Error, FromRequest, HttpRequest, Result, dev::Payload};
use futures::future::{Ready};
use std::{pin::Pin, future::Future};


#[derive(Debug, Clone)]
pub struct AuthedUser {
    pub participant_id: i32,
}


impl FromRequest for AuthedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthedUser, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let participant_id = 2;
        println!("In Auth: participant_id: {:?}", participant_id );
        Box::pin(async move {
            Ok(AuthedUser { participant_id })
        })
    }
}

/*
// Define a middleware to perform authentication
pub async fn auth_middleware(req: HttpRequest) -> Result<HttpRequest, actix_web::Error> {
    // Perform authentication logic here (e.g., check authentication token)
    let user = AuthedUser {
       participant_id: 2,
    };
    println!("Auth middleware: participant_id: {:?}", 2);
    // Set the authenticated user information in the request extensions
    req.extensions_mut().insert(user);

    // Continue processing the request
    Ok(req)
}*/