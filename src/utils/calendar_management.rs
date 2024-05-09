use chrono::{DateTime, TimeZone, Utc, Datelike, Duration};
use std::collections::BTreeMap;

use crate::{models::office::Office, services::contract::ContractServiceImpl};
use crate::services::relation::RelationServiceImpl;
use crate::services::office::OfficeServiceImpl;

pub async fn create_calendar_for_office(office: &Office) -> BTreeMap<(i32, u32, u32), u32> {
    
    let contract_service = ContractServiceImpl {
        contract_repository: Box::new(crate::repository::contract::ContractRepositoryImpl),
        office_service: OfficeServiceImpl {
            office_repository: Box::new(crate::repository::office::OfficeRepositoryImpl)
        },
        relation_service: RelationServiceImpl {
            repository: Box::new(crate::repository::relation::RelationRepositoryImpl)
        }
    };
    
    let mut date_map = create_calendar(office.num_posts.unwrap() as u32);
    
    match contract_service.find_by_office_id(office.id.clone()).await {
        Ok(contracts) => {
            for contract in contracts {
                let start_date = contract.start_date;
                let end_date = contract.end_date;
                add_contract_dates(start_date, end_date, contract.num_posts as u32, &mut date_map);
            }
        },
        Err(_) => {}
    };
    
    return date_map
}

pub fn create_calendar(nb_posts: u32) -> BTreeMap<(i32, u32, u32), u32> {

    let date_now = Utc::now();
    let start_date = Utc.with_ymd_and_hms(date_now.year(), date_now.month(), date_now.day(), 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    let mut date_map: BTreeMap<(i32, u32, u32), u32> = BTreeMap::new();
    
    let mut current_date = start_date;
    
    while current_date <= end_date {
        date_map.insert((current_date.year(), current_date.month(), current_date.day()), nb_posts);
        current_date = current_date + Duration::days(1);
    }

    return date_map
}

pub fn add_contract_dates(start_date: DateTime<Utc>, end_date: DateTime<Utc>, nb_posts: u32, date_map: &mut BTreeMap<(i32, u32, u32), u32>) -> &mut BTreeMap<(i32, u32, u32), u32> {
    //reserve les dates si elle ne ce chevauche pas // ou ajoute les date des contrat existant
    let start_key = (start_date.year(), start_date.month(), start_date.day());
    let end_key = (end_date.year(), end_date.month(), end_date.day());

    for (_, value) in date_map.range_mut(start_key..=end_key) {
        *value = *value - nb_posts;
        // println!("Date : {:?}, Valeur : {}", key, value);
    }

    return date_map
}

pub fn check_date_availability(start_date: DateTime<Utc>, end_date: DateTime<Utc>, date_map: &BTreeMap<(i32, u32, u32), u32>, nb_posts: u32) -> bool {
    let start_key = (start_date.year(), start_date.month(), start_date.day());
    let end_key = (end_date.year(), end_date.month(), end_date.day());

    for (_, value) in date_map.range(start_key..=end_key) {
        if nb_posts > *value {
            return false
        }
    }
    return true
}

pub fn available_dates(date_map: &BTreeMap<(i32, u32, u32), u32>, nb_posts: u32, check_year: bool) -> bool {
    let mut tab_dispo: Vec<(DateTime<Utc>, DateTime<Utc>)> = Vec::new();
    let mut tab_dispo_final: Vec<(DateTime<Utc>, DateTime<Utc>)> = Vec::new();

    let mut start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let mut is_start = true;
    let post_resa = nb_posts;
   
    for (key, value) in date_map.iter() {
        if *value >= post_resa && is_start {
            start = Utc.with_ymd_and_hms(key.0, key.1, key.2, 0, 0, 0).unwrap();
            is_start = false;
        }
        else if *value < post_resa && !is_start {
            let mut end = Utc.with_ymd_and_hms(key.0, key.1, key.2, 0, 0, 0).unwrap();
            end = end - Duration::days(1);

            tab_dispo.push((start, end));
            is_start = true;
        }
    }

    if !is_start {
        let end = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        tab_dispo.push((start, end));
    }
    
    for date in tab_dispo {
        if total_days_between_dates(date.0, date.1) >= 120 {
            tab_dispo_final.push(date);
        }
    }

    if tab_dispo_final.len() == 0 {
        println!("Désolé, il n'y a pas de dates disponibles pour le nombre de postes que vous avez demandé.");
        return false
    } else {

        if check_year {
            let mut total_days = 0;

            for date in tab_dispo_final.clone() {
                total_days += total_days_between_dates(date.0, date.1);
            }

            if total_days < 360 {
                println!("Désolé, il n'y a pas de dates disponibles pour le nombre de postes que vous avez demandé.");
                return false
            }
        }

        println!();
        println!("Les dates disponibles pour la réservation de {} postes sont :", post_resa);
        for date in tab_dispo_final {
            println!("Du {}/{}/{} au {}/{}/{}", date.0.day(), date.0.month(), date.0.year(), date.1.day(), date.1.month(), date.1.year());
        }
    }
    return true
}

pub fn total_days_between_dates(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> i64 {
    
    let duration = end_date.signed_duration_since(start_date);
    let days = duration.num_days();
    return days
}