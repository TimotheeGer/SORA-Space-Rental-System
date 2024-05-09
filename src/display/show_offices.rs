use crate::display::utils::{display_brand, display_processed_confirm, display_office};

use crate::repository::office::OfficeRepositoryImpl;
use crate::services::office::OfficeServiceImpl;

pub async fn show_offices_menu() {
    
    let office_service = OfficeServiceImpl {
        office_repository: Box::new(OfficeRepositoryImpl),
    };
    
    
    loop {
        
        display_brand();
        
        match office_service.find_all().await {
            Ok(offices) => {
                if offices.len() == 0 {
                    println!("Aucun bureau disponible pour le moment.");
                } else {
                    println!("Voici nos bureau!");
                    for office in offices {
                        if office.id.starts_with("ofc") {
                            display_office(&office);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Erreur lors de la récupération des bureaux: {:?}", e);
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