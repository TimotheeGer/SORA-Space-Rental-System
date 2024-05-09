use crate::display::reservation_office::reservation_office_menu;
use crate::display::utils::{display_brand, display_processed_confirm, display_processed_select, display_user_connected, display_office};
use crate::utils::user;

use crate::repository::office::OfficeRepositoryImpl;
use crate::services::office::OfficeServiceImpl;

pub async fn book_office_menu() {

    let office_service = OfficeServiceImpl {
        office_repository: Box::new(OfficeRepositoryImpl),
    };
    
    let user = user::get_user().unwrap();
    let mut exit = false;

    loop {
    
        display_brand();
        display_user_connected(&user.first_name.clone(), &user.user_role.clone());

        let offices = match office_service.find_by_prefix_id("ofc".to_owned()).await {
            Ok(office) => {
                if office.len() == 0 {
                    println!("Aucun bureau disponible pour le moment.");
                    match display_processed_confirm("Voulez vous reserver ?".to_owned(), "Dépechez-vous les bureau parte trés vite!".to_owned()) {
                        Ok(comfirm) => {
                            if comfirm {
                                break;
                            }
                        },
                        Err(_) => break
                    };
                }
                office
            },
            Err(e) => {
                println!("Erreur lors de la récupération des bureaux: {:?}", e);
                break;
            }
        };
        
        let office_names: Vec<String> = offices.clone().into_iter().map(|office| office.name).collect();
        let mut office_names_str: Vec<&str> = office_names.iter().map(|name| name.as_str()).collect();
        office_names_str.push("Quitter");

        let choice = match display_processed_select("Les bureaux:".to_owned(), office_names_str.clone()) {
            Ok(choice) => choice,
            Err(_) => {
                break;
            }
        };
        
        for office in &offices {
            if choice == "Quitter" {
                exit = true;
                break;
            }
            if office.name == choice && office.id.starts_with("ofc") {
                display_office(&office.clone());
                match display_processed_confirm("Voulez vous reserver ?".to_owned(), "Dépechez-vous les bureau parte trés vite!".to_owned()) {
                    Ok(comfirm) => {
                        if comfirm {
                            reservation_office_menu(office).await;
                            break;
                        } else {
                            continue;
                        }
                    },
                    Err(_) => continue
                };
            }
        }
        if exit {
            break;
        }
    }
}