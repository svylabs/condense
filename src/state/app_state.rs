use crate::signers::Signer;
use crate::ckg::CDKGSession;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub signers: Arc<Mutex<Vec<Signer>>>,
    pub cdkg_sessions: Arc<Mutex<Vec<CDKGSession>>>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            signers: Arc::new(Mutex::new(Vec::new())),
            cdkg_sessions: Arc::new(Mutex::new(Vec::new()))
        }
    }
}