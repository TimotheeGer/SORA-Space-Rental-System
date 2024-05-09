use crate::models::office::*;
use crate::repository::office::OfficeRepository;

pub struct OfficeServiceImpl {
    pub office_repository: Box<dyn OfficeRepository>,
}

impl OfficeServiceImpl {

    pub async fn creat(&self, office: Office) -> Result<Office, diesel::result::Error> {
        self.office_repository.creat(office).await
    }

    pub async fn find_by_prefix_id(&self, prefix_id: String) -> Result<Vec<Office>, diesel::result::Error> {
        self.office_repository.find_by_prefix_id(prefix_id).await
    }

    // pub async fn find_by_owner_id(&self, owner_id: String) -> Result<Vec<Office>, diesel::result::Error> {
    //     self.office_repository.find_by_owner_id(owner_id).await
    // }

    pub async fn find_all(&self) -> Result<Vec<Office>, diesel::result::Error> {
        self.office_repository.find_all().await
    }

    pub async fn find_by_name(&self, name: String) -> Result<Office, diesel::result::Error> {
        self.office_repository.find_by_name(name).await
    }

    pub async fn find_by_id(&self, id: String) -> Result<Office, diesel::result::Error> {
        self.office_repository.find_by_id(id).await
    }

    pub fn maximum_number_workstations_for_surface(&self, surface: f64) -> i32 {

        let mut max_posts;

        if surface < 84.0 {
            max_posts = (surface / 8.0) * 5.0;
        }
        else {
            max_posts = (surface / 7.0) * 5.0;
        }

        if max_posts > 180.0 {
            max_posts = 180.0;
        }

        return max_posts as i32
    }



}