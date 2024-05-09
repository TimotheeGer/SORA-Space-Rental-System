use diesel::prelude::*;
use crate::{models::user::User, schema::users};

use crate::db::establish_connection;
use async_trait::async_trait;


#[async_trait]
pub trait UserRepository: Send+Sync{
    async fn find_by_name(&self, first_name: &String, last_name: &String) -> Result<User, diesel::result::Error>;
    async fn creat(&self, user: User) -> Result<User, diesel::result::Error>;
    async fn find_all(&self) -> Result<Vec<User>, diesel::result::Error>;
    async fn find_by_id(&self, id: &String) -> Result<User, diesel::result::Error>; 
}

pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    
    async fn find_by_name(&self, first_name: &String, last_name: &String) -> Result<User, diesel::result::Error> {
        let mut connection = establish_connection();
        let user = users::table
            .filter(users::first_name.eq(first_name))
            .filter(users::last_name.eq(last_name))
            .first(&mut connection)?;
        Ok(user)
    }
    
    async fn find_all(&self) -> Result<Vec<User>, diesel::result::Error> {
        let mut connection = establish_connection();
        let users = users::table.load::<User>(&mut connection)?;
        Ok(users)
    }

    async fn creat(&self, user: User) -> Result<User, diesel::result::Error> {
        let mut connection = establish_connection();
        let user = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&mut connection)?;
        Ok(user)
    }

    async fn find_by_id(&self, id: &String) -> Result<User, diesel::result::Error> {
        let mut connection = establish_connection();
        let user = users::table
            .filter(users::id.eq(id))
            .first(&mut connection)?;
        Ok(user)
    }
    // Implémentez les autres méthodes ici
}