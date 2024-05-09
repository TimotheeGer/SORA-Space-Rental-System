use crate::models::relation::*;
use crate::models::office::OfficeError;

use crate::repository::relation::RelationRepository;

pub struct RelationServiceImpl {
    pub repository: Box<dyn RelationRepository>
}

impl RelationServiceImpl {
    
    pub async fn create_guest(&self, relation: GuestRelation) -> Result<GuestRelation, OfficeError> {
        self.repository.create_guest(relation).await
    }

    pub async fn create_host(&self, relation: HostRelation) -> Result<HostRelation, OfficeError> {
        self.repository.create_host(relation).await
    }

    pub async fn find_all_guests(&self) -> Result<Vec<GuestRelation>, diesel::result::Error> {
        self.repository.find_all_guests().await
    }

    pub async fn find_all_hosts(&self) -> Result<Vec<HostRelation>, diesel::result::Error> {
        self.repository.find_all_hosts().await
    }

    pub async fn find_by_host_id(&self, host_id: String) -> Result<Vec<HostRelation>, diesel::result::Error> {
        self.repository.find_by_host_id(host_id).await
    }

    pub async fn find_by_guest_id(&self, guest_id: String) -> Result<Vec<GuestRelation>, diesel::result::Error> {
        self.repository.find_by_guest_id(guest_id).await
    }
}