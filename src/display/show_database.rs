use crate::display::utils::{display_database, display_processed_select, display_processed_confirm};
use crate::services::user::UserServiceImpl;
use crate::repository::user::UserRepositoryImpl;
use crate::services::office::OfficeServiceImpl;
use crate::repository::office::OfficeRepositoryImpl;
use crate::services::contract::ContractServiceImpl;
use crate::repository::contract::ContractRepositoryImpl;
use crate::services::relation::RelationServiceImpl;
use crate::repository::relation::RelationRepositoryImpl;


pub async fn show_database() {

    let user_service = UserServiceImpl {
        user_repository: Box::new(UserRepositoryImpl)
    };

    let contract_service = ContractServiceImpl {
        contract_repository: Box::new(ContractRepositoryImpl),
        relation_service: RelationServiceImpl {
            repository: Box::new(RelationRepositoryImpl)
        },
        office_service: OfficeServiceImpl {
            office_repository: Box::new(OfficeRepositoryImpl)
        }
    };

    
    loop {
        display_database();

        let choice = match display_processed_select("Menu Datebase:".to_owned(), vec!["Users", "Offices", "Contracts", "Guest_relations", "host_relations", "Quitter"]) {
            Ok(choice) => choice,
            Err(_) => {
                break;
            }
        };

        match choice.as_str() {
            "Users" => {
                match user_service.find_all().await {
                    Ok(users) => println!("{:#?}", users),
                    Err(_) => println!("Erreur lors de la récupération des utilisateurs")
                };
                match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
                    Ok(comfirm) => if comfirm { continue; },
                    Err(_) => continue
                };
            }
            "Offices" => {
                match contract_service.office_service.find_all().await {
                    Ok(offices) => println!("{:#?}", offices),
                    Err(_) => println!("Erreur lors de la récupération des bureaux")
                };
                match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
                    Ok(comfirm) => if comfirm { continue; },
                    Err(_) => continue
                };            
            }
            "Contracts" => {
                match contract_service.find_all().await {
                    Ok(contracts) => println!("{:#?}", contracts),
                    Err(_) => println!("Erreur lors de la récupération des contrats")
                };
                match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
                    Ok(comfirm) => if comfirm { continue; },
                    Err(_) => continue
                };            
            }
            "Guest_relations" => {
                match contract_service.relation_service.find_all_guests().await {
                    Ok(relations) => println!("{:#?}", relations),
                    Err(_) => println!("Erreur lors de la récupération des relations guest")
                };
                match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
                    Ok(comfirm) => if comfirm { continue; },
                    Err(_) => continue
                };            
            }
            "host_relations" => {
                match contract_service.relation_service.find_all_hosts().await {
                    Ok(relations) => println!("{:#?}", relations),
                    Err(_) => println!("Erreur lors de la récupération des relations host")
                };
                match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
                    Ok(comfirm) => if comfirm { continue; },
                    Err(_) => continue
                };            
            }
            "Quitter" => {
                break;
            }
            _ => {
                println!("Choix invalide. Veuillez entrer un numéro valide.");
            }
        }
    }
}