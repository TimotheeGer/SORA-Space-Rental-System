use diesel::prelude::*;
use async_trait::async_trait;

use crate::db::establish_connection;
use crate::models::contract::Contract;


#[async_trait]
pub trait ContractRepository: Send+Sync{
    async fn creat(&self, contract: Contract) -> Result<Contract, diesel::result::Error>;
    async fn find_by_host_id(&self, host_id: String) -> Result<Vec<Contract>, diesel::result::Error>;
    async fn find_by_guest_id(&self, guest_id: String) -> Result<Vec<Contract>, diesel::result::Error>;
    async fn find_by_office_id(&self, office_id: String) -> Result<Vec<Contract>, diesel::result::Error>;
    async fn find_all(&self) -> Result<Vec<Contract>, diesel::result::Error>;
}

pub struct ContractRepositoryImpl;

#[async_trait]
impl ContractRepository for ContractRepositoryImpl {
        
        async fn creat(&self, contract: Contract) -> Result<Contract, diesel::result::Error> {
            let mut connection = establish_connection();
            diesel::insert_into(crate::schema::contracts::table)
                .values(&contract)
                .execute(&mut connection)
                .expect("Error saving new contract");
            Ok(contract)
        }
    
        async fn find_by_host_id(&self, host_id: String) -> Result<Vec<Contract>, diesel::result::Error> {
            let mut connection = establish_connection();
            let result = crate::schema::contracts::table
                .filter(crate::schema::contracts::host_id.eq(host_id))
                .load::<Contract>(&mut connection)?;
            Ok(result)
        }
    
        async fn find_by_guest_id(&self, guest_id: String) -> Result<Vec<Contract>, diesel::result::Error> {
            let mut connection = establish_connection();
            let result = crate::schema::contracts::table
                .filter(crate::schema::contracts::guest_id.eq(guest_id))
                .load::<Contract>(&mut connection)?;
            Ok(result)
        }

        async fn find_by_office_id(&self, office_id: String) -> Result<Vec<Contract>, diesel::result::Error> {
            let mut connection = establish_connection();
            let result = crate::schema::contracts::table
                .filter(crate::schema::contracts::office_id.eq(office_id))
                .load::<Contract>(&mut connection)?;
            Ok(result)
        }
    
        async fn find_all(&self) -> Result<Vec<Contract>, diesel::result::Error> {
            let mut connection = establish_connection();
            let result = crate::schema::contracts::table
                .load::<Contract>(&mut connection)?;
            Ok(result)
        }
}