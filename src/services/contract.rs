use crate::models::contract::Contract;
use crate::models::office::Office;
use crate::models::user::User;
use crate::models::relation::*;
use crate::repository::contract::ContractRepository;
use crate::services::relation::RelationServiceImpl;
use crate::services::office::OfficeServiceImpl;
use nanoid::nanoid;

pub struct ContractServiceImpl {
    pub contract_repository: Box<dyn ContractRepository>,
    pub relation_service: RelationServiceImpl,
    pub office_service: OfficeServiceImpl
}

impl ContractServiceImpl {

    pub async fn creates(&self, contracts: Vec<Contract>, office: &Office, user: User) -> Result<(), diesel::result::Error> {
        
        for contract in contracts {
   
            match self.contract_repository.creat(contract).await {
                Ok(contract) => {

                    if contract.num_posts == office.num_posts.unwrap() {
                       let new_relation = GuestRelation::new(user.id.clone(), contract.office_id);
                       match self.relation_service.create_guest(new_relation).await {
                            Ok(_) => return Ok(()),
                            Err(_) => return Err(diesel::result::Error::NotFound)
                       };
                    } else {
                        
                        let new_office = Office::new(format!("Espace#{}", nanoid!(10)), office.address_text.clone(), office.surface, Some(contract.num_posts), office.price_per_post, Some(office.id.clone()), contract.guest_id, "spl".to_owned());

                        match self.office_service.creat(new_office).await {
                            Ok(office) => {
                                let new_relation = GuestRelation::new(user.id.clone(), office.id.clone());
                                self.relation_service.create_guest(new_relation).await.unwrap();
                            },
                            Err(err) => return Err(err)
                        };
                    }

                },
                Err(err) => {
                    return Err(err);
                }
            };



   
        }
        
        return Ok(());
    }

    // pub async fn create(&self, new_contract: Contract) -> Result<Contract, diesel::result::Error> {
    //     self.contract_repository.creat(new_contract).await
    // }

    pub async fn find_by_office_id(&self, office_id: String) -> Result<Vec<Contract>, diesel::result::Error> {
        self.contract_repository.find_by_office_id(office_id).await
    }

    pub async fn find_by_host_id(&self, host_id: String) -> Result<Vec<Contract>, diesel::result::Error> {
        self.contract_repository.find_by_host_id(host_id).await
    }

    pub async fn find_by_guest_id(&self, guest_id: String) -> Result<Vec<Contract>, diesel::result::Error> {
        self.contract_repository.find_by_guest_id(guest_id).await
    }

    pub async fn find_all(&self) -> Result<Vec<Contract>, diesel::result::Error> {
        self.contract_repository.find_all().await
    }

}