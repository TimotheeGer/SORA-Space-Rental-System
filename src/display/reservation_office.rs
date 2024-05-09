use colored::*;

use crate::models::office::Office;
use crate::models::contract::Contract;
use crate::display::utils::{display_brand, display_processed_confirm, display_user_connected, calendar_select_date, display_office, display_processed_inputs};
use crate::utils::{user, calendar_management::*};

use crate::services::office::OfficeServiceImpl;
use crate::repository::office::OfficeRepositoryImpl;
use crate::services::contract::ContractServiceImpl;
use crate::repository::contract::ContractRepositoryImpl;
use crate::services::relation::RelationServiceImpl;
use crate::repository::relation::RelationRepositoryImpl;

pub async fn reservation_office_menu(office: &Office) {

    let contract_service = ContractServiceImpl {
        contract_repository: Box::new(ContractRepositoryImpl),
        relation_service: RelationServiceImpl {
            repository: Box::new(RelationRepositoryImpl)},
        office_service: OfficeServiceImpl {
            office_repository: Box::new(OfficeRepositoryImpl),
        }
    };
    
    let user = user::get_user().unwrap();
    let mut vec_contract: Vec<Contract> = Vec::new();

    loop {
        let mut exit = false;
        display_brand();
        display_user_connected(&user.first_name.clone(), &user.user_role.clone());
        display_office(&office.clone());

        println!("Nous vous fournirons les dates disponibles pour les postes que vous souhaitez réserver.");
        println!();

        let mut num_posts = "40".to_owned();
        loop {

            num_posts = match display_processed_inputs("Entrez le nombre de poste".to_owned(), format!("min.40 max.{}", office.num_posts.unwrap().clone())) {
                Ok(num_posts) => num_posts,
                Err(_) => {
                    exit = true;
                    break;
                }
            };
            if num_posts.parse::<i32>().is_ok() && num_posts.parse::<i32>().unwrap() >= 40 && num_posts.parse::<i32>().unwrap() <= office.num_posts.unwrap().clone() {
                break;
            } else {
                println!("{}{}", "Le nombre de poste doit être un nombre entre 40 et ".red(), office.num_posts.unwrap().clone().to_string().red());
            }
        }
        if exit {
            break;
        }

        let mut calendar = create_calendar_for_office(office).await;
        if !available_dates(&calendar.clone(), num_posts.parse::<u32>().unwrap(), true) {
            match display_processed_confirm("Quitter?".to_owned(), "".to_owned()) {
                Ok(comfirm) => {
                    if comfirm {
                        break;
                    } else {
                        continue;
                    }
                },
                Err(_) => break
            };
        };
        
        println!();
        println!("{}", "Imformation:".yellow());
        println!("Chaque location doit durer au moins 4 mois,");
        println!("avec un total obligatoire d'au moins 12 mois, cumulables sur plusieurs périodes.");
        println!();
        println!("Veuillez maintenant choisir vos dates.");
        println!();
        
        let mut total_date = 0;

        loop {
            
            let  start_date = match calendar_select_date("Date de début".to_owned()) {
                Ok(date) => date,
                Err(_) => {
                    exit = true;
                    break;
                }
            };
            let  end_date = match calendar_select_date("Date de fin".to_owned()) {
                Ok(date) => date,
                Err(_) => {
                    exit = true;
                    break;
                }
            };

            println!();
            
            if start_date > end_date {
                println!("{}", "La date de début doit être antérieure à la date de fin.".red());
                continue;
            }

            if total_days_between_dates(start_date, end_date) < 120 {
                println!("{}", "La réservation doit durer au moins 4 mois.".red());
                println!();
                continue;
            }
            
            total_date += total_days_between_dates(start_date, end_date);
            println!("Total mois : {}", (total_date / 30).to_string().magenta());
            
            if total_date < 360 {
                
                if !check_date_availability(start_date, end_date, &calendar.clone(), num_posts.parse::<u32>().unwrap()) {
                    println!("{}", "Les dates que vous avez choisies ne sont pas disponibles.".red());
                    total_date -= total_days_between_dates(start_date, end_date);
                    continue;
                } else {
                    println!("{}", "réservation valide.".green());
                    println!();
                    add_contract_dates(start_date, end_date, num_posts.parse::<u32>().unwrap(), &mut calendar);
                    if !available_dates(&calendar.clone(), num_posts.parse::<u32>().unwrap(), false) {
                        match display_processed_confirm("quitter?".to_owned(), "".to_owned()) {
                            Ok(comfirm) => {
                                if comfirm {
                                    exit = true;
                                    break;
                                }
                            },
                            Err(_) => break
                        };
                    }
                }
                
                println!("{}", "Les réservation doivent durer au moins 12 mois au total.".yellow());
                
                if 360 - total_date < 120 {
                    println!("Vous devez reserver encore {} mois minimum", 4);
                    println!();
                } else {
                    println!("Vous devez reserver encore {} mois minimum", (360 - total_date) / 30 + 1);
                    println!();
                }
                
                let new_contract = Contract::new(office.owner_id.clone(), user.id.clone(), office.id.clone(), num_posts.parse::<i32>().unwrap(), office.price_per_post.unwrap().clone() * num_posts.parse::<i32>().unwrap(), start_date, end_date);
                vec_contract.push(new_contract);
                continue;

            } else {
                if !check_date_availability(start_date, end_date, &calendar.clone(), num_posts.parse::<u32>().unwrap()) {
                    println!("{}", "Les dates que vous avez choisies ne sont pas disponibles.".red());
                    total_date -= total_days_between_dates(start_date, end_date);
                    continue;
                } 
                let new_contract = Contract::new(office.owner_id.clone(), user.id.clone(), office.id.clone(), num_posts.parse::<i32>().unwrap(), office.price_per_post.unwrap().clone() * num_posts.parse::<i32>().unwrap(), start_date, end_date);
                vec_contract.push(new_contract);
                println!("{} pour un total de {} mois au prix de {}€/mois", "La réservation est valide.".bold(), total_date / 30 + 1, office.price_per_post.unwrap().clone() * num_posts.parse::<i32>().unwrap());
                println!();
                break;
            }
            
        }
        if exit {
            break;
        }   

        match display_processed_confirm("Comfirmer?".to_owned(), "Dépechez-vous les bureau parte trés vite!".to_owned()) {
            Ok(comfirm) => {
                if comfirm {
                    let _ = contract_service.creates(vec_contract, office, user).await;
                    break;
                } else {
                    vec_contract.clear();
                    continue;
                }
            },
            Err(_) => break
        };


    }


}