use diesel::prelude::*;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use crate::schema::users;
use derive_more::{Display, Error};


#[derive(Debug, Display, Error, Clone, PartialEq)]
pub enum UserRole {
    HOST,
    GUEST
}


impl std::str::FromStr for UserRole {
    type Err = ();
    fn from_str(input: &str) -> Result<UserRole, Self::Err> {
        match input {
            "HOST"  => Ok(UserRole::HOST),
            "GUEST"  => Ok(UserRole::GUEST),
            _ => Err(()),
        }
    }
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    pub last_name: String,
    pub user_role: String,
}

impl User {
    pub fn new(first_name: String, last_name: String, user_role: UserRole) -> Self {
        User {
            id: format!("usr-{}", Uuid::new_v4().to_string()),
            created_at: Utc::now(),
            first_name,
            last_name,
            user_role: user_role.to_string(),
        }
    }
}