use diesel::prelude::*;
use async_trait::async_trait;

use crate::db::establish_connection;

use crate::models::office::OfficeError;
use crate::models::relation::*;


#[async_trait]
pub trait RelationRepository: Send+Sync {
    async fn create_guest(&self, relation: GuestRelation) -> Result<GuestRelation, OfficeError>;
    async fn create_host(&self, relation: HostRelation) -> Result<HostRelation, OfficeError>;
    async fn find_by_guest_id(&self, guest_id: String) -> Result<Vec<GuestRelation>, diesel::result::Error>;
    async fn find_by_host_id(&self, host_id: String) -> Result<Vec<HostRelation>, diesel::result::Error>;
    async fn find_all_guests(&self) -> Result<Vec<GuestRelation>, diesel::result::Error>;
    async fn find_all_hosts(&self) -> Result<Vec<HostRelation>, diesel::result::Error>;
}

pub struct RelationRepositoryImpl;

#[async_trait]
impl RelationRepository for RelationRepositoryImpl {

    async fn find_by_host_id(&self, host_id: String) -> Result<Vec<HostRelation>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::host_relations::table
            .filter(crate::schema::host_relations::user_id.eq(host_id))
            .load::<HostRelation>(&mut connection)?;
        Ok(result)
    }

    async fn find_by_guest_id(&self, host_id: String) -> Result<Vec<GuestRelation>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::guest_relations::table
            .filter(crate::schema::guest_relations::user_id.eq(host_id))
            .load::<GuestRelation>(&mut connection)?;
        Ok(result)
    }

    async fn create_guest(&self, relation: GuestRelation) -> Result<GuestRelation, OfficeError> {
        use crate::schema::guest_relations::dsl::*;

        let mut connection = establish_connection();
        diesel::insert_into(guest_relations)
            .values(&relation)
            .execute(&mut connection)
            .expect("Error saving new guest relation");

        Ok(relation)
    }

    async fn create_host(&self, relation: HostRelation) -> Result<HostRelation, OfficeError> {
        use crate::schema::host_relations::dsl::*;

        let mut connection = establish_connection();
        diesel::insert_into(host_relations)
            .values(&relation)
            .execute(&mut connection)
            .expect("Error saving new host relation");

        Ok(relation)
    }

    async fn find_all_guests(&self) -> Result<Vec<GuestRelation>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::guest_relations::table
            .load::<GuestRelation>(&mut connection)?;
        Ok(result)
    }

    async fn find_all_hosts(&self) -> Result<Vec<HostRelation>, diesel::result::Error> {
        let mut connection = establish_connection();
        let result = crate::schema::host_relations::table
            .load::<HostRelation>(&mut connection)?;
        Ok(result)
    }
}