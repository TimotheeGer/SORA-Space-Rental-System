use std::thread::sleep;

use crate::display::utils::{display_brand, display_processed_select, display_user_connected};
use crate::display::{show_offices_user, add_office};
use std::time::Duration as duration;
use crate::utils::user;

use super::show_host_contract;


pub async fn host_menu() {
    
    let user = user::get_user().unwrap();

    loop {
        display_brand();
        display_user_connected(&user.first_name.clone(), &user.user_role.clone());

        let choice = match display_processed_select("Menu Host:".to_owned(), vec!["Ajouter un bureau", "Mes bureaux", "Mes contrats", "Deconnection"]) {
            Ok(choice) => choice,
            Err(_) => {
                user::clear_user();
                break;
            }
        };
        
        match choice.as_str() {
            "Ajouter un bureau" => {
                add_office::add_office_menu().await;
            }
            "Mes bureaux" => {
                show_offices_user::show_offices_user().await;
            }
            "Mes contrats" => {
                show_host_contract::show_host_contract().await;
            }
            "Deconnection" => {
                user::clear_user();
                break;
            }
            _ => {
                println!("Choix invalide. Veuillez entrer un numéro valide.");
                sleep(duration::from_secs(2));
            }
        }
    }
}