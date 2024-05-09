use diesel::prelude::*;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use crate::schema::contracts;


#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = contracts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contract {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub host_id: String,
    pub guest_id: String,
    pub office_id: String,
    pub num_posts: i32,
    pub monthly_amount: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl Contract {
    pub fn new(host_id: String, guest_id: String, office_id: String, num_posts: i32, monthly_amount: i32, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        Contract {
            id: format!("agr-{}", Uuid::new_v4().to_string()),
            created_at: Utc::now(),
            host_id,
            guest_id,
            office_id,
            num_posts,
            monthly_amount,
            start_date,
            end_date,
        }
    }
}