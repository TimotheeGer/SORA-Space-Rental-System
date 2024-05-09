use diesel::prelude::*;
use async_trait::async_trait;

use crate::models::office::Office;
use crate::db::establish_connection;

#[async_trait]
pub trait OfficeRepository: Send+Sync{
    async fn creat(&self, office: Office) -> Result<Office, diesel::result::Error>;
    async fn find_by_owner_id(&self, owner_id: String) -> Result<Vec<Office>, diesel::result::Error>;
    async fn find_all(&self) -> Result<Vec<Office>, diesel::result::Error>;
    async fn find_by_prefix_id(&self, prefix_id: String) -> Result<Vec<Office>, diesel::result::Error>;
    async fn find_by_name(&self, name: String) -> Result<Office, diesel::result::Error>;
    async fn find_by_id(&self, id: String) -> Result<Office, diesel::result::Error>;
    async fn find_by_ids(&self, ids: Vec<String>) -> Result<Vec<Office>, diesel::result::Error>;
}

pub struct OfficeRepositoryImpl;

#[async_trait]
impl OfficeRepository for OfficeRepositoryImpl {
    
    async fn creat(&self, office: Office) -> Result<Office, diesel::result::Error> {
        let mut connection = establish_connection();
        diesel::insert_into(crate::schema::offices::table)
            .values(&office)
            .execute(&mut connection)
            .expect("Error saving new office");
        Ok(office)
    }

    async fn find_by_owner_id(&self, owner_id: String) -> Result<Vec<Office>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::offices::table
            .filter(crate::schema::offices::owner_id.eq(owner_id))
            .load::<Office>(&mut connection)?;
        Ok(result)
    }

    async fn find_all(&self) -> Result<Vec<Office>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::offices::table
            .load::<Office>(&mut connection)?;
        Ok(result)
    }

    async fn find_by_prefix_id(&self, prefix_id: String) -> Result<Vec<Office>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::offices::table
            .filter(crate::schema::offices::id.like(format!("{}%", prefix_id)))
            .load::<Office>(&mut connection)?;
        Ok(result)
    }

    async fn find_by_name(&self, name: String) -> Result<Office, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::offices::table
            .filter(crate::schema::offices::name.eq(name))
            .first::<Office>(&mut connection)?;
        Ok(result)
    }

    async fn find_by_id(&self, id: String) -> Result<Office, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::offices::table
            .filter(crate::schema::offices::id.eq(id))
            .first::<Office>(&mut connection)?;
        Ok(result)
    }

    async fn find_by_ids(&self, ids: Vec<String>) -> Result<Vec<Office>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::offices::table
            .filter(crate::schema::offices::id.eq_any(ids))
            .load::<Office>(&mut connection)?;
        Ok(result)
    }
}