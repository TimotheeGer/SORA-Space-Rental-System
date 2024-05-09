use std::thread::sleep;
use std::time::Duration as duration;

use crate::display::{registration, description, login, show_offices, database_menu};

use crate::display::utils::{display_processed_select, display_brand};

pub async fn display_menu() {

    loop {

        display_brand();

        let choix = match display_processed_select("Menu Principale:".to_owned(), vec!["Sora c'est quoi ?", "Voir les bureaux", "Inscription", "Connexion", "", "Base de données", "Quitter"]) {
            Ok(choix) => choix,
            Err(_) => {
                println!("Merci d'avoir utilisé le système de gestion de la société SORA. À bientôt !");
                break;
            }
        };

        match choix.as_str() {
            "Sora c'est quoi ?" => {
                description::description_menu();
            }
            "Voir les bureaux" => {
                show_offices::show_offices_menu().await;
            }
            "Inscription" => {
                registration::registration_menu().await;
            }
            "Connexion" => {
                login::login_menu().await;
            }
            "Quitter" => {
                println!("Merci d'avoir utilisé le système de gestion de la société SORA. À bientôt !");
                break;
            }
            "Base de données" => {
                database_menu::database_menu().await;
            }
            _ => {
                println!("Choix invalide. Veuillez entrer un choix valide.");
                sleep(duration::from_secs(1));
            }
        }
    }
}