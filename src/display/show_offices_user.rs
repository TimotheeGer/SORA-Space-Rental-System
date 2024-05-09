use crate::display::utils::{display_brand, display_processed_confirm, display_user_connected, display_office};

use crate::repository::office::OfficeRepositoryImpl;
use crate::services::office::OfficeServiceImpl;
use crate::services::relation::RelationServiceImpl;
use crate::repository::relation::RelationRepositoryImpl;
use crate::utils::user;

pub async fn show_offices_user() {
    
    let office_service = OfficeServiceImpl {
        office_repository: Box::new(OfficeRepositoryImpl),
    };

    let relation_service = RelationServiceImpl {
        repository: Box::new(RelationRepositoryImpl),
    };
    
    let user = user::get_user().unwrap();
    
    loop {
        
        display_brand();
        display_user_connected(&user.first_name.clone(), &user.user_role.clone());

        match user.user_role.as_str() {
            "GUEST" => {
                match relation_service.find_by_guest_id(user.id.clone()).await {
                    Ok(relations) => {
                        if relations.len() == 0 {
                            println!("Vous n'avez pas de bureaux pour le moment.");
                            println!();
                        } else {
                            for relation in relations {
                                match office_service.find_by_id(relation.office_id.clone()).await {
                                    Ok(office) => display_office(&office),
                                    Err(_) => break
                                }
                            }
                        }
                    },
                    Err(_) => break
                }
            },
            "HOST" => {
                match relation_service.find_by_host_id(user.id.clone()).await {
                    Ok(relations) => {
                        if relations.len() == 0 {
                            println!("Vous n'avez pas de bureaux pour le moment.");
                            println!();
                        } else {
                            for relation in relations {
                                match office_service.find_by_id(relation.office_id.clone()).await {
                                    Ok(office) => display_office(&office),
                                    Err(_) => break
                                }
                            }
                        }
                    },
                    Err(_) => break
                }
            },
            _ => break
        };
        
        
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