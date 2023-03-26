use chrono::prelude::*;
use std::collections::HashMap;
use serde_json::{Result, Value};
use uuid::Uuid;
pub trait HiveObject {
    fn new(json_parameters: String) -> Self;
    fn get_id(&self) -> &str;
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

impl HiveObject for User {
    /// This function acts as a constructor to a `User` object taking and consuming a string
    /// containing its component parameters
    /// `parameters` format is a json Example
    /// "{'id_number': '123456789', 'hashed_password': 'password_hashed', 'email': 'random@hotmail.com'}"
    fn new(json_parameters: String) -> Result<User, HiveObjectsError> {
        let mut parameters_dictionary: HashMap<String, Value> = serde_json::from_str(&json_parameters[..]).unwrap();
        let id_number = match parameters_dictionary.get("id_number") {
            Some(i) => i.to_string(),
            None => return Err(HiveObjectsError::CreationError),
        };
        let email = match parameters_dictionary.get("email") {
            Some(i) => i.to_string(),
            None => return Err(HiveObjectsError::CreationError),
        };
        let hashed_password = match parameters_dictionary.get("hashed_password") {
            Some(i) => i.to_string(),
            None => return Err(HiveObjectsError::CreationError),
        };
        let object_id = Uuid::new_v4().to_string();
        let created_at: DateTime<Utc> = Utc::now();
        let updated_at: DateTime<Utc> = Utc::now();

        let new_user = User {
            id_number: id_number,
            email: email,
            hashed_password: hashed_password,
            object_id: object_id,
            created_at: created_at,
            updated_at: created_at,

        };
        Ok(new_user)
    }
    fn get_id(&self) -> &str{
        &self.object_id
    }

}


#[cfg(test)]
mod hive_objects_tests {
    use super::*;

    #[test]
    fn create_new_user() {
        let json_parameters = "{'id_number': '123456789', 'hashed_password': 'password_hashed', 'email': 'random@hotmail.com'}".to_string();
        let new_user: User = match  User::new(json_parameters) {
            Ok(i) => i,
            Err(e) => return,
        };
        assert_eq!(new_user.id_number, String::from("123456789"));
        assert_eq!(new_user.email, String::from("random@hotmail.com"));
        assert_eq!(new_user.hashed_password, String::from("password_hashed"));
    }

}
