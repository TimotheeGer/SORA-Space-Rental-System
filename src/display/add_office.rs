use std::thread::sleep;
use std::time::Duration as duration;
use colored::*;

use crate::display::utils::{display_brand, display_processed_confirm, display_processed_inputs, display_user_connected};
use crate::models::office::Office;
use crate::models::relation::HostRelation;
use crate::utils::user;
use crate::services::office::OfficeServiceImpl;
use crate::repository::office::OfficeRepositoryImpl;
use crate::services::relation::RelationServiceImpl;
use crate::repository::relation::RelationRepositoryImpl;


pub async fn add_office_menu() {
    
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
        
        let name = match display_processed_inputs("Entrez le nom de votre espace: ".to_owned(), "ex: espace opera".to_owned()) {
            Ok(name) => name,
            Err(_) => break
        };
        
        let address = match display_processed_inputs("Entrez l'adresse: ".to_owned(), "adresse postal".to_owned()) {
            Ok(address) => address,
            Err(_) => break
        };
        
        let mut surface: String;
        loop {
            surface = match display_processed_inputs("Entrez la surface".to_owned(), "surface min 64m²".to_owned()) {
                Ok(surface) => surface,
                Err(_) => continue
            };
            if surface.parse::<f64>().is_ok() && surface.parse::<f64>().unwrap() >= 64.0{
                break;
            } else {
                println!("{}", "La surface doit être un nombre!".red());
            }
        }

        let mut num_posts: String;
        loop {

            let mut max_post = office_service.maximum_number_workstations_for_surface(surface.parse::<f64>().unwrap());
            if max_post > 180 {
                max_post = 180;
            }

            num_posts = match display_processed_inputs("Entrez le nombre de poste".to_owned(), format!("min.40 max.{} pour {}m²", max_post, surface)) {
                Ok(num_posts) => num_posts,
                Err(_) => continue
            };
            if num_posts.parse::<i32>().is_ok() && num_posts.parse::<i32>().unwrap() >= 40 && num_posts.parse::<i32>().unwrap() <= max_post {

                break;
            } else {
                println!("{}{}", "Le nombre de poste doit être un nombre entre 40 et ".red(), max_post.to_string().red());
            }
        }

        let mut price_per_post: String;
        loop {
            price_per_post = match display_processed_inputs("Entrez le prix par poste".to_owned(), "min.300€ max.800€".to_owned()) {
                Ok(price_per_post) => price_per_post,
                Err(_) => continue
            };
            if price_per_post.parse::<f64>().is_ok() && price_per_post.parse::<f64>().unwrap() >= 300.0 && price_per_post.parse::<f64>().unwrap() <= 800.0 {
                break;
            } else {
                println!("{}", "Le prix par poste doit être un nombre entre 300 et 800!".red());
            }
        }

        let owner_id = user.id.clone();
        let comfirm = match display_processed_confirm("Confirmez-vous ?".to_owned(), "This data is stored for good reasons".to_owned()) {
            Ok(comfirm) => comfirm,
            Err(_) => break
        };
        
        if comfirm {

            if let Ok(office) = office_service.find_by_name(name.clone()).await {
                println!("Ce Bureau est déja enregistré : {}", office.name.red());
                sleep(duration::from_secs(1));
                break;
            }

            let new_office = Office::new(name.clone(), address.clone(), surface.parse::<f64>().unwrap(), Some(num_posts.parse::<i32>().unwrap()), Some(price_per_post.parse::<i32>().unwrap()), None, owner_id.clone(), "ofc".to_owned());
           
            match office_service.creat(new_office).await {
                Ok(office) => {
                    let new_relation = HostRelation::new(user.id, office.id.clone());
                    if relation_service.create_host(new_relation).await.is_err() {
                        println!("{} {}", "Erreur lors de la création de la relation".red(), "Host".red());
                        break;
                    }
                    println!("{} {} {}", "Bureau".green(), office.name, "ajouté avec succès!".green());
                    sleep(duration::from_secs(1));
                    break;
                },
                Err(error) => {
                    println!("error: {}", error);
                    sleep(duration::from_secs(1));
                }
            }

        }
    }
}