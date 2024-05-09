use std::thread::sleep;
use std::time::Duration as duration;

use crate::display::utils::{display_brand, display_processed_select, display_user_connected};
use crate::display::{show_offices_user, book_office};
use crate::utils::user;

use super::show_guest_contract;

pub async fn guest_menu() {
    
    let user = user::get_user().unwrap();

    loop {

        display_brand();
        display_user_connected(&user.first_name.clone(), &user.user_role.clone());

        let choice = match display_processed_select("Menu Guest:".to_owned(), vec!["Voir les bureaux", "Mes bureaux", "Mes contrats", "Deconnection"]) {
            Ok(choice) => choice,
            Err(_) => {
                user::clear_user();
                break;
            }
        };

        match choice.as_str() {
            "Voir les bureaux" => {
                book_office::book_office_menu().await;
            }
            "Mes bureaux" => {
                show_offices_user::show_offices_user().await;
            }
            "Mes contrats" => {
                show_guest_contract::show_guest_contract().await;
            }
            "Deconnection" => {
                user::clear_user();
                break;
            }
            _ => {
                println!("Choix invalide. Veuillez entrer un choix valide.");
                sleep(duration::from_secs(2));
            }
            
        }
    }
}