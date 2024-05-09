use diesel::prelude::*;
use chrono::DateTime;
use chrono::offset::Utc;

use crate::schema::{host_relations, guest_relations};


#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = host_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HostRelation {
    pub user_id: String,
    pub office_id: String,
    pub created_at: DateTime<Utc>,
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
    pub user_id: String,
    pub office_id: String,
    pub created_at: DateTime<Utc>,
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