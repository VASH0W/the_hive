use chrono::prelude::*
use std::collections::HashMap;
use serde_json::{Result, Value};
pub enum HiveObject {
    User(User),
    Community(Community),
    Debate(Debate),
    Thought(Thought),
}

pub enum HiveObjectsError {
    CreationError,
    UpdateError,
    DeletionError,
}

pub struct User {
    id_number: String,
    email: String,
    hashed_password: String,
    object_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// This function acts as a constructor to a `User` object taking and consuming a string
    /// containing its component parameters
    /// `parameters` format is a json Example
    /// "{'id_number': '123456789', 'hashed_password': 'password_hashed', 'email': 'random@hotmail.com'}"
    fn new(json_parameters: String) -> Result<Self, HiveObjectsError> {
        let mut parameters_dictionary: HashMap<String, Value> = serde_json::from_str(&json_parameters[..]).unwrap();
        let id_number: String = match parameters_dictionary.get("id_number") {
            Some(i) => i,
            None => return HiveObjectsError::CreationError,
        };
        

    }

}