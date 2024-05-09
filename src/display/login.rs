use std::thread::sleep;
use std::time::Duration as duration;

use colored::Colorize;

use crate::display::host::host_menu;
use crate::display::guest::guest_menu;
use crate::display::utils::{display_brand, display_processed_confirm, display_processed_inputs};

use crate::services::user::UserServiceImpl;
use crate::repository::user::UserRepositoryImpl;

use crate::utils::user;


pub async fn login_menu() {
    
    let user_service = UserServiceImpl {
        user_repository: Box::new(UserRepositoryImpl),
    };

    loop {

        display_brand();
        println!("Login:");

        let first_n = match display_processed_inputs("Entrez votre prénom".to_owned(), "Prénom".to_owned()) {
            Ok(first_n) => first_n,
            Err(_) => break
        };
        let last_n = match display_processed_inputs("Entrez votre nom".to_owned(), "Nom".to_owned()) {
            Ok(last_n) => last_n,
            Err(_) => break
        };
        let comfirm = match display_processed_confirm("Confirmez-vous ?".to_owned(), "".to_owned()) {
            Ok(comfirm) => comfirm,
            Err(_) => break
        };

        if comfirm {
            if let Ok(user) = user_service.find_by_name(&first_n.clone(), &last_n.clone()).await {
                user::set_user(user.clone());
                println!("connection réussie !");
                sleep(duration::from_secs(1));
                match user.user_role.as_str() {
                    "HOST" => host_menu().await,
                    "GUEST" => guest_menu().await,
                    _ => guest_menu().await
                }
                break;
            } else {
                match display_processed_confirm(format!("{}", "vous n'avez pas de compte voulez vous quitter?".red()), "vous devez creer un compte pour acceder au service".to_owned()) {
                    Ok(comfirm) => {
                        if comfirm {
                            break;
                        } 
                    },
                    Err(_) => break
                };
            }
        }
    }
}