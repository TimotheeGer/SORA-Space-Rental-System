use crate::display::utils::{display_brand, display_processed_confirm, display_user_connected, display_contract};

use crate::services::office::OfficeServiceImpl;
use crate::repository::office::OfficeRepositoryImpl;
use crate::services::contract::ContractServiceImpl;
use crate::repository::contract::ContractRepositoryImpl;
use crate::services::relation::RelationServiceImpl;
use crate::repository::relation::RelationRepositoryImpl;
use crate::services::user::UserServiceImpl;
use crate::repository::user::UserRepositoryImpl;
use crate::utils::user;

pub async fn show_host_contract() {
    
    let contract_service = ContractServiceImpl {
        contract_repository: Box::new(ContractRepositoryImpl),
        relation_service: RelationServiceImpl {
            repository: Box::new(RelationRepositoryImpl)},
        office_service: OfficeServiceImpl {
            office_repository: Box::new(OfficeRepositoryImpl),
        }
    };

    let user_service = UserServiceImpl {
        user_repository: Box::new(UserRepositoryImpl)
    };
    
    let user = user::get_user().unwrap();
    
    loop {
        
        display_brand();
        display_user_connected(&user.first_name.clone(), &user.user_role.clone());

        
        match contract_service.find_by_host_id(user.id.clone()).await {
            Ok(contracts) => {
                if contracts.len() == 0 {
                    println!("Aucun contrat.");
                    println!(); 
                } else {
                    println!("Mes contrats!");
                    for contract in contracts {
                        let user_guest = match user_service.find_by_id(&contract.guest_id.clone()).await {
                            Ok(host) => host,
                            Err(_) => {
                                println!("Erreur lors de la récupération du guest");
                                break;
                            }
                        };
                        let office = match contract_service.office_service.find_by_id(contract.office_id.clone()).await {
                            Ok(office) => office,
                            Err(_) => {
                                println!("Erreur lors de la récupération du bureau");
                                break;
                            }
                        };
                        display_contract(&contract, &office, user.clone(), user_guest);
                    }
                    println!()
                }
            },
            Err(e) => {
                println!("Erreur lors de la récupération des contrats: {:?}", e);
            }
        }
        
        match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
            Ok(comfirm) => {
                if comfirm {
                    break;
                }
            },
            Err(_) => break
            
        };
    }
}