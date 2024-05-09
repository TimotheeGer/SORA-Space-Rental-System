use diesel::prelude::*;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use std::fmt;
use rand::Rng;


use crate::schema::{offices, host_relations, guest_relations};

#[derive(Debug)]
#[allow(dead_code)]
pub enum OfficeError {
    InvalidWorkstations,
    InvalidPrice,
    InvalidSurface,
    Custom(String),

    // Ajoutez ici d'autres types d'erreurs personnalisées...
}

impl fmt::Display for OfficeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OfficeError::InvalidWorkstations => write!(f, "The number of workstations must be between 40 and 180"),
            OfficeError::InvalidPrice => write!(f, "The price per post per month must be between 300 and 800 euros"),
            OfficeError::InvalidSurface => write!(f, "The surface per post must be 8m2 for less than 60 posts and 7m2 for more than 60 posts"),
            OfficeError::Custom(ref msg) => write!(f, "{}", msg),
            // Implémentez ici d'autres types d'erreurs personnalisées...
        }
    }
}

impl std::error::Error for OfficeError {}


#[derive(Debug, Clone)]
pub struct NewOffice {
    pub name: String,
    pub address_text: String,
    pub surface: f64,
    pub num_posts: Option<i32>,
    pub price_per_post: Option<i32>,
    pub parent_office_id: Option<String>,
    pub owner_id: String,
}

#[derive(Queryable, Insertable, Selectable, AsChangeset, Debug, Clone)]
#[diesel(table_name = offices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Office {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub address_text: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub surface: f64,
    pub num_posts: Option<i32>,
    pub price_per_post: Option<i32>,
    pub parent_office_id: Option<String>,
    pub owner_id: String,
}

impl Office {
    pub fn new(name: String, address_text: String, surface: f64, num_posts: Option<i32>, price_per_post: Option<i32>, parent_office_id: Option<String>, owner_id: String, prefix: String) -> Self {
        let mut rng = rand::thread_rng();

        Office {
            id: format!("{}-{}", prefix, Uuid::new_v4().to_string()),
            created_at: Utc::now(),
            name,
            address_text,
            latitude: Some(rng.gen_range(-90.0..90.0)),
            longitude: Some(rng.gen_range(-180.0..180.0)),
            surface,
            num_posts,
            price_per_post,
            parent_office_id,
            owner_id,
        }
    }
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = host_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HostRelation {
    pub created_at: DateTime<Utc>,
    pub user_id: String,
    pub office_id: String,
}

impl HostRelation {
    pub fn new(user_id: String, office_id: String) -> Self {
        HostRelation {
            created_at: Utc::now(),
            user_id,
            office_id,
        }
    }
}

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = guest_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GuestRelation {
    pub created_at: DateTime<Utc>,
    pub user_id: String,
    pub office_id: String,
}

impl GuestRelation {
    pub fn new(user_id: String, office_id: String) -> Self {
        GuestRelation {
            created_at: Utc::now(),
            user_id,
            office_id,
        }
    }
}