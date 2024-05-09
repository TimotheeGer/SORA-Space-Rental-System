use crate::display::utils::{display_database, display_processed_confirm};
use inquire::{Password, PasswordDisplayMode};
use dotenv::dotenv;
use std::env;


pub async fn password_menu() -> bool {
    
    dotenv().ok();

    let database_pwd = env::var("DATABASE_PWD").expect("error DATABASE_PWD");

    loop {
        
        display_database();

        let name = Password::new("Entre le mot de passe:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt();

        match name {
            Ok(mdp ) => {
                if mdp == database_pwd {
                    return true;
                } else {
                    println!("Désolé, le mot de passe est incorrect.");
                }
            },
            Err(_) => return false
        }

        match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
            Ok(comfirm) => {
                if comfirm {
                    return false;
                }
            },
            Err(_) => return false
        };
    }
}