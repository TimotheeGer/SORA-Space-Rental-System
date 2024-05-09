use crate::models::user::User;
use crate::repository::user::UserRepository;

pub struct UserServiceImpl {
    pub user_repository: Box<dyn UserRepository>,
}

impl UserServiceImpl {

    pub async fn find_by_name(&self, first_name: &String, last_name: &String) -> Result<User, diesel::result::Error> {
        self.user_repository.find_by_name(first_name, last_name).await
    }

    pub async fn creat(&self, user: User) -> Result<User, diesel::result::Error> {
        self.user_repository.creat(user).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, diesel::result::Error> {
        self.user_repository.find_all().await
    }

    pub async fn find_by_id(&self, id: &String) -> Result<User, diesel::result::Error> {
        self.user_repository.find_by_id(id).await
    }
}