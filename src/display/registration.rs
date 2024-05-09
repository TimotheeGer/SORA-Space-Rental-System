use std::thread::sleep;
use std::time::Duration as duration;

use crate::{display::utils::{display_brand, display_processed_confirm, display_processed_inputs, display_processed_select}, repository::user::UserRepositoryImpl};
use crate::display::host::host_menu;
use crate::display::guest::guest_menu;
use crate::models::user::{User, UserRole};
use crate::services::user::UserServiceImpl;
use crate::utils::user;


pub async fn registration_menu() {
    
    let user_service = UserServiceImpl {
        user_repository: Box::new(UserRepositoryImpl),
    };

    loop {
        display_brand();
        
        let user_role = match display_processed_select("Inscription:".to_owned(), vec!["HOST", "GUEST"]) {
            Ok(role) => role,
            Err(_) => break
        };
        let first_name = match display_processed_inputs("Entrez votre prénom".to_owned(), "Prénom".to_owned()) {
            Ok(first_name) => first_name,
            Err(_) => break
        };
        let last_name = match display_processed_inputs("Entrez votre nom".to_owned(), "Nom".to_owned()) {
            Ok(last_name) => last_name,
            Err(_) => break
        };
        let comfirm = match display_processed_confirm("Confirmez-vous ?".to_owned(), "This data is stored for good reasons".to_owned()) {
            Ok(comfirm) => comfirm,
            Err(_) => break
        };
        
        if comfirm {

            if let Ok(user) = user_service.find_by_name(&first_name.clone(), &last_name.clone()).await {
                    println!("{} {} est déjà inscrit", user.first_name, user.last_name);
                    sleep(duration::from_secs(1));
                    break;
            }
            
            let user_role = match user_role.as_str() {
                "HOST" => UserRole::HOST,
                "GUEST" => UserRole::GUEST,
                _ => UserRole::GUEST
            };

            let new_user = User::new(first_name, last_name, user_role);

            match user_service.creat(new_user).await {
                Ok(user) => {
                    user::set_user(user.clone());
                    println!("Bienvennu {} votre inscription est réussie !", user.first_name);
                    sleep(duration::from_secs(1));
                    match user.user_role.as_str() {
                        "HOST" => host_menu().await,
                        "GUEST" => guest_menu().await,
                        _ => guest_menu().await
                    }
                    break;
                },
                Err(error) => {
                    println!("error: {}", error);
                    sleep(duration::from_secs(2));
                }
            }
        }
    }
}