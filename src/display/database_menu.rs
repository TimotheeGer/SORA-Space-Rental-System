use crate::display::utils::{display_database, display_processed_select};
use crate::display::indice::indice_menu;
use crate::display::password::password_menu;
use crate::display::show_database::show_database;

pub async fn database_menu() {
    
    loop {
        display_database();

        let choice = match display_processed_select("Menu base de données:".to_owned(), vec!["Mot de passe", "Indice", "Quitter"]) {
            Ok(choice) => choice,
            Err(_) => {
                break;
            }
        };

        match choice.as_str() {
            "Mot de passe" => {
               match password_menu().await {
                     true => show_database().await,
                     false => continue
               };
            }
            "Indice" => {
                indice_menu();
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