use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use inquire::{DateSelect, Text, required};
use std::process::Command;
use colored::*;


use crate::models::{contract::Contract, user::User, office::{Office, OfficeError}};

pub fn calendar_select_date(custom_msg: String) -> Result<DateTime<Utc>, OfficeError> {
    
    let res = DateSelect::new(custom_msg.as_str())
    .with_default(Utc::now().naive_utc().into())
    .with_min_date(Utc::now().naive_utc().into())
    .with_max_date(chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
    .with_week_start(chrono::Weekday::Mon)
    .with_help_message("Veuillez sélectionner une période d'au moins 4 mois.")
    .prompt();

    let date_time_utc = match res {
        Ok(date ) => {
            let naive_datetime: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
            let datetime: DateTime<Utc> = Utc::from_utc_datetime(&Utc, &naive_datetime);
            Some(datetime)
        },
        Err(_) => {
            println!("There was an error in the system.");
            return Err(OfficeError::Custom("There was an error in the system.".to_owned()));
        }
    };
    if date_time_utc.is_none() {
        return Err(OfficeError::Custom("There was an error in the system.".to_owned()));
    }
    return Ok(date_time_utc.unwrap())
}

pub fn display_processed_inputs(custom_msg: String, custom_placeholder: String) -> Result<String, OfficeError>{

    let input = Text::new(custom_msg.as_str())
        .with_placeholder(custom_placeholder.as_str())
        .with_validator(required!())
        .prompt();

    match input {
        Ok(input) => Ok(input),
        Err(_) => Err(OfficeError::Custom("An error happened when asking for your input, try again later.".to_owned())),
    }
}

pub fn display_processed_select(custom_msg: String, options: Vec<&str>) -> Result<String, OfficeError>{

    let ans = inquire::Select::new(custom_msg.as_str(), options).prompt();

    match ans {
        Ok(choice) => Ok(choice.to_string()),
        Err(_) => Err(OfficeError::Custom("There was an error, please try again".to_owned())),
    }
}

pub fn display_processed_confirm(custom_msg: String, custom_help: String) -> Result<bool, OfficeError>{

    let ans = inquire::Confirm::new(custom_msg.as_str())
        .with_default(true)
        .with_help_message(custom_help.as_str())
        .prompt();

    match ans {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(_) => Err(OfficeError::Custom("Error with questionnaire, try again later".to_owned())),
    }
}

fn clear_screen() {
    let _ = Command::new("clear").status();
}

pub fn display_brand() {
    clear_screen();
    println!("                {}", "Bienvenue chez".bold());
    println!(r"
     ██  ███████╗ ██████╗ ██████╗  █████╗ 
     █   ██╔════╝██╔═══██╗██╔══██╗██╔══██╗
         ███████╗██║   ██║██████╔╝███████║
      █  ╚════██║██║   ██║██╔══██╗██╔══██║
     ██  ███████║╚██████╔╝██║  ██║██║  ██║
         ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝                                  
    ");
    println!("    Le premier opérateur de bureaux inoccupés
    
    ");
}

pub fn display_user_connected(user: &String, role: &String) {
    println!("{} : {} {}", user.bold(), role.magenta().bold(), "Connecté".green());
    println!();
}

pub fn display_office(office: &Office) {
    println!("----------------------------------------------------------");
    println!("Bureau: {}", office.name);
    println!("Adresse: {}", office.address_text);
    println!("Superficie: {}m²", office.surface);
    println!("{} postes", office.num_posts.unwrap());
    println!("{}€/par poste par mois", office.price_per_post.unwrap());
    println!("----------------------------------------------------------");
}

pub fn display_contract(contract: &Contract, office: &Office, host: User, guest: User) {

    println!("----------------------------------------------------------");
    println!("{}", "- Contrat de loacation de bureau -".bold());
    println!();
    println!("Creation: {}", contract.created_at.format("%d-%m-%Y").to_string());
    println!("Bureau: {}", office.name.clone());
    println!("Adresse: {}", office.address_text.clone());
    println!("{}€/par poste par mois", office.price_per_post.unwrap());
    println!("Propriétaire: {}", format!("{} {}", host.first_name, host.last_name));
    println!("Locataire: {}", format!("{} {}", guest.first_name, guest.last_name));
    println!("Date de début: {}", contract.start_date.format("%d-%m-%Y").to_string());
    println!("Date de fin: {}", contract.end_date.format("%d-%m-%Y").to_string());
    println!("Nombre de postes: {}", contract.num_posts);
    println!("Coût total mensuel HT: {}€/mois", contract.monthly_amount);
    println!("TVA (20%): {}€/mois", (contract.monthly_amount as f32 * 0.2) as i32);
    println!("Coût total TTC: {}€/mois", (contract.monthly_amount as f32 * 1.2) as i32);
    println!("----------------------------------------------------------");
    println!();
}

pub fn display_database() {
    clear_screen();
    println!("                          {}", "Bienvenue dans la".bold());
    println!(r"
    ██████   █████  ████████  █████  ██████   █████  ███████ ███████ 
    ██   ██ ██   ██    ██    ██   ██ ██   ██ ██   ██ ██      ██      
    ██   ██ ███████    ██    ███████ ██████  ███████ ███████ █████   
    ██   ██ ██   ██    ██    ██   ██ ██   ██ ██   ██      ██ ██      
    ██████  ██   ██    ██    ██   ██ ██████  ██   ██ ███████ ███████ 
                                                                     
    ");
    println!("");

}